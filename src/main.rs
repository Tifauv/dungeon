mod components;
mod state;
mod systems;
mod plugins;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::plugins::dungeon::DungeonPlugin;


fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin::default(),
			WorldInspectorPlugin::default(),
		))
		.add_plugins(DungeonPlugin)
		.run();
}
