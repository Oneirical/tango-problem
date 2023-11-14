use rand::{Rng, seq::IteratorRandom, thread_rng};
use bevy::prelude::*;

use crate::simulation::{PLAY_AREA_WIDTH, PLAY_AREA_HEIGHT};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new());
    }
}

#[derive(Component, Clone, PartialEq, Debug)]
pub enum Species {
    Wall,
    Nothing,
    Psychic,
    Beacon,
}

pub fn build_map(
    parameters: Vec<Species>,
) -> Vec<Species> {
    let mut map = Map::new();
    let mut rng = rand::thread_rng();

    // First we completely randomize the map, setting 55% of it to be floor.
    for y in 0..PLAY_AREA_HEIGHT {
        for x in 0..PLAY_AREA_WIDTH {
            let roll = rng.gen_range(0..100);
            let idx = map.xy_idx(x, y);
            let edge = x == 0 || y == 0 || y == PLAY_AREA_HEIGHT-1 || x == PLAY_AREA_WIDTH-1;
            if roll > 85 || edge { map.tiles[idx] = Species::Wall }
        }
    }
    for _i in 0..15 {
        let mut newtiles = map.tiles.clone();

        for y in 1..PLAY_AREA_HEIGHT-1 {
            for x in 1..PLAY_AREA_WIDTH-1 {
                let idx = map.xy_idx(x, y);
                let mut neighbors = 0;
                if map.tiles[idx - 1] == Species::Wall { neighbors += 1; }
                if map.tiles[idx + 1] == Species::Wall { neighbors += 1; }
                if map.tiles[idx - PLAY_AREA_WIDTH as usize] == Species::Wall { neighbors += 1; }
                if map.tiles[idx + PLAY_AREA_WIDTH as usize] == Species::Wall { neighbors += 1; }
                if map.tiles[idx - (PLAY_AREA_WIDTH as usize - 1)] == Species::Wall { neighbors += 1; }
                if map.tiles[idx - (PLAY_AREA_WIDTH as usize + 1)] == Species::Wall { neighbors += 1; }
                if map.tiles[idx + (PLAY_AREA_WIDTH as usize - 1)] == Species::Wall { neighbors += 1; }
                if map.tiles[idx + (PLAY_AREA_WIDTH as usize + 1)] == Species::Wall { neighbors += 1; }

                if neighbors > 4 || neighbors == 0 {
                    newtiles[idx] = Species::Wall;
                }
                else {
                    newtiles[idx] = Species::Nothing;
                }
            }
        }
        map.tiles = newtiles.clone();
    }
    let mut eligible_spawns = Vec::new();
    for y in 0..PLAY_AREA_HEIGHT {
        for x in 0..PLAY_AREA_WIDTH {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == Species::Nothing{
                eligible_spawns.push((x,y));
            }
        }
    }
    let queue_of_species = parameters.clone();
    for s in queue_of_species{
        let empty_spaces = eligible_spawns.clone();
        let (i, t) = empty_spaces.iter().enumerate().choose(&mut thread_rng()).unwrap();
        eligible_spawns.remove(i);
        let idx = map.xy_idx(t.0, t.1);
        map.tiles[idx] = s;
    }
    map.tiles
}

#[derive(Resource)]
pub(crate) struct Map {
    pub tiles: Vec<Species>,
    pub population: Vec<Species>,
}

impl Map{
    fn new() -> Self{
        let mut recipe = vec![Species::Beacon];
        for _i in 0..15{
            recipe.push(Species::Psychic);
        }
        let mut new_map = Self { tiles: Vec::with_capacity((PLAY_AREA_HEIGHT*PLAY_AREA_WIDTH) as usize), population: recipe };
        for _i in 0..PLAY_AREA_HEIGHT*PLAY_AREA_WIDTH{
            new_map.tiles.push(Species::Nothing);
        }
        new_map
    }
    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y as usize * PLAY_AREA_WIDTH as usize) + x as usize
    }
}