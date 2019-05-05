// textures.rs: standard or common texutre related functions
use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureHandle,
        TextureMetadata,
    },
};

pub fn load_texture<N>(name: N, world: &World) -> TextureHandle
where
    N: Into<String>,
{
    // read resource into world.
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

pub fn load_spritesheet<N>(
    name: N,
    texture_handle: TextureHandle,
    world: &World,
) -> SpriteSheetHandle
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        SpriteSheetFormat,
        texture_handle,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}
