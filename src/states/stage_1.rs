use amethyst::{prelude::*, utils::application_root_dir};

use crate::components::player::Player;
use crate::entities::{camera::init_camera, player::init_player};
use crate::utils::textures::{load_spritesheet, load_texture};

pub struct Stage1;

pub const ARENA_WIDTH: f32 = 384.0;
pub const ARENA_HEIGHT: f32 = 256.0;

const PLAYER_INIT_X: f32 = ARENA_WIDTH / 2.0;
const PLAYER_INIT_Y: f32 = ARENA_HEIGHT / 2.0;

impl SimpleState for Stage1 {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let player_texture_path = format!(
            "{}/resources/textures/0x72_16x16DungeonTileset.v4.png",
            application_root_dir()
        );
        let spritesheet_path = format!(
            "{}/resources/textures/0x72_16*16DungeonTileset.ron",
            application_root_dir()
        );

        let texture_handle = load_texture(player_texture_path, &world);
        let sprite_sheet_handle = load_spritesheet(spritesheet_path, texture_handle, &world);

        // Temp code. to be removed once we get some systems in place. See pong.
        world.register::<Player>();

        init_player(world, PLAYER_INIT_X, PLAYER_INIT_Y, sprite_sheet_handle);
        init_camera(world, ARENA_WIDTH, ARENA_HEIGHT);
    }
}
