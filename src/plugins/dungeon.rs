use bevy::prelude::*;
use avian3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components;
use crate::state;
use crate::systems;
use crate::plugins;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, p_app: &mut App) {
        p_app
            .insert_resource(GlobalAmbientLight::NONE)
            .add_plugins((
                PhysicsPlugins::default(),
                InputManagerPlugin::<components::player::UserAction>::default(),
                plugins::character_controller::CharacterControllerPlugin,
            ))
            .add_systems(Startup, (
                state::level00::spawn_map,
                state::level00::spawn_player,
                // Those are for debug only !
                /*
                *state::level00::spawn_axis,
                *state::level00::spawn_global_light,
                */
            ))
            .add_systems(Update, (
                systems::ui::set_camera_viewports,
                systems::cursor::draw,
                systems::light::flicker_torch,
                systems::actions::move_top_camera,
            ));
    }
}
