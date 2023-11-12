use std::{time::Duration, f32::consts::PI};
use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, lens::TransformPositionLens, Tween};
use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};

use crate::{psychics::{Position, Soul, ActionType, FinishedTrace, Trace, PsychicSettings, Marker}, nn::Net};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TheatreSettings{time_between_turns: Timer::new(Duration::from_millis(200), TimerMode::Repeating), current_turn: 0, max_turn_number: 100});
        app.insert_resource(SimulationSettings{max_turn_number: 100, current_turn: 100, current_generation: 0});
        app.add_systems(Update, time_passes);
        app.add_systems(Update, simulate_generation);
        app.add_systems(Update, ship_gen_to_theatre);
        app.add_systems(Update, evolve_generation);
        app.register_type::<SimulationSettings>();
    }
}

pub const PLAY_AREA_WIDTH: u32 = 45;
pub const PLAY_AREA_HEIGHT: u32 = 45;
pub const TILE_SIZE: f32 = 16.;

#[derive(Resource)]
pub struct TheatreSettings {
    pub time_between_turns: Timer,
    pub current_turn: usize,
    pub max_turn_number: usize,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct SimulationSettings {
    pub max_turn_number: usize,
    pub current_turn: usize,
    pub current_generation: usize
}

fn process_motion(
    cur_x: u32,
    cur_y: u32,
    action: ActionType
) -> (u32, u32){
    let mut dx = 0;
    let mut dy = 0;
    match action {
        ActionType::North => dy = 1,
        ActionType::South => dy = -1,
        ActionType::West => dx = 1,
        ActionType::East => dx = -1,
        _ => dx = 0,
    }
    (process_x(cur_x as i32 + dx) as u32, process_y(cur_y as i32 + dy) as u32)
}

fn ship_gen_to_theatre(
    ship: Query<&Trace>,
    mut theatre: Query<&mut FinishedTrace>,
    keys: Res<Input<KeyCode>>,
    psy_sets: Res<PsychicSettings>,
    mut config: ResMut<TheatreSettings>,
){
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let mut all_positions = Vec::with_capacity(psy_sets.number_at_start as usize);
    let mut index_num = 0;
    for tracer in ship.iter(){
        all_positions.push(tracer.shipped_positions.clone());
        index_num += 1;
    }
    index_num = 0;
    for mut displayed in theatre.iter_mut(){
        displayed.positions = all_positions[index_num].clone();
        index_num += 1;
    }
    config.current_turn = 0;
}

fn evolve_generation(
    mut config: ResMut<SimulationSettings>,
    mut psychics: Query<(&mut Position, &mut Soul, &mut Trace)>,
    psy_settings: Res<PsychicSettings>,
    mut mark: Query<(&mut Position, &mut Trace), (Without<Soul>, With<Marker>)>,

){
    if config.current_turn < config.max_turn_number{
        return;
    }
    let mut rng = rand::thread_rng();
    let mut beacon_of_light: (u32, u32) = (0,0); // Very gory when more Hylics will get added.
    for (mut pos, mut trace) in mark.iter_mut(){
        beacon_of_light = (pos.x, pos.y);
        (pos.x, pos.y) = (rng.gen_range(0..PLAY_AREA_WIDTH), rng.gen_range(0..PLAY_AREA_HEIGHT));
        trace.shipped_positions = trace.positions.clone();
        trace.positions = Vec::with_capacity(config.max_turn_number);
        trace.positions.push(pos.starting_position);
    }
    let mut all_souls: Vec<Net> = Vec::with_capacity(psy_settings.number_at_start as usize); 
    let mut all_fitnesses: Vec<f32> = Vec::with_capacity(psy_settings.number_at_start as usize);
    for (mut position, mut soul, mut trace) in psychics.iter_mut(){
        let mut final_fitness = 90.-((position.x as i32 - beacon_of_light.0 as i32).abs() + (position.y as i32 - beacon_of_light.1 as i32).abs()) as f32;
        match position.x {
            1..=43 => match position.y {
                1..=43 => final_fitness = final_fitness,
                _ => final_fitness *= 0.1,
            },
            _ => final_fitness *= 0.1
        }
        soul.fitness = final_fitness;
        trace.shipped_positions = trace.positions.clone();
        trace.positions = Vec::with_capacity(config.max_turn_number);
        (position.x, position.y) = position.starting_position;
        trace.positions.push(position.starting_position);
        all_souls.push(soul.nn.clone());
        all_fitnesses.push(final_fitness);
    }
    //dbg!(all_fitnesses.clone());
    let (_max_fitness, gene_pool) = create_gene_pool(all_fitnesses);
    let mut rng = rand::thread_rng();
    for (mut _position, mut soul, mut _trace) in psychics.iter_mut(){
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

fn locate_quadrant(
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
        false=> theta = theta
    }
    let result = theta as u32;
    match result {
        45..=134 => [0., 1., 0., 0.].to_vec(),
        135..=224 => [0., 0., 1., 0.].to_vec(),
        225..=315 => [1., 0., 0., 0.].to_vec(),
        _ => [0., 0., 0., 1.].to_vec()
    }
}

fn simulate_generation( // Trying hard to make this concurrent with time_passes. Not sure if it will work. 10th November 2023
    mut config: ResMut<SimulationSettings>,
    mut psychics: Query<(&mut Position, &mut Soul, &mut Trace)>,
    mut hylics: Query<(&mut Position, &mut Trace), Without<Soul>>,
){    
    if config.current_turn == config.max_turn_number{
        return;
    }
    assert!(config.current_turn < config.max_turn_number);
    for turn in 0..config.max_turn_number+1{
        let mut beacon_of_light: (u32, u32) = (0,0); // Very gory when more Hylics will get added.
        for (mut position, mut trace) in hylics.iter_mut(){
            let (checked_new_x, checked_new_y) = process_motion(position.x, position.y, ActionType::Wait);
            (position.x, position.y) = (checked_new_x, checked_new_y);
            trace.positions.push((position.x, position.y));
            beacon_of_light = (position.x, position.y);
        }
        for (mut position, mut soul, mut trace) in psychics.iter_mut(){
            config.current_turn = turn;
            soul.senses_input = locate_quadrant(position.x, position.y, beacon_of_light.0, beacon_of_light.1);
            soul.decision_outputs = soul.nn.decide(&soul.senses_input);
            let index_of_biggest = soul.decision_outputs.iter().enumerate().fold((0, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
            let final_decision = soul.action_choices[index_of_biggest.0];
            let (checked_new_x, checked_new_y) = process_motion(position.x, position.y, final_decision);
            (position.x, position.y) = (checked_new_x, checked_new_y);
            trace.positions.push((position.x, position.y));
        }
    }
}

fn time_passes(
    time: Res<Time>,
    mut config: ResMut<TheatreSettings>,
    mut psychics: Query<(&Transform, &mut Animator<Transform>, &FinishedTrace)>, // Later on, Has<Soul> could be good for non-nn creatures?
){
    config.time_between_turns.tick(time.delta());
    if config.time_between_turns.finished() {
        for (transform, mut anim, trace) in psychics.iter_mut(){
            if trace.positions.len() == 0 || config.current_turn >= config.max_turn_number{
                continue;
            }
            let (x, y) = (trace.positions[config.current_turn].0, trace.positions[config.current_turn].1);
            let start = transform.translation;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(199),
                TransformPositionLens {
                    start,
                    end: Vec3::new(TILE_SIZE * x as f32, TILE_SIZE * y as f32, 0.),
                },
            );
            anim.set_tweenable(tween);
        }
        config.current_turn += 1;
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