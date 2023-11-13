mod psychics;
mod timeline;
mod ui;
mod nn;
mod util;
mod map;

use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use map::MapPlugin;
use psychics::PsychicPlugin;
use ui::UIPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use timeline::TimePlugin;
use bevy::input::common_conditions::input_toggle_active;

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
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_systems(PreStartup, load_spritesheet)
        .add_systems(Startup, load_camera)
        .add_plugins(TweeningPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(PsychicPlugin)
        .add_plugins(TimePlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Update, character_movement)
        .add_systems(Update, zoom_2d)
        .run();
}

fn load_spritesheet( // I am so glad this works. Just looking at this code is going to make me fail NNN. - 8th November 2023
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
){
    let img_path = "spritesheet.png".to_owned();
    let texture_handle = asset_server.load(img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        80, 2, None, None
    );
    let handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteSheetHandle{handle});
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