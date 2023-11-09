use bevy::prelude::*;
use bevy::sprite::Anchor::BottomLeft;

use crate::SpriteSheetHandle;

pub struct PsychicPlugin;

impl Plugin for PsychicPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PsychicSettings{number_at_start: 16});
        app.add_systems(Startup, distribute_psychics);
    }
}

#[derive(Bundle)]
pub struct PsychicBundle {
    sprite_bundle: SpriteSheetBundle,
    psychic: Psychic,
    soul: Soul,
    position: Position,

}

impl PsychicBundle { // This is the start of something great. 8th November 2023
    pub fn new(
        tex_handle: &SpriteSheetHandle
    ) -> Self {
        let texture_atlas_handle = &tex_handle.handle;
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
            psychic: Psychic {  },
            soul: Soul {  },
            position: Position { x: 0, y: 0 },
        }
    }
    pub fn with_position(mut self, x: u32, y: u32) -> Self { // Absolutely immaculate!
        self.position.x = x;
        self.position.y = y;
        self.sprite_bundle.transform.translation.x = x as f32 * 16.0;
        self.sprite_bundle.transform.translation.y = y as f32 * 16.0;
        self
    }
}

#[derive(Component)]
pub struct Psychic{}

#[derive(Component)]
pub struct Soul{}

#[derive(Resource)]
pub struct PsychicSettings {
    number_at_start: u32,
}

#[derive(Component)]
pub struct Position{
    x: u32,
    y: u32,
}

fn distribute_psychics(
    mut commands: Commands,
    psy_settings: Res<PsychicSettings>,
    tex_handle: Res<SpriteSheetHandle>,
    //settings: &mut Settings,
    //soul: Option<Vec<NeuNet>>,
){
    let psy_amount = psy_settings.number_at_start;
    for i in 0..psy_amount{
        let psy = PsychicBundle::new(&tex_handle)
            .with_position(0+i, 0);
        commands.spawn(psy);
    }
}