mod components;
mod systems;
mod state;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
	fn build(&self, p_app: &mut App) {
		p_app
			.insert_resource(GlobalAmbientLight::NONE)
			.add_plugins(InputManagerPlugin::<components::player::UserAction>::default())
			.add_systems(Startup, (
				state::level00::spawn_map,
				state::level00::spawn_player,
				// Those are for debug only !
				/*
				state::level00::spawn_axis,
				state::level00::spawn_global_light,
				*/
			))
			.add_systems(Update, (
				systems::ui::set_camera_viewports,
				systems::cursor::draw,
				systems::light::flicker_torch,
				(
					systems::actions::move_player,
					systems::actions::move_top_camera,
				).chain(),
			));
	}
}


fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(DungeonPlugin)
		.run();
}
