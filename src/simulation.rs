use std::f32::consts::PI;
use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::Distribution};

use crate::{psychics::{Position, Soul, Trace, PsychicSettings}, nn::Net, axiom::Axiom, map::{Map, Species, build_map}};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSettings{max_turn_number: 100, current_turn: 100, current_generation: 0});
        app.add_systems(Update, simulate_generation);
        app.add_systems(Update, evolve_generation);
        app.register_type::<SimulationSettings>();
    }
}

pub const PLAY_AREA_WIDTH: u32 = 45;
pub const PLAY_AREA_HEIGHT: u32 = 45;

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
    mut psychics: Query<(&mut Position, &mut Soul, &mut Trace), With<Soul>>,
    mut hylics: Query<(&mut Position, &mut Trace, &Species), Without<Soul>>,
    map: Res<Map>,
){    
    if config.current_turn == config.max_turn_number{
        return;
    }
    assert!(config.current_turn < config.max_turn_number);
    for turn in 0..config.max_turn_number+1{
        let mut beacon_of_light: (u32, u32) = (0,0);
        for (mut position, mut trace, species) in hylics.iter_mut(){
            let action = Axiom::Move { dx: 0, dy: 0 };
            (position.x, position.y) = process_motion(position.x, position.y, action, map.tiles);
            trace.positions.push((position.x, position.y));
            match species{
                Species::Beacon => beacon_of_light = (position.x, position.y),
                _ => ()
            }
        }
        for (mut position, mut soul, mut trace) in psychics.iter_mut(){
            config.current_turn = turn;
            soul.senses_input = locate_quadrant(position.x, position.y, beacon_of_light.0, beacon_of_light.1);
            soul.decision_outputs = soul.nn.decide(&soul.senses_input);
            let index_of_biggest = soul.decision_outputs.iter().enumerate().fold((0, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
            let final_decision = soul.action_choices[index_of_biggest.0];
            (position.x, position.y) = process_motion(position.x, position.y, final_decision, map.tiles);
            trace.positions.push((position.x, position.y));
        }
    }
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

pub fn target_is_empty(
    new_pos: (u32, u32),
    collision_map: Vec<Species>,
) -> bool {
    let idx = xy_idx(pos.0, pos.1);
    if collision_map[idx] == Species::Nothing{
        true
    }
    else { false }
}

fn process_motion(
    cur_x: u32,
    cur_y: u32,
    action: Axiom,
    collision_map: Vec<Species>,
) -> (u32, u32){
    let (dx, dy) = action.act_motion();
    let new_coords = (process_x(cur_x as i32 + dx) as u32, process_y(cur_y as i32 + dy) as u32);
    if target_is_empty(new_coords, collision_map){
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
    mut psychics: Query<(&mut Position, &mut Soul, &mut Trace, &Species), With<Soul>>, // Consider making this the same query with Has<Soul>
    psy_settings: Res<PsychicSettings>,
    mut hylics: Query<(&mut Position, &mut Trace, &Species), Without<Soul>>,
    mut map: ResMut<Map>,

){
    if config.current_turn < config.max_turn_number{
        return;
    }
    map.tiles = build_map(map.population.clone());
    let mut placed_coords = Vec::new();
    let mut beacon_of_light: (u32, u32) = (0,0); // Very gory when more Hylics will get added.
    for (mut pos, mut trace, species) in hylics.iter_mut(){
        for y in 0..PLAY_AREA_HEIGHT {
            for x in 0..PLAY_AREA_WIDTH {
                let idx = map.xy_idx(x, y);
                let tile = &map.tiles[idx];
                if species == tile && !placed_coords.contains(&(x,y)) {
                    (pos.x, pos.y) = (x, y);
                    trace.shipped_positions = trace.positions.clone();
                    trace.positions = Vec::with_capacity(config.max_turn_number);
                    trace.positions.push((x, y));
                    placed_coords.push((x, y));
                    match species{
                        Species::Beacon => beacon_of_light = (pos.x, pos.y),
                        _ => ()
                    }
                    break;
                }
            }
        }
    }
    let mut all_souls: Vec<Net> = Vec::with_capacity(psy_settings.number_at_start as usize); 
    let mut all_fitnesses: Vec<f32> = Vec::with_capacity(psy_settings.number_at_start as usize);
    for (mut pos, mut soul, mut trace, species) in psychics.iter_mut(){
        let final_fitness = 1./((pos.x as i32 - beacon_of_light.0 as i32).abs() + (pos.y as i32 - beacon_of_light.1 as i32).abs() + 1) as f32;
        soul.fitness = final_fitness;

        for y in 0..PLAY_AREA_HEIGHT {
            for x in 0..PLAY_AREA_WIDTH {
                let idx = map.xy_idx(x, y);
                let tile = &map.tiles[idx];
                if species == tile && !placed_coords.contains(&(x,y)) {
                    (pos.x, pos.y) = (x, y);
                    trace.shipped_positions = trace.positions.clone();
                    trace.positions = Vec::with_capacity(config.max_turn_number);
                    trace.positions.push((x, y));
                    placed_coords.push((x, y));
                    break;
                }
            }
        }
        all_souls.push(soul.nn.clone());
        all_fitnesses.push(final_fitness);
    }
    //dbg!(all_fitnesses.clone());
    let (_max_fitness, gene_pool) = create_gene_pool(all_fitnesses);
    let mut rng = rand::thread_rng();
    for (mut _position, mut soul, mut _trace, _species) in psychics.iter_mut(){
        let soul_idx = gene_pool.sample(&mut rng);
        let mut rand_soul = all_souls[soul_idx].clone();
        rand_soul.mutate();
        soul.nn = rand_soul;
    }
    config.current_turn = 0;
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