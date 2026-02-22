use bevy::prelude::*;
use avian3d::prelude::*;
use avian3d::math::*;
use leafwing_input_manager::prelude::ActionState;
use std::f32::consts::FRAC_PI_4;

use crate::components::base::*;
use crate::components::player::*;
use crate::components::movement::*;
use crate::components::character_controller::*;



/// Dynamically adds or removes the Grounded component on a CharacterController.
///
pub fn update_grounded(
    mut p_commands: Commands,
    mut p_query: Query<(Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
                       With<CharacterController>>,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut p_query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= **angle
            }
            else {
                true
            }
        });

        if is_grounded {
            p_commands.entity(entity).insert(Grounded);
        }
        else {
            p_commands.entity(entity).remove::<Grounded>();
        }
    }
}


/// Apply gravity to the CharacterControllers.
pub fn apply_gravity(
    p_time: Res<Time>,
    mut p_controllers: Query<(&ControllerGravity, &mut LinearVelocity)>,
) {
    for (gravity, mut linear_velocity) in &mut p_controllers {
        linear_velocity.0 += **gravity * p_time.delta_secs();
    }
}


pub fn rotate_character(
    p_time: Res<Time>,
    p_action: Query<&ActionState<UserAction>, With<Player>>,
    mut p_controllers: Query<(&mut Transform, &CameraSensitivity),
                         With<Player>>,
) {
    let action_state = p_action.single().expect("Player actions not found");

    for (mut transform, camera_sensitivity) in &mut p_controllers {
        let look_axis = action_state.clamped_axis_pair(&UserAction::LookAround);
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let delta_yaw   = -look_axis.x * camera_sensitivity.x * p_time.delta_secs();
        let delta_pitch = -look_axis.y * camera_sensitivity.y * p_time.delta_secs();
        yaw  += delta_yaw;
        pitch = (pitch + delta_pitch).clamp(-FRAC_PI_4, FRAC_PI_4);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
}


/// Move the CharacterControllers.
pub fn move_character(
    p_time: Res<Time>,
    p_action: Query<&ActionState<UserAction>, With<Player>>,
    mut p_controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &Transform,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    let action_state = p_action.single().expect("Player actions not found");
    let move_axis = action_state.clamped_axis_pair(&UserAction::Move);

    for (movement_acceleration, jump_impulse, transform, mut linear_velocity, is_grounded) in &mut p_controllers {
        let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let flat_rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0);

        let movement: Vec3 = Vec3::new(move_axis.x, 0.0, -move_axis.y);
        linear_velocity.0 += flat_rotation * movement * **movement_acceleration * p_time.delta_secs();
    }
}


pub fn apply_movement_damping(mut p_query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut p_query {
        linear_velocity.x *= **damping_factor;
        linear_velocity.z *= **damping_factor;
    }
}


#[allow(clippy::type_complexity)]
pub fn kinematic_controller_collisions(
    p_collisions: Collisions,
    p_bodies: Query<&RigidBody>,
    p_collider_rbs: Query<&ColliderOf, Without<Sensor>>,
    mut p_controllers: Query<
       (&mut Position, &mut LinearVelocity, Option<&MaxSlopeAngle>),
       (With<RigidBody>, With<CharacterController>)
    >,
    p_time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in p_collisions.iter() {
        // Get the rigid body entities of the colliders (could be children)
        let Ok([&ColliderOf { body: rb1 }, &ColliderOf { body: rb2 }]) =
            p_collider_rbs.get_many([contacts.collider1, contacts.collider2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first or second entity
        // in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = p_controllers.get_mut(rb1) {
                is_first = true;
                character_rb = *p_bodies.get(rb1).unwrap();
                is_other_dynamic = p_bodies.get(rb2).is_ok_and(|rb| rb.is_dynamic());
                character
            }
            else if let Ok(character) = p_controllers.get_mut(rb2) {
                is_first = false;
                character_rb = *p_bodies.get(rb2).unwrap();
                is_other_dynamic = p_bodies.get(rb1).is_ok_and(|rb| rb.is_dynamic());
                character
            }
            else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.normal
            }
            else {
                manifold.normal
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.points.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_between(Vector::Y);
            let climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= **angle);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character up and down
                // the surface smoothly.
                if climbable {
                    // Points in the normal's direction in the XZ plane.
                    let normal_direction_xz = normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_xz = linear_velocity.dot(normal_direction_xz);

                    // Snap the Y speed based on the speed at which the character is moving up or down the slope,
                    // and how steep the slope is.
                    let max_y_speed = -linear_velocity_xz * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                }
                else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to a collide-and-slide
                    // algorithm.

                    // Do nothing if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            }
            else {
                // The character is not yet intersecting the other object, but the narrow phase detected
                // a speculative collision.
                // We need to push back the part of the velocity that would cause penetration within the
                // next frame.
                let normal_speed = linear_velocity.dot(normal);

                // Do nothing if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply
                let impulse_magnitude = normal_speed - (deepest_penetration / p_time.delta_secs());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                }
                else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}
