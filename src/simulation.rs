use std::f32::consts::PI;
use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::Distribution};

use crate::{psychics::{Position, Soul, Trace, PsychicSettings}, nn::Net, axiom::Axiom, map::{Map, Species, build_map, xy_idx}};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSettings{max_turn_number: MAX_TURN_NUMBER, current_turn: MAX_TURN_NUMBER, current_generation: 0});
        app.add_systems(Update, simulate_generation);
        app.add_systems(Update, evolve_generation);
        app.register_type::<SimulationSettings>();
    }
}

pub const PLAY_AREA_WIDTH: u32 = 45;
pub const PLAY_AREA_HEIGHT: u32 = 45;
pub const MAX_TURN_NUMBER: usize = 100;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct SimulationSettings {
    pub max_turn_number: usize,
    pub current_turn: usize,
    pub current_generation: usize
}

fn simulate_generation( // Trying hard to make this concurrent with time_passes. Not sure if it will work. 10th November 2023
    // In order to make effects and spells happen: make a vector of (position, effect). Then, at the start of next turn, make them all happen. 12th November 2023
    mut config: ResMut<SimulationSettings>,
    mut psychics: Query<(&mut Position, &mut Soul, &mut Trace, &mut Species), With<Soul>>,
    mut hylics: Query<(&mut Position, &mut Trace, &mut Species), Without<Soul>>,
    mut map: ResMut<Map>,
){    
    if config.current_turn == config.max_turn_number{
        return;
    }
    assert!(config.current_turn < config.max_turn_number);
    for _turn in 0..1{ // large impact on performance: this number is the simulation speed
        let mut beacon_of_light: (u32, u32) = (0,0);
        for (mut position, _trace, mut species) in hylics.iter_mut(){
            // Each entity can do an action by itself.
            map = exit_tile(map, position.x, position.y);

            let action = Axiom::Move { dx: 0, dy: 0 };
            (position.x, position.y) = process_motion(position.x, position.y, action, &map.tiles);
            *species = process_metamorphosis(action, *species);
            map = process_axioms(map, action, (position.x, position.y));

            map = enter_tile(map, position.x, position.y, *species);

            // This is terrible and should be removed.
            match *species{
                Species::Beacon => beacon_of_light = (position.x, position.y),
                _ => ()
            }
        }
        for (mut position, mut soul, mut _trace, mut species) in psychics.iter_mut(){
            soul.senses_input = locate_quadrant(position.x, position.y, beacon_of_light.0, beacon_of_light.1);
            soul.senses_input.append(&mut find_adjacent_collisions((position.x, position.y), &map.tiles));
            soul.senses_input.append(&mut vec![10./(10.+((position.x as i32 - beacon_of_light.0 as i32).abs() + (position.y as i32 - beacon_of_light.1 as i32).abs()) as f64)]);
            soul.decision_outputs = soul.nn.decide(&soul.senses_input);
            let index_of_biggest = soul.decision_outputs.iter().enumerate().fold((0, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
            let action = soul.action_choices[index_of_biggest.0];
            if !soul.actions_chosen.contains(&action.act_motion()){ soul.actions_chosen.push(action.act_motion())};
            // Each entity can do an action by itself.
            map = exit_tile(map, position.x, position.y);

            (position.x, position.y) = process_motion(position.x, position.y, action, &map.tiles);
            *species = process_metamorphosis(action, *species);
            map = process_axioms(map, action, (position.x, position.y));

            map = enter_tile(map, position.x, position.y, *species);
        }
        //debug_print_axiom_map(&map);
        for (mut position, mut trace, mut species) in hylics.iter_mut(){
            map = exit_tile(map, position.x, position.y);
            // Then, the Axiom effects happen.
            let action = grab_axiom_at_pos(&map.axiom_map, (position.x, position.y)); // This makes it impossible to stack multiple axioms in one location, it might need to be changed to a vector.       
            map = void_axiom_at(map, (position.x, position.y));
            (position.x, position.y) = process_motion(position.x, position.y, action, &map.tiles);
            *species = process_metamorphosis(action, *species);
            map = process_axioms(map, action, (position.x, position.y));

            map = enter_tile(map, position.x, position.y, *species);
            trace.positions.push((position.x, position.y));
            trace.identity.push(*species);
        }
        for (mut position, mut _soul, mut trace, mut species) in psychics.iter_mut(){
            map = exit_tile(map, position.x, position.y);

            let action = grab_axiom_at_pos(&map.axiom_map, (position.x, position.y));
            map = void_axiom_at(map, (position.x, position.y));
            (position.x, position.y) = process_motion(position.x, position.y, action, &map.tiles);
            *species = process_metamorphosis(action, *species);
            map = process_axioms(map, action, (position.x, position.y));

            map = enter_tile(map, position.x, position.y, *species);
            trace.positions.push((position.x, position.y));
            trace.identity.push(*species);
        }
        config.current_turn += 1;
    }
}

pub fn process_axioms(
    mut map: ResMut<Map>,
    action: Axiom,
    cur_pos: (u32, u32),
)-> ResMut<Map>{
    let effects = action.act_axioms(cur_pos, &map);
    if effects.is_empty() {return map;}
    for i in effects{
        let idx = map.xy_idx(i.1.0, i.1.1);
        map.axiom_map[idx] = i.0;
    }
    map
}

pub fn debug_print_axiom_map(
    map: &ResMut<Map>,
){
    let mut string = String::from("");
    for i in &map.axiom_map{
        let char: char = match i{
            Axiom::Void => '.',
            _ => '#'
        };
        string.push(char);
        if string.len() == 45{
            dbg!(string);
            string = String::from("");
        }
    }
}

pub fn void_axiom_at(
    mut map: ResMut<Map>,
    pos: (u32, u32)
) -> ResMut<Map>{
    let idx = map.xy_idx(pos.0, pos.1);
    map.axiom_map[idx] = Axiom::Void;
    map
}

pub fn process_x(new_pos: i32) -> i32 {
    match new_pos >= PLAY_AREA_WIDTH as i32{
        true => PLAY_AREA_WIDTH as i32-1,
        false => match new_pos < 0 {
            true => 0,
            false => new_pos
        }
    }
}

pub fn process_y(new_pos: i32) -> i32 {
    match new_pos >= PLAY_AREA_HEIGHT as i32{
        true => PLAY_AREA_HEIGHT as i32-1,
        false => match new_pos < 0 {
            true => 0,
            false => new_pos
        }
    }
}

pub fn enter_tile(mut map: ResMut<Map>, x: u32, y: u32, species: Species) -> ResMut<Map>{
    let idx = map.xy_idx(x, y);
    map.tiles[idx] = species;
    map
}

pub fn exit_tile(mut map: ResMut<Map>, x: u32, y: u32) -> ResMut<Map>{
    let idx = map.xy_idx(x, y);
    map.tiles[idx] = Species::Nothing;
    map
}

pub fn get_adjacent_coords(
    pos: (u32, u32),
) -> Vec<(u32, u32)>{
    let mut search = Vec::with_capacity(4);
    let mut output = Vec::with_capacity(4);
    for i in [(0,1), (1,0), (-1, 0), (0,-1)]{
        search.push(i);
    }
    for i in search{
        let new_coords = (process_x(pos.0 as i32+i.0) as u32, process_y(pos.1 as i32+i.1) as u32);
        output.push(new_coords);
    }
    output
}


pub fn find_adjacent_collisions(
    pos: (u32, u32),
    collision_map: &[Species]
) -> Vec<f64>{
    let mut output = Vec::with_capacity(4);
    for i in get_adjacent_coords(pos){
        if target_is_empty(i, collision_map){
            output.push(1.);
        } else {output.push(0.)};
    }
    output

}

pub fn grab_axiom_at_pos(
    axiom_map: &[Axiom],
    pos: (u32, u32),
) -> Axiom {
    let idx = xy_idx(pos.0, pos.1);
    axiom_map[idx]
}

pub fn target_is_empty(
    new_pos: (u32, u32),
    collision_map: &[Species],
) -> bool {
    let idx = xy_idx(new_pos.0, new_pos.1);
    collision_map[idx] == Species::Nothing
}

fn process_metamorphosis(
    action: Axiom,
    species: Species
) -> Species {action.act_transform(species)}

fn process_motion(
    cur_x: u32,
    cur_y: u32,
    action: Axiom,
    collision_map: &[Species],
) -> (u32, u32){
    let (dx, dy) = action.act_motion();
    let new_coords = (process_x(cur_x as i32 + dx) as u32, process_y(cur_y as i32 + dy) as u32);
    if target_is_empty(new_coords, collision_map) { //
        new_coords
    } else { (cur_x, cur_y) }   
}

fn locate_quadrant( // Move this to a Senses file later
    ori_x: u32,
    ori_y: u32,
    dest_x: u32,
    dest_y: u32,
) -> Vec<f64> {
    let dx = dest_x as i32-ori_x as i32;
    let dy = dest_y as i32-ori_y as i32;
    let mut theta: f32;
    match dx == 0{
        true => match dy > 0 {
            true => theta = 90.,
            false => theta = 270.,
        }
        false => theta = ((dy) as f32).atan2(dx as f32) * (180./PI),
    }
    match theta < 0.{
        true => theta += 360.,
        false=> ()
    }
    let result = theta as u32;
    let angles = [270, 90, 180, 0];
    let mut output = [0., 0., 0., 0.];
    for (i, a) in angles.iter().enumerate(){
        let mut sense = - (0.55 * (result/100) as f32 - (a/180) as f32).abs() + 1.;
        if sense < 0. {sense = 0.}
        output[i] = sense as f64;
    }
    output.to_vec()
}

fn evolve_generation(
    mut config: ResMut<SimulationSettings>,
    mut psychics: Query<(&mut Position, &mut Soul, &mut Trace, &mut Species), With<Soul>>, // Consider making this the same query with Has<Soul>
    psy_settings: Res<PsychicSettings>,
    mut hylics: Query<(&mut Position, &mut Trace, &mut Species), Without<Soul>>,
    mut map: ResMut<Map>,

){
    if config.current_turn < config.max_turn_number{
        return;
    }
    (map.tiles, map.catalogue, map.locations, map.axiom_map) = build_map(map.population.clone());
    let mut beacon_of_light: (u32, u32) = (0,0); // Very gory when more Hylics will get added.
    for (mut pos, mut trace, mut species) in hylics.iter_mut(){
        trace.shipped_positions = trace.positions.clone();
        trace.shipped_identity = trace.identity.clone();
        trace.positions = Vec::with_capacity(config.max_turn_number);
        let creature = trace.original_species;
        let index = map.catalogue.iter().position(|r| r == &creature).unwrap();
        if map.locations[index].is_empty(){
            trace.positions.push((0, 0));
            // Super gory. Since we're always stuck with too many walls, some of them can't find a position and get tucked in a stack in the corner instead. Fix this.
            break;
        }
        let Some((x,y)) = map.locations[index].pop() else { 
            dbg!(&map.locations[index]);
            dbg!(index);
            panic!("Locations assigment did not find an XY pair.") };
        (pos.x, pos.y) = (x, y);
        pos.starting_position = (x,y);
        *species = creature;
        trace.positions.push((x, y));
        trace.identity.push(*species);
        match *species{
            Species::Beacon => beacon_of_light = (pos.x, pos.y),
            _ => ()
        }
    }
    let mut all_souls: Vec<Net> = Vec::with_capacity(psy_settings.number_at_start as usize); 
    let mut all_fitnesses: Vec<f32> = Vec::with_capacity(psy_settings.number_at_start as usize);
    let mut best_fit = (0., 0);
    for (mut pos, mut soul, mut trace, mut species) in psychics.iter_mut(){
        let mut final_fitness = if (pos.x as i32 - beacon_of_light.0 as i32).abs() < 2 && (pos.y as i32 - beacon_of_light.1 as i32).abs() < 2{
            100.
        } else if (pos.x as i32 - beacon_of_light.0 as i32).abs() < 5 && (pos.y as i32 - beacon_of_light.1 as i32).abs() < 5{
            50.
        } else if (pos.x as i32 - beacon_of_light.0 as i32).abs() < 10 && (pos.y as i32 - beacon_of_light.1 as i32).abs() < 10{
            10.
        } else {
            1.
        };
        //dbg!(soul.actions_chosen.clone());
        if soul.actions_chosen.len() > 2{

            final_fitness += 1000.;
        }

        soul.actions_chosen = Vec::new();
        //30.-((pos.x as i32 - beacon_of_light.0 as i32).abs() + (pos.y as i32 - beacon_of_light.1 as i32).abs()) as f32;
        if pos.x == 44 || pos.y == 44 || pos.x == 0 || pos.y == 0{
            final_fitness *= 0.5;
        }
        if (pos.x, pos.y) == pos.starting_position{
            final_fitness = 0.3;
        }
        soul.fitness = final_fitness;
        let creature = trace.original_species;
        let index = map.catalogue.iter().position(|r| r == &creature).unwrap();
        let Some((x,y)) = map.locations[index].pop() else { 
            dbg!(&map.locations[index]);
            dbg!(index);
            panic!("Locations assigment did not find an XY pair.") };
        (pos.x, pos.y) = (x, y);
        *species = creature;
        pos.starting_position = (x,y);
        trace.shipped_positions = trace.positions.clone();
        trace.shipped_identity = trace.identity.clone();
        trace.positions = Vec::with_capacity(config.max_turn_number);
        trace.positions.push((x, y));
        trace.identity.push(*species);
        match *species{
            Species::Beacon => beacon_of_light = (pos.x, pos.y),
            _ => ()
        }

        all_souls.push(soul.nn.clone());
        all_fitnesses.push(final_fitness);
        if final_fitness > best_fit.0{
            best_fit = (final_fitness, all_fitnesses.len()-1);
        }
    }
    //dbg!(all_fitnesses.clone());
    let (_max_fitness, gene_pool) = create_gene_pool(all_fitnesses);
    let mut rng = rand::thread_rng();
    for (mut _position, mut soul, mut _trace, _species) in psychics.iter_mut(){
        let soul_idx = gene_pool.sample(&mut rng);
        let mut rand_soul = all_souls[soul_idx].clone(); // soul_idx
        rand_soul.mutate();
        soul.nn = rand_soul;
    }
    config.current_turn = 0 ;
    config.current_generation += 1;
}

fn create_gene_pool(values: Vec<f32>) -> (f32, WeightedIndex<f32>) {
    let mut max_fitness = 0.0;
    let mut weights = Vec::new();

    for v in values.iter() {
        if *v > max_fitness {
            max_fitness = *v;
        }
        weights.push(*v);
    }

    (
        max_fitness,
        WeightedIndex::new(&weights).expect("Failed to generate gene pool"),
    )
}