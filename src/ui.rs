use bevy::prelude::*;

use crate::timeline::{PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, draw_black_square);
    }
}

fn draw_black_square(
    mut commands: Commands

){
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(PLAY_AREA_WIDTH as f32 * 16. + 16., PLAY_AREA_HEIGHT as f32 * 16. + 16.)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(PLAY_AREA_WIDTH as f32 / 2. * 16., PLAY_AREA_HEIGHT as f32 / 2. * 16., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0., 0.),
            custom_size: Some(Vec2::new(PLAY_AREA_WIDTH as f32 * 16., PLAY_AREA_HEIGHT as f32 * 16.)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new((PLAY_AREA_WIDTH) as f32 / 2. * 16., (PLAY_AREA_HEIGHT) as f32 / 2. * 16., 0.)),
        ..default()
    });
}