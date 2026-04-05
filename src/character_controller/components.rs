use bevy::prelude::*;
use avian3d::prelude::*;
use avian3d::math::*;

use crate::components::movement::*;


#[derive(Component)]
pub struct CharacterController;


#[derive(Component, Deref, DerefMut)]
pub struct ControllerGravity(Vector);

impl ControllerGravity {
    pub fn default_vector() -> Vector {
        Vector::NEG_Y * 9.81
    }

}
impl Default for ControllerGravity {
    fn default() -> Self {
        Self(Self::default_vector())
    }
}

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body                : RigidBody,
    collider            : Collider,
    ground_caster       : ShapeCaster,
    gravity             : ControllerGravity,
    movement            : MovementBundle,
}

impl CharacterControllerBundle {
    pub fn new(p_collider: Collider, p_gravity: Vector) -> Self {
        // The shape caster is slightly smaller than the collider
        let mut caster_shape = p_collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body                : RigidBody::Kinematic,
            collider            : p_collider,
            ground_caster       : ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_distance(0.2),
            gravity             : ControllerGravity(p_gravity),
            movement            : MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        p_acceleration   : Scalar,
        p_damping        : Scalar,
        p_jump_impulse   : Scalar,
        p_max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(
            p_acceleration,
            p_damping,
            p_jump_impulse,
            p_max_slope_angle
        );
        self
    }
}
