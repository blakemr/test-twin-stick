use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{Flipped, SpriteRender, SpriteSheetHandle},
};

use crate::components::player::Player;

/// Player entitiy
///
/// # args:
///
/// * world: world to load into
/// * x: inital x position
/// * y: initial y position
/// * sprite_sheet_handle: handle for the sprite to use
///
pub fn init_player(world: &mut World, x: f32, y: f32, sprite_sheet_handle: SpriteSheetHandle) {
    // Set position
    let mut transform = Transform::default();
    transform.set_x(x);
    transform.set_y(y);

    // Set sprite
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };

    // Build entity

    world
        .create_entity()
        .with(transform)
        .with(Player::new())
        .with(Flipped::None)
        .with(sprite_render.clone())
        .build();
}
