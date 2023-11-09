mod psychics;

use bevy::prelude::*;
use psychics::{PsychicPlugin, PsychicSettings};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TGFP".into(),
                        resolution: (1024.0, 576.0).into(),
                        //resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(PreStartup, load_spritesheet)
        .add_systems(Startup, load_camera)
        .insert_resource(PsychicSettings{number_at_start: 16})
        .add_plugins(PsychicPlugin)
        .add_systems(Update, character_movement)
        .add_systems(Update, zoom_2d)
        .run();
}

fn load_spritesheet(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
){
    let img_path = "spritesheet.png".to_owned();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        80, 2, None, None
    );
    let handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteSheetHandle{handle: handle});
}

fn load_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}


#[derive(Resource)]
pub struct SpriteSheetHandle {
    handle: Handle<TextureAtlas>
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Camera2d)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y -= 1000.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y += 1000.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x -= 1000.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x += 1000.0 * time.delta_seconds();
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