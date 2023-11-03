use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TGFP".into(),
                        resolution: (1920.0, 1080.0).into(),
                        //resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, create_map)
        .add_systems(Update, character_movement)
        .add_systems(Update, zoom_2d)
        .run();
}

pub struct Map {
    pub tiles : Vec<Floor>,
    pub width : i32,
    pub height : i32,
}

#[derive(Component, Debug)]
pub struct Creature {
    pub species : String,
}

#[derive(Component, Debug, PartialEq, Copy, Clone)]
pub struct Floor {
    pub creature_id : i32,
    pub x: i32,
    pub y: i32,
}

pub fn create_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let play_space = Map::new_map();
    let img_path = "spritesheet.png".to_owned();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        80, 2, None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 2,
            custom_size: Some(Vec2::new(64.0*45.0, 64.0*45.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3{ x: 64.0 * 22.0, y: 64.0*22.0, z: 0.0},
            ..default()
        },
        ..default()
    });
    for i in play_space.tiles{
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 1,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3{ x: i.x as f32 * 64.0, y: i.y as f32 * 64.0, z: 0.0},
                ..default()
            },
            ..default()
        });
    }
}

fn character_movement(
    mut characters: Query<(&mut Transform, &TextureAtlasSprite)>,
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

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
    fn new_map() -> Map {
        let empty_floor = Floor{
            creature_id : -1,
            x: 0,
            y: 0,
        };
        let mut map = Map{
            tiles: vec![empty_floor; 45*45],
            width : 45,
            height : 45,
        };
        let mut x_count = 0;
        let mut y_count = 0;
        for i in &mut map.tiles{
            if x_count > map.width-1{
                x_count = 0;
                y_count += 1;
            }
            i.x = x_count;
            i.y = y_count;
            assert!((0..45).contains(&i.x));
            x_count += 1;
        }
        map
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