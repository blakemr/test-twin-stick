use amethyst::{
    core::{nalgebra::Vector3, Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::components::player::Player;

const MOVEMENT_SCALE: f32 = 100.0;

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, players, input, time): Self::SystemData) {
        // Stabalize movement relative to framerate
        let delta = time.delta_seconds();

        // Player movement tracker
        let mut velocity: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        for (transform, _) in (&mut transforms, &players).join() {
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

            transform.move_global(velocity * delta * MOVEMENT_SCALE);
        }
    }
}
