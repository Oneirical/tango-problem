use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor::BottomLeft;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Animator, Tween, EaseFunction};
use rand::Rng;

use crate::axiom::{Axiom, AxiomKit};
use crate::map::{Map, Species, build_map};
use crate::SpriteSheetHandle;
use crate::nn::Net;
use crate::simulation::{PLAY_AREA_HEIGHT, PLAY_AREA_WIDTH};
use crate::theatre::TILE_SIZE;

pub struct PsychicPlugin;

impl Plugin for PsychicPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PsychicSettings{number_at_start: 16});
        app.add_systems(Startup, distribute_psychics);
        app.register_type::<Soul>();
        app.register_type::<Position>();
    }
}

#[derive(Bundle)]
pub struct PsychicBundle {
    soul: Soul,
    position: Position,
    trace: Trace,
    name: Name,
    species: Species,
}

#[derive(Bundle)]
pub struct HylicBundle {
    position: Position,
    trace: Trace,
    name: Name,
    species: Species
}

#[derive(Bundle)]
pub struct TheatreBundle {
    sprite_bundle: SpriteSheetBundle,
    animation: Animator<Transform>,
    finished_trace: FinishedTrace,
    name: Name,
    species: Species
}

impl TheatreBundle { // Creatures displayed on screen.
    pub fn new(
        tex_handle: &SpriteSheetHandle
    ) -> Self {
        let texture_atlas_handle = &tex_handle.handle;
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(1000),
            TransformPositionLens {
                start: Vec3::ZERO,
                end: Vec3::ZERO,
            },
        );
        Self{
            sprite_bundle : SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite{
                    index : 0_usize,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    anchor: BottomLeft,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3{ x: 0., y: 0., z: 0.0},
                    
                    ..default()
                },
                ..default()
            },
            animation: Animator::new(tween),
            finished_trace: FinishedTrace{positions: Vec::new()},
            name: Name::new("TheatreDisplay"),
            species: Species::Wall
        }
    }
    pub fn with_sprite(mut self, s: usize) -> Self { // Absolutely immaculate!
        self.sprite_bundle.sprite.index = s;
        self
    }
    pub fn with_position(mut self, x: u32, y: u32) -> Self {
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(1000),
            TransformPositionLens {
                start: Vec3::ZERO,
                end: Vec3::new(TILE_SIZE * x as f32, TILE_SIZE * y as f32, 0.)
            },
        );
        self.animation = Animator::new(tween);
        self
    }
    pub fn with_species(mut self, species: Species) -> Self {
        self.species = species;
        self
    }
}

impl PsychicBundle { // Creatures simulated in the genetic process.
    // This is the start of something great. 8th November 2023
    pub fn new() -> Self {
        Self{
            soul: Soul {                 
                nn: Net::new(vec![
                    4_usize,
                    15,
                    5,
                ]),
                senses_input: Vec::new(),
                decision_outputs: Vec::new(), 
                action_choices: Vec::new(),
                fitness: 0.
            },
            position: Position { x: 0, y: 0, starting_position: (0, 0) },
            trace: Trace {positions: Vec::new(), shipped_positions: Vec::new()},
            name: Name::new("Psychic"),
            species: Species::Wall
        }
    }
    pub fn with_position(mut self, x: u32, y: u32) -> Self { // Absolutely immaculate!
        self.position.x = x;
        self.position.y = y;
        self.position.starting_position = (x, y);
        self
    }
    pub fn with_axiom_kits(mut self, kits: Vec<AxiomKit>) -> Self{
        for kit in kits{
            self.soul.action_choices.append(&mut kit.unpack());
        }
        self.soul.nn = Net::new(vec![
            4_usize, // This will have to be adapted to the number of Senses.
            15,
            self.soul.action_choices.len(),
        ]);
        self
    }
    pub fn with_species(mut self, species: Species) -> Self {
        self.species = species;
        self
    }
}

impl HylicBundle { // Creatures without a neural network, who present challenges for the Psychics.
    pub fn new() -> Self{
        Self{
            position: Position { x: 0, y: 0, starting_position: (0, 0) },
            trace: Trace {positions: Vec::new(), shipped_positions: Vec::new()},
            name: Name::new("Hylic"),
            species: Species::Wall
        }
    }
    pub fn with_position(mut self, x: u32, y: u32) -> Self { // Absolutely immaculate!
        self.position.x = x;
        self.position.y = y;
        self.position.starting_position = (x, y);
        self
    }
    pub fn with_species(mut self, species: Species) -> Self {
        self.species = species;
        self
    }
}

#[derive(Component)]
pub struct Marker{
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Soul{
    pub nn: Net,
    pub decision_outputs: Vec<f64>,
    pub senses_input: Vec<f64>,
    pub action_choices: Vec<Axiom>,
    pub fitness: f32,
}

#[derive(Resource)]
pub struct PsychicSettings {
    pub number_at_start: u32,
}

#[derive(Component, Default, Reflect)]
pub struct Position{
    pub x: u32,
    pub y: u32,
    pub starting_position: (u32, u32)
}

#[derive(Component)]
pub struct Trace{
    pub positions: Vec<(u32, u32)>,
    pub shipped_positions: Vec<(u32, u32)>
}

#[derive(Component)]
pub struct FinishedTrace{
    pub positions: Vec<(u32, u32)>
}

fn distribute_psychics(
    mut commands: Commands,
    tex_handle: Res<SpriteSheetHandle>,
    mut map: ResMut<Map>,
    //settings: &mut Settings,
    //soul: Option<Vec<NeuNet>>,
){
    map.tiles = build_map(map.population.clone());
    for y in 0..PLAY_AREA_HEIGHT {
        for x in 0..PLAY_AREA_WIDTH {
            let idx = map.xy_idx(x, y);
            let tile = &map.tiles[idx];
            match tile {
                Species::Wall => {
                    let wall_t = TheatreBundle::new(&tex_handle).with_sprite(3).with_position(x, y).with_species(Species::Wall);
                    let wall = HylicBundle::new().with_position(x, y).with_species(Species::Wall);
                    commands.spawn(wall_t);
                    commands.spawn(wall);
                },
                Species::Psychic => {
                    let psy = PsychicBundle::new()
                        .with_position(x, y)
                        .with_axiom_kits(vec![AxiomKit::Motion])
                        .with_species(Species::Psychic);
                    let theatre = TheatreBundle::new(&tex_handle).with_position(x, y).with_species(Species::Psychic);
                    commands.spawn(psy);
                    commands.spawn(theatre);
                },
                Species::Beacon => {
                    let mut rng = rand::thread_rng();
                    let mark = HylicBundle::new().with_position(rng.gen_range(0..PLAY_AREA_WIDTH), rng.gen_range(0..PLAY_AREA_HEIGHT)).with_species(Species::Beacon);
                    let x_spot = TheatreBundle::new(&tex_handle).with_sprite(1).with_position(x, y).with_species(Species::Beacon); // sprite and species should probably be merged
                    commands.spawn(mark);
                    commands.spawn(x_spot);
                }
                _ => ()
            }
        }
    }
}