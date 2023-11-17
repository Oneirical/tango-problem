use bevy::reflect::Reflect;

#[derive(Clone, PartialEq, Copy, Reflect)]
pub enum Axiom{
    Move{dx: i32, dy: i32},
    Void
}

impl Axiom{
    pub fn act_motion(self) -> (i32, i32) {
        match self{
            Axiom::Move { dx, dy } => (dx, dy),
            _ => (0, 0)
        }
    }
}

pub enum AxiomKit{
    Motion,
}

impl AxiomKit{
    pub fn unpack(self) -> Vec<Axiom>{
        match self{
            AxiomKit::Motion => vec![Axiom::Move { dx: 0, dy: 1 }, Axiom::Move { dx: 0, dy: -1 }, Axiom::Move { dx: -1, dy: 0 }, Axiom::Move { dx: 1, dy: 0 }, Axiom::Move { dx: 0, dy: 0 }] // this might not be that good - hard to encourage action diversity by fitness? See Tango Problem
        }
    }
}