use bevy::prelude::*;
use bevy::window::CursorOptions;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;
use std::f32::consts::FRAC_PI_4;

use crate::components::base::Grounded;
use crate::components::player::*;
use crate::components::movement::*;
use crate::components::input_actions;


pub fn apply_rotation(
    p_action: On<Fire<input_actions::LookAround>>,
    mut p_controllers: Query<&mut Transform, With<Player>>,
    p_cursor: Single<&CursorOptions>,
) {
    // Rotate only when the cursor has been grabbed
    // TODO Disable only if action is triggered by mouse ?
    if p_cursor.visible {
        return;
    }

    // TODO properly manage unwrap errors ?
    let mut transform = p_controllers.get_mut(p_action.context).unwrap();

    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
    yaw   += p_action.value.x.to_radians();
    pitch += p_action.value.y.to_radians();

    // Limit pitch to [-PI/4, +PI/4]
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch.clamp(-FRAC_PI_4, FRAC_PI_4), 0.0);
}


pub fn apply_movement(
    p_action: On<Fire<input_actions::Move>>,
    mut p_controllers: Query<(
        &MovementAcceleration,
        &Transform,
        &mut LinearVelocity),
        With<Player>
        >,
) {
    // TODO properly manage unwrap errors ?
    let (movement_acceleration, transform, mut linear_velocity) = p_controllers.get_mut(p_action.context).unwrap();

    // Move to the camera direction, only on a flat level (yaw)
    let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
    let flat_rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0);

    // Movement consists of X and -Z components, so swap Y and Z with negation.
    let mut velocity = p_action.value.extend(0.0).xzy();
    velocity.z = -velocity.z;

    linear_velocity.0 += flat_rotation * velocity * **movement_acceleration;
}


pub fn apply_jump(
    p_action: On<Fire<input_actions::Jump>>,
    mut p_controllers: Query<(&mut LinearVelocity, &JumpImpulse, Has<Grounded>), With<Player>>,
) {
    // TODO properly manage unwrap errors ?
    let (mut velocity, jump_impulse, is_grounded) = p_controllers.get_mut(p_action.context).unwrap();

    if is_grounded {
        velocity.y = **jump_impulse;
    }
}
