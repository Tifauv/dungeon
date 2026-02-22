use bevy::prelude::*;
use avian3d::math::*;


/// The acceleration used for character movement.
#[derive(Component, Deref, DerefMut)]
pub struct MovementAcceleration(Scalar);


#[derive(Component, Deref, DerefMut)]
pub struct MovementDampingFactor(Scalar);


#[derive(Component, Deref, DerefMut)]
pub struct JumpImpulse(Scalar);


#[derive(Component, Deref, DerefMut)]
pub struct MaxSlopeAngle(Scalar);


/// Bundle of movement-related components for a character.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration   : MovementAcceleration,
    damping        : MovementDampingFactor,
    jump_impulse   : JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        p_acceleration   : Scalar,
        p_damping        : Scalar,
        p_jump_impulse   : Scalar,
        p_max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration   : MovementAcceleration(p_acceleration),
            damping        : MovementDampingFactor(p_damping),
            jump_impulse   : JumpImpulse(p_jump_impulse),
            max_slope_angle: MaxSlopeAngle(p_max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}
