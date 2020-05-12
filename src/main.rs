//mod audio;
mod dungeon;
//mod systems;

use amethyst::{
	audio::{AudioBundle, DjSystemDesc},
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
						.with_clear([
							// Linear colorspace for #232629
							f32::powf((35.0 / 255.0 + 0.055) / 1.055, 2.4), // R 
							f32::powf((38.0 / 255.0 + 0.055) / 1.055, 2.4), // G
							f32::powf((41.0 / 255.0 + 0.055) / 1.055, 2.4), // B
							1.0])                                           // A
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
//		.with(systems::WinnerSystem,       "winner_system", &["ball_system"])
		;
	
	let mut game = Application::new(assets_dir, Dungeon::default(), game_data)?;
	game.run();

	Ok(())
}
