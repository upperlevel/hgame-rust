use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

// textures

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage
        )
    };

    let texture_id = 0;
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    material_texture_set.insert(texture_id, texture_handle);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat,
        texture_id,
        (),
        &sprite_sheet_store
    )
}

// components

pub struct Human;

impl Human {
    fn new() -> Human {
        Human
    }
}

impl Component for Human {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_humans(world: &mut World) {
    let sprite_sheet_handle = load_sprite_sheet(world);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false
    };

    let mut transform = Transform::default();
    transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(Human::new())
        .with(transform) // deletes
        .with(sprite_render) // deletes sprite_render
        .build();
}

// camera

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(transform)
        .build();
}

// main

pub struct HGame;

impl<'a, 'b> SimpleState<'a, 'b> for HGame {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // registers the human component. in this way we initialise the storage.
        world.register::<Human>();

        initialise_camera(world);
        initialise_humans(world);
    }

    fn on_stop(&mut self, _data: StateData<GameData>) {
        println!("Game stopped")
    }
}




