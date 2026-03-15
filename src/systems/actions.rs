use bevy::prelude::*;
use avian3d::prelude::*;
use std::ops::Deref;

use crate::components::player::*;


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
