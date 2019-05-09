use amethyst::{
    ecs::{Read, ReadExpect, System, WriteStorage},
    ui::UiText,
    utils::fps_counter::FPSCounter,
};

use crate::entities::fps::FpsText;

pub struct FpsDisplaySystem;

impl<'s> System<'s> for FpsDisplaySystem {
    type SystemData = (
        Read<'s, FPSCounter>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, FpsText>,
    );

    fn run(&mut self, (fps, mut ui_text, fps_text): Self::SystemData) {
        println!("{}", fps.frame_fps() as i32);

        // if let Some(text) = ui_text.get_mut(fps_text) {
        //     text.text = (fps.frame_fps() as i32).to_string();
        // }
    }
}
