use bevy::prelude::*;

use crate::timeline::{PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, draw_black_square);
        app.add_systems(Update, character_movement);
        app.add_systems(Update, zoom_2d);
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

fn character_movement(
    mut characters: Query<(&mut Transform, &Camera2d)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 1000.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 1000.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 1000.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 1000.0 * time.delta_seconds();
        }
    }
}

fn zoom_2d(
    mut q: Query<&mut OrthographicProjection, With<Camera2d>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if input.pressed(KeyCode::O) {
        let mut projection = q.single_mut();

        // example: zoom in
        projection.scale += 0.8 * time.delta_seconds();
        // example: zoom out
        //projection.scale *= 0.75;
    
        // always ensure you end up with sane values
        // (pick an upper and lower bound for your application)
        projection.scale = projection.scale.clamp(0.5, 5.0);
    }
    else if input.pressed(KeyCode::P) {
        let mut projection = q.single_mut();

        // example: zoom in
        projection.scale -= 0.8 * time.delta_seconds();
        projection.scale = projection.scale.clamp(0.5, 5.0);
    }
}