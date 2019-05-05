use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{Camera, Projection},
};

/// Camera entity
///
/// Both width and height stretch or compress things
/// in view to fit the screen
///
/// # args:
///
/// * world: world camera belongs to
/// * width: how wide the camera screen is.
/// * height: how tall the camera screen is
///
pub fn init_camera(world: &mut World, width: f32, height: f32) {
    // Set camera position (this is static for this game)
    let mut transform = Transform::default();
    transform.set_z(1.0);

    // Build entity
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, width, 0.0, height,
        )))
        .with(transform)
        .build();
}
