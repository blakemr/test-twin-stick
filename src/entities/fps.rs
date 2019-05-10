use amethyst::{
    assets::Loader,
    ecs::Entity,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub struct FpsText {
    pub fps: Entity,
}

pub fn init_fps_display(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "resources/fonts/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let fps_transform = UiTransform::new(
        "FPS".to_string(),
        Anchor::TopLeft,
        50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
        0,
    );

    let fps = world
        .create_entity()
        .with(fps_transform)
        .with(UiText::new(
            font.clone(),
            "N".to_string(),
            [0.0, 0.0, 0.0, 1.0],
            50.0,
        ))
        .build();

    world.add_resource(FpsText { fps });
}
