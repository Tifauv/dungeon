mod color;
//mod audio;
mod dungeon;
mod systems;

use amethyst::{
	core::transform::TransformBundle,
	input::{InputBundle, StringBindings},
	prelude::*,
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle,
	},
	ui::{RenderUi, UiBundle},
	utils::application_root_dir
};

//use crate::audio::Music;
use crate::color::to_linear_rgba;
use crate::dungeon::Dungeon;


fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	// Resources directories and files
	let app_root = application_root_dir()?;
	let display_config_path = app_root.join("config").join("display.ron");
	let binding_path        = app_root.join("config").join("bindings.ron");
	let assets_dir          = app_root.join("assets");
	
	let input_bundle = InputBundle::<StringBindings>::new()
		.with_bindings_from_file(binding_path)?;
	
	let game_data = GameDataBuilder::default()
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				// The RenderToWindow plugin provides all the scaffolding for opening a window an drawing on it.
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)?
						.with_clear(to_linear_rgba([35.0, 38.0, 41.0, 1.0])) // #232629
				)
				// The RenderFlat2D plugin is used to render entities with a 'SpriteRender' component.
				.with_plugin(RenderFlat2D::default())
				.with_plugin(RenderUi::default())
		)?
		.with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?
		.with_bundle(UiBundle::<StringBindings>::new())?
//		.with_bundle(AudioBundle::default())?
/*		.with_system_desc(
			DjSystemDesc::new(|music: &mut Music| music.music.next()),
			"dj_system",
			&[]
		)
*/
//		.with(systems::PaddleSystem,       "paddle_system", &["input_system"])
//		.with(systems::MoveBallsSystem,      "ball_system", &[])
//		.with(systems::BounceSystem,    "collision_system", &["paddle_system", "ball_system"])
		.with(systems::MovePlayerSystem::default(), "player_system", &["input_system"])
		.with(systems::WallsSystem,                 "walls_system",  &["player_system"])
		;
	
	let mut game = Application::new(assets_dir, Dungeon::default(), game_data)?;
	game.run();

	Ok(())
}
