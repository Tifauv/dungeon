use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::components;
use crate::state;
use crate::systems;
use crate::input;
use crate::character_controller;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, p_app: &mut App) {
        p_app
            .insert_resource(GlobalAmbientLight::NONE)
            .add_plugins((
                PhysicsPlugins::default(),
                EnhancedInputPlugin,
                character_controller::plugin::CharacterControllerPlugin,
            ))
            .add_input_context::<components::player::Player>()
            .add_observer(input::observers::capture_cursor)
            .add_observer(input::observers::release_cursor)
            .add_systems(Startup, (
                state::level00::setup,
                /*state::level00::setup_debug,*/
            ))
            // Disable mouse actions when hovering a UI component
            .add_systems(PreUpdate, input::systems::disable_mouse.before(EnhancedInputSystems::Update))
            .add_systems(Update, (
                systems::ui::set_camera_viewports,
                systems::light::flicker_torch,
                systems::actions::move_top_camera,
            ));
    }
}
