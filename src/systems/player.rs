use amethyst::{
    core::{nalgebra::Vector2, Time, Transform},
    ecs::{Join, Read, System, WriteStorage},
    input::InputHandler,
};

// use crate::components::player::Player;

const MOVEMENT_SCALE_X: f32 = 1.2;
const MOVEMENT_SCALE_Y: f32 = 1.2;

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, input, time): Self::SystemData) {
        // Stabalize movement relative to framerate
        let delta = time.delta_seconds();

        // Player movement tracker
        let mut velocity: Vector2<f32> = Vector2::new(1.0, 0.0);

        for transform in (&mut transforms).join() {
            if let Some(mv_amount) = input.axis_value("player_x") {
                velocity[0] = mv_amount as f32;
            }
            if let Some(mv_amount) = input.axis_value("player_y") {
                velocity[1] = mv_amount as f32;
            }

            // Make sure its not faster to go diagonally.
            // Must check to not normalize 0 length vector.
            if velocity.magnitude() != 0.0 {
                velocity = velocity.normalize();
            }

            // Test code. please ignore.
            println!("x: {}", velocity[0]);
            println!("y: {}", velocity[1]);

            transform.translate_x(velocity[0] * delta * MOVEMENT_SCALE_X);
            transform.translate_y(velocity[1] * delta * MOVEMENT_SCALE_Y);
        }
    }
}
