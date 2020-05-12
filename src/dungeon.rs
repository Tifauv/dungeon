use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::timing::Time,
	core::transform::Transform,
	ecs::prelude::{Component, DenseVecStorage, Entity},
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
	ui::{Anchor, TtfFormat, UiText, UiTransform}
};


// World
pub const WORLD_HEIGHT: f32 = 1024.0;
pub const WORLD_WIDTH:  f32 = 1024.0;

// Camera
pub const CAMERA_HEIGHT: f32 = 100.0;
pub const CAMERA_WIDTH:  f32 = 100.0;


/// Initializes the camera in a way that our screen covers (100x100) above the whole world and is in the bottom left.
fn initialize_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(WORLD_WIDTH * 0.5, WORLD_HEIGHT * 0.5, 1.0);
	
	world
		.create_entity()
		.with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
		.with(transform)
		.build();
}


#[derive(Default)]
pub struct Dungeon;

impl SimpleState for Dungeon {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		
		initialize_camera(world);
	}
}
