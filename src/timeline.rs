use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, lens::TransformPositionLens, Tween};

use crate::psychics::{Position, Soul, ActionType, InTheatre, FinishedTrace, Trace};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TheatreSettings{time_between_turns: Timer::new(Duration::from_millis(200), TimerMode::Repeating), current_turn: 0, max_turn_number: 100});
        app.insert_resource(SimulationSettings{max_turn_number: 100, current_turn: 0});
        app.add_systems(Update, time_passes);
        app.add_systems(Update, simulate_generation);
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

#[derive(Resource)]
pub struct SimulationSettings {
    pub max_turn_number: usize,
    pub current_turn: usize
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

fn simulate_generation(
    mut config: ResMut<SimulationSettings>,
    mut psychics: Query<(&mut Position, &mut Soul, &mut Trace), Without<InTheatre>>
){
    if config.current_turn >= config.max_turn_number{
        return;
    }
    for (mut position, mut soul, mut trace) in psychics.iter_mut(){
        if config.current_turn == 0 {
            trace.positions = Vec::with_capacity(config.max_turn_number);
        }
        soul.decision_outputs = soul.nn.decide(&soul.senses_input);
        let index_of_biggest = soul.decision_outputs.iter().enumerate().fold((0, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
        let final_decision = soul.action_choices[index_of_biggest.0];
        let (checked_new_x, checked_new_y) = process_motion(position.x, position.y, final_decision);
        (position.x, position.y) = (checked_new_x, checked_new_y);
        trace.positions[config.current_turn] = (position.x, position.y);
    }
    config.current_turn += 1;
}

fn time_passes(
    time: Res<Time>,
    mut config: ResMut<TheatreSettings>,
    mut psychics: Query<(&Transform, &mut Animator<Transform>, &FinishedTrace)>, // Later on, Has<Soul> could be good for non-nn creatures?
){
    config.time_between_turns.tick(time.delta());
    if config.time_between_turns.finished() {
        for (transform, mut anim, trace) in psychics.iter_mut(){
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