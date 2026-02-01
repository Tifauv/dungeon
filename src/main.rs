mod components;
mod systems;
mod state;

use bevy::prelude::*;


pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
	fn build(&self, p_app: &mut App) {
		p_app.add_systems(Startup, (
			state::level00::spawn_axis,
			state::level00::spawn_map,
			state::level00::spawn_player,
			state::level00::spawn_top_view,
		));
		//p_app.add_systems(Update,  systems::map::show);
		p_app.add_systems(Update,  (
			systems::ui::set_camera_viewports,
			systems::cursor::draw,
			systems::light::flicker_torch,
		));
	}
}


fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(GlobalAmbientLight::NONE)
		.add_plugins(DungeonPlugin)
		.run();
}
