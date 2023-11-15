use bevy::reflect::Reflect;

#[derive(Clone, PartialEq, Copy, Reflect)]
pub enum Axiom{
    Move{dx: i32, dy: i32}
}

impl Axiom{
    pub fn act_motion(self) -> (i32, i32) {
        match self{
            Axiom::Move { dx, dy } => (dx, dy)
        }
    }
}

pub enum AxiomKit{
    Motion,
}

impl AxiomKit{
    pub fn unpack(self) -> Vec<Axiom>{
        match self{
            AxiomKit::Motion => vec![Axiom::Move { dx: 0, dy: 1 }, Axiom::Move { dx: 0, dy: -1 }, Axiom::Move { dx: -1, dy: 0 }, Axiom::Move { dx: 1, dy: 0 }, Axiom::Move { dx: 0, dy: 0 }]
        }
    }
}