use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    let map = Map::new_map();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, draw_creature)
        .run();
}

fn hello_world() {

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
}

pub fn draw_creature(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("22677410.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
    fn new_map() -> Map {
        let empty_floor = Floor{
            creature_id : -1, 
        };
        let map = Map{
            tiles: vec![empty_floor; 45*45],
            width : 45,
            height : 45,
        };
        map
    }
}