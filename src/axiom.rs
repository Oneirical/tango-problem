use bevy::ecs::system::ResMut;

use crate::{map::{Species, Map}, simulation::get_adjacent_coords};

#[derive(Clone, PartialEq, Copy)]
pub enum Axiom{
    Move{dx: i32, dy: i32},
    PaintAdjacent{ color: Species },
    SpeciesTransform{new_species: Species},
    Void
}

impl Axiom{
    pub fn act_motion(self) -> (i32, i32) {
        match self{
            Axiom::Move { dx, dy } => (dx, dy),
            _ => (0, 0)
        }
    }
    pub fn act_transform(self, original_species: Species) -> Species {
        match self{
            Axiom::SpeciesTransform { new_species } => new_species,
            _ => original_species
        }
    }
    pub fn act_axioms(
        self,
        pos: (u32, u32),
        map: &ResMut<Map>,
    ) -> Vec<(Axiom, (u32, u32))>{
        let mut output = Vec::new();
        match self{
            Axiom::PaintAdjacent {color} => {
                for i in get_adjacent_coords(pos){
                    if map.tiles[map.xy_idx(i.0, i.1)] == Species::Wall{
                        output.push((Axiom::SpeciesTransform { new_species: color }, i));
                    }
                }
            },
            _ => ()
        };
        output
    }
}

pub enum AxiomKit{
    Motion,
    PaintKit
}

impl AxiomKit{
    pub fn unpack(self) -> Vec<Axiom>{
        match self{
            AxiomKit::Motion => vec![Axiom::Move { dx: 0, dy: 1 }, Axiom::Move { dx: 0, dy: -1 }, Axiom::Move { dx: -1, dy: 0 }, Axiom::Move { dx: 1, dy: 0 }, Axiom::Move { dx: 0, dy: 0 }], // this might not be that good - hard to encourage action diversity by fitness? See Tango Problem
            AxiomKit::PaintKit => vec![Axiom::Move { dx: 0, dy: 1 }, Axiom::Move { dx: 0, dy: -1 }, Axiom::Move { dx: -1, dy: 0 }, Axiom::Move { dx: 1, dy: 0 }, Axiom::PaintAdjacent {color: Species::TermiPainted}],
        }
    }
}