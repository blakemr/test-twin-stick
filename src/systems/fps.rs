use amethyst::{
    ecs::{Read, System},
    utils::fps_counter::FPSCounter,
};

pub struct FpsDisplaySystem;

impl<'s> System<'s> for FpsDisplaySystem {
    type SystemData = Read<'s, FPSCounter>;

    fn run(&mut self, fps: Self::SystemData) {
        println!("{}", fps.frame_fps());
    }
}
