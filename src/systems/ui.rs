use bevy::prelude::*;
use bevy::camera::Viewport;
use bevy::window::WindowResized;
use crate::components::base::*;

pub fn set_camera_viewports(
    windows: Query<&Window>,
    mut window_resized_reader: MessageReader<WindowResized>,
    mut query: Query<(&CameraPosition, &mut Camera)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for window_resized in window_resized_reader.read() {
        let window = windows.get(window_resized.window).unwrap();
        let size = window.physical_size() / UVec2::new(2, 1);

        for (camera_position, mut camera) in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: camera_position.pos * size,
                physical_size: size,
                ..default()
            });
        }
    }
}
