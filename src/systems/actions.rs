use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::ops::{Deref, DerefMut};
use crate::components::player::*;


pub fn move_player(
    p_action: Query<&ActionState<UserAction>, With<Player>>,
    mut p_player: Single<(&Player, &mut Transform), Without<Camera>>,
    mut p_camera: Query<(&ChildOf, &mut Transform), With<Camera>>,
    p_timer: Res<Time>,
) {
    debug!(">> System move_player");
    let action_state = p_action.single().expect("Player actions not found");

    // Move
    let move_axis = action_state.clamped_axis_pair(&UserAction::Move);
    debug!("Move:");
    debug!("  distance: {}", move_axis.length());
    debug!("         x: {}", move_axis.x);
    debug!("         y: {}", move_axis.y);

    let (_player, player_transform) = p_player.deref_mut();
    player_transform.translation.x += move_axis.x * p_timer.delta_secs();
    player_transform.translation.z -= move_axis.y * p_timer.delta_secs();

    // Camera rotation
    let look_axis = action_state.clamped_axis_pair(&UserAction::LookAround);
    debug!("Look:");
    debug!("  distance: {}", look_axis.length());
    debug!("         x: {}", look_axis.x);
    debug!("         y: {}", look_axis.y);

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
