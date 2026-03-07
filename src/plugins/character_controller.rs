use bevy::prelude::*;
use avian3d::prelude::*;

use crate::systems::character_controller::*;
use crate::observers::character_controller::*;


pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, p_app: &mut App) {
        p_app
            .add_observer(apply_rotation)
            .add_observer(apply_movement)
            .add_observer(apply_jump)
            .add_systems(
                Update,
                (
                    update_grounded,
                    apply_gravity,
                    apply_movement_damping,
                ).chain(),
            )
            .add_systems(
                PhysicsSchedule,
                kinematic_controller_collisions.in_set(NarrowPhaseSystems::Last),
            );
    }
}
