use crate::components::base::*;
use bevy::prelude::*;

pub fn draw(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
            // Calculate a ray pointing from the camera into the world based on the cursor's position.
            && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
            // Calculate if and where the ray is hitting the ground plane.
            && let Some(point) = ray.plane_intersection_point(ground.translation(), InfinitePlane3d::new(ground.up())) {
        // Draw a circle just above the ground plane at that position.
        gizmos.circle(
            Isometry3d::new(
                point + ground.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.2,
            Color::WHITE,
        );
    }
}
