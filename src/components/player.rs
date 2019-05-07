use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player {
    pub width: f32,
    pub height: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
