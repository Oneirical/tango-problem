use bevy::prelude::*;

fn main() {
    let map = Map::new_map();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello_world)
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