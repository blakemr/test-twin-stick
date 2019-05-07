use amethyst::{
    core::{nalgebra::Vector3, Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::Flipped,
};

use crate::components::player::Player;

const MOVEMENT_SCALE: f32 = 100.0;

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Flipped>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut flipped, players, input, time): Self::SystemData) {
        // Stabalize movement relative to framerate
        let delta = time.delta_seconds();

        // Player movement tracker
        let mut velocity: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        for (transform, flip, _) in (&mut transforms, &mut flipped, &players).join() {
            if let Some(mv_amount) = input.axis_value("player_x") {
                velocity[0] = mv_amount as f32;

                // If player is moving left, turn the player left.
                // Else If player is moving right, turn the player right.
                if velocity[0] > 0.0 {
                    *flip = Flipped::Horizontal;
                } else if velocity[0] < 0.0 {
                    *flip = Flipped::None;
                }
            }
            if let Some(mv_amount) = input.axis_value("player_y") {
                velocity[1] = mv_amount as f32;
            }

            // Make sure its not faster to go diagonally.
            // Must check to not normalize 0 length vector.
            if velocity.magnitude() != 0.0 {
                velocity = velocity.normalize();
            }

            transform.move_global(velocity * delta * MOVEMENT_SCALE);
        }
    }
}
