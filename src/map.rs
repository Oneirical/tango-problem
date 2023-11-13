use rand::Rng;
use bevy::prelude::*;

use crate::timeline::{PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, build);
        app.insert_resource(Map::new());
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CreatureType {
    Wall,
    Nothing,
    Psychic,
    Beacon,
}

fn build(
    mut map: ResMut<Map>,
) {
    let mut rng = rand::thread_rng();

    // First we completely randomize the map, setting 55% of it to be floor.
    for y in 0..PLAY_AREA_HEIGHT {
        for x in 0..PLAY_AREA_WIDTH {
            let roll = rng.gen_range(0..100);
            let idx = map.xy_idx(x, y);
            let edge = x == 0 || y == 0 || y == PLAY_AREA_HEIGHT-1 || x == PLAY_AREA_WIDTH-1;
            if roll > 55 || edge { map.tiles[idx] = CreatureType::Wall }
        }
    }
    for _i in 0..15 {
        let mut newtiles = map.tiles.clone();

        for y in 1..PLAY_AREA_HEIGHT-1 {
            for x in 1..PLAY_AREA_WIDTH-1 {
                let idx = map.xy_idx(x, y);
                let mut neighbors = 0;
                if map.tiles[idx - 1] == CreatureType::Wall { neighbors += 1; }
                if map.tiles[idx + 1] == CreatureType::Wall { neighbors += 1; }
                if map.tiles[idx - PLAY_AREA_WIDTH as usize] == CreatureType::Wall { neighbors += 1; }
                if map.tiles[idx + PLAY_AREA_WIDTH as usize] == CreatureType::Wall { neighbors += 1; }
                if map.tiles[idx - (PLAY_AREA_WIDTH as usize - 1)] == CreatureType::Wall { neighbors += 1; }
                if map.tiles[idx - (PLAY_AREA_WIDTH as usize + 1)] == CreatureType::Wall { neighbors += 1; }
                if map.tiles[idx + (PLAY_AREA_WIDTH as usize - 1)] == CreatureType::Wall { neighbors += 1; }
                if map.tiles[idx + (PLAY_AREA_WIDTH as usize + 1)] == CreatureType::Wall { neighbors += 1; }

                if neighbors > 4 || neighbors == 0 {
                    newtiles[idx] = CreatureType::Wall;
                }
                else {
                    newtiles[idx] = CreatureType::Nothing;
                }
            }
        }

        map.tiles = newtiles.clone();
    }
}

#[derive(Resource)]
pub(crate) struct Map {
    pub tiles: Vec<CreatureType>,
}

impl Map{
    fn new() -> Self{
        let mut new_map = Self { tiles: Vec::with_capacity((PLAY_AREA_HEIGHT*PLAY_AREA_WIDTH) as usize) };
        for _i in 0..PLAY_AREA_HEIGHT*PLAY_AREA_WIDTH{
            new_map.tiles.push(CreatureType::Nothing);
        }
        new_map
    }
    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y as usize * PLAY_AREA_WIDTH as usize) + x as usize
    }
}