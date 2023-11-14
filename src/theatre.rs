use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, lens::TransformPositionLens, Tween};

use crate::psychics::{FinishedTrace, Trace, PsychicSettings};

pub struct TheatrePlugin;

impl Plugin for TheatrePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TheatreSettings{time_between_turns: Timer::new(Duration::from_millis(200), TimerMode::Repeating), current_turn: 0, max_turn_number: 100});
        app.add_systems(Update, time_passes);
        app.add_systems(Update, ship_gen_to_theatre);
    }
}

pub const TILE_SIZE: f32 = 16.;

#[derive(Resource)]
pub struct TheatreSettings {
    pub time_between_turns: Timer,
    pub current_turn: usize,
    pub max_turn_number: usize,
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
    for tracer in ship.iter(){
        all_positions.push(tracer.shipped_positions.clone());
    }
    for (i, mut displayed) in theatre.iter_mut().enumerate(){
        displayed.positions = all_positions[i].clone();
    }
    config.current_turn = 0;
}

fn time_passes(
    time: Res<Time>,
    mut config: ResMut<TheatreSettings>,
    mut psychics: Query<(&Transform, &mut Animator<Transform>, &FinishedTrace)>, // Later on, Has<Soul> could be good for non-nn creatures?
){
    config.time_between_turns.tick(time.delta());
    if config.time_between_turns.finished() {
        for (transform, mut anim, trace) in psychics.iter_mut(){
            if trace.positions.is_empty() || config.current_turn >= config.max_turn_number{
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