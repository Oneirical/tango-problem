use bevy::prelude::*;

use crate::SpriteSheetHandle;

pub struct PsychicPlugin;

impl Plugin for PsychicPlugin {
    fn build(&self, app: &mut App) {
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

impl PsychicBundle {
    pub fn new(
        tex_handle: &SpriteSheetHandle
    ) -> Self {
        let texture_atlas_handle = &tex_handle.handle;
        Self{
            sprite_bundle : SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite{
                    index : 0 as usize,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
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
    pub fn with_position(mut self, x: u32, y: u32) -> Self {
        self.position.x = x;
        self.position.y = y;
        self
    }
}

#[derive(Component)]
pub struct Psychic{}

#[derive(Component)]
pub struct Soul{}

#[derive(Resource)]
pub struct PsychicSettings {
    pub(crate) number_at_start: u32,
}

#[derive(Component)]
pub struct Position{
    x: u32,
    y: u32,
}

pub fn distribute_psychics(
    mut commands: Commands,
    psy_settings: Res<PsychicSettings>,
    tex_handle: Res<SpriteSheetHandle>,
    //settings: &mut Settings,
    //soul: Option<Vec<NeuNet>>,
){
    let psy_amount = psy_settings.number_at_start;
    for i in 0..psy_amount{
        let psy = PsychicBundle::new(&tex_handle)
            .with_position(10+i, 20);
        commands.spawn(psy);
    }

}