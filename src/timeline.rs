use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, lens::TransformPositionLens, Tween};

use crate::psychics::{Psychic, Position, Soul, ActionType};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnsSetting{time_between_turns: Timer::new(Duration::from_millis(500), TimerMode::Repeating)});
        app.add_systems(Update, time_passes);
    }
}

const PLAY_AREA_WIDTH: u32 = 45;
const PLAY_AREA_HEIGHT: u32 = 45;
const TILE_SIZE: f32 = 16.;

#[derive(Resource)]
pub struct TurnsSetting {
    time_between_turns: Timer,
}

fn time_passes(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<TurnsSetting>,
    mut psychics: Query<(Entity, &mut Transform, &mut Position, &mut Soul), With<Psychic>>, // Later on, Has<Soul> could be good for non-nn creatures?
){
    config.time_between_turns.tick(time.delta());
    if config.time_between_turns.finished() {
        for (entity, mut transform, mut position, mut soul) in psychics.iter_mut(){
            soul.decision_outputs = soul.nn.decide(&soul.senses_input);
            let index_of_biggest = soul.decision_outputs.iter().enumerate().fold((0, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
            let final_decision = soul.action_choices[index_of_biggest.0];
            let mut dx = 0;
            let mut dy = 0;
            match final_decision {
                ActionType::North => dy = 1,
                ActionType::South => dy = -1,
                ActionType::West => dx = 1,
                ActionType::East => dx = -1,
                ActionType::Wait => dx = 0,
            }
            let checked_new_x = process_x(position.x as i32 + dx) as u32;
            let checked_new_y = process_y(position.y as i32 + dy) as u32;
            position.x = checked_new_x;
            position.y = checked_new_y;
            let start = transform.translation;
            /*
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(100),
                TransformPositionLens {
                    start,
                    end: Vec3::new(start.x + TILE_SIZE * position.x as f32, start.y + TILE_SIZE * position.y as f32, 0.),
                },
            );
            dbg!(position.y);
            */
            transform.translation.x = start.x + TILE_SIZE * position.x as f32;
            transform.translation.y = start.y + TILE_SIZE * position.y as f32;
            //commands.entity(entity).insert(Animator::new(tween));
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