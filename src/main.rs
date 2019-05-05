extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::{Application, Config, GameDataBuilder},
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA},
};

mod components;
mod entities;
mod states;
mod systems;
mod utils;

use crate::states::stage_1::Stage1;

fn main() -> amethyst::Result<()> {
    // Start the logger
    amethyst::start_logger(Default::default());

    use amethyst::utils::application_root_dir;

    // Pull basic settings from RON file
    let config_path = format!("{}/resources/display_config.ron", application_root_dir());
    let binding_path = format!("{}/resources/input_bindings.ron", application_root_dir());

    let config = DisplayConfig::load(&config_path);
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([256.0, 256.0, 256.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            // System to use
            systems::PlayerMovementSystem,
            // Some arbitrary name (used for dependance tracking)
            "player_movement_system",
            // List of dependancies
            // I guess this is in the input_bundle???
            //      update: yup. its hiding in there. so this is one
            //              of those things you just "have to know."
            &["input_system"],
        );

    let mut game = Application::new("./", Stage1, game_data)?;

    game.run();

    Ok(())
}
