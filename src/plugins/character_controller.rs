use bevy::prelude::*;
use avian3d::prelude::*;

use crate::systems::character_controller::*;


pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, p_app: &mut App) {
        p_app
            .add_systems(
                Update,
                (
                    rotate_character,
                    update_grounded,
                    apply_gravity,
                    move_character,
                    apply_movement_damping,
                ).chain(),
            )
            .add_systems(
                PhysicsSchedule,
                kinematic_controller_collisions.in_set(NarrowPhaseSystems::Last),
            );
    }
}
