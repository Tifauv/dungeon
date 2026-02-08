use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::f32::consts::FRAC_PI_4;
use std::ops::{Deref, DerefMut};
use crate::components::player::*;


pub fn move_player(
    //p_mouse_motion
    p_action: Query<&ActionState<UserAction>, With<Player>>,
    mut p_player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
    p_timer: Res<Time>,
) {
    debug!(">> System move_player");
    let action_state = p_action.single().expect("Player actions not found");
    let (player_transform, camera_sensitivity) = p_player.deref_mut();

    // Camera rotation
    let look_axis = action_state.clamped_axis_pair(&UserAction::LookAround);
    debug!("Look:");
    debug!("  distance: {}", look_axis.length());
    debug!("         x: {}", look_axis.x);
    debug!("         y: {}", look_axis.y);

    let delta_yaw   = -look_axis.x * camera_sensitivity.x;
    let delta_pitch = -look_axis.y * camera_sensitivity.y;

    let (mut yaw, mut pitch, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
    yaw   += delta_yaw;
    pitch = (pitch + delta_pitch).clamp(-FRAC_PI_4, FRAC_PI_4);

    player_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

    // Move
    let move_axis = action_state.clamped_axis_pair(&UserAction::Move);
    debug!("Move:");
    debug!("  distance: {}", move_axis.length());
    debug!("         x: {}", move_axis.x);
    debug!("         y: {}", move_axis.y);

    let flat_rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0);
    let mut velocity = move_axis.extend(0.0).xzy();
    velocity.z = -velocity.z;

    player_transform.translation += flat_rotation * velocity * p_timer.delta_secs();

    debug!("<< System move_player");
}


pub fn move_top_camera(
    p_player: Single<(&PlayerFollowers, &Transform), With<Player>>,
    mut p_cameras: Query<(&AbovePlayer, &mut Transform), (With<Camera>, Without<Player>)>,
) {
    debug!(">> System move_top_camera");

    let (player_followers, player_transform) = p_player.deref();

    for follower in player_followers.iter() {
        if let Ok((above, mut camera_transform)) = p_cameras.get_mut(follower) {
            // Move the camera with the player
            camera_transform.translation = player_transform.translation + Vec3::new(0., above.altitude, 0.0);
            // Look at the player
            camera_transform.look_at(player_transform.translation, Vec3::NEG_Z);
        }
    }

    debug!("<< System move_top_camera");
}
