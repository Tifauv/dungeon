use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::components;
use crate::state;
use crate::systems;
use crate::observers;
use crate::plugins;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, p_app: &mut App) {
        p_app
            .insert_resource(GlobalAmbientLight::NONE)
            .add_plugins((
                PhysicsPlugins::default(),
                EnhancedInputPlugin,
                plugins::character_controller::CharacterControllerPlugin,
            ))
            .add_input_context::<components::player::Player>()
            .add_observer(observers::input::capture_cursor)
            .add_observer(observers::input::release_cursor)
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
                //systems::actions::print_started_collisions,
                //systems::actions::print_stopped_collisions,
                systems::ui::set_camera_viewports,
                systems::light::flicker_torch,
                systems::actions::move_top_camera,
            ));
    }
}
