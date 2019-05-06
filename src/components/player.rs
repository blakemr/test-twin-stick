use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum Facing {
    Left,
    Right,
}

pub struct Player {
    pub facing: Facing,
    pub width: f32,
    pub height: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            facing: Facing::Left,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
