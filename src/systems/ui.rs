use bevy::prelude::*;
use bevy::camera::Viewport;
use bevy::window::WindowResized;
use crate::components::base::*;

pub fn set_camera_viewports(
    windows: Query<&Window>,
    mut window_resized_reader: MessageReader<WindowResized>,
    mut query: Query<(&CameraView, &mut Camera)>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for window_resized in window_resized_reader.read() {
        let window = windows.get(window_resized.window).unwrap();

        for (view, mut camera) in &mut query {
            let size = window.physical_size().as_vec2() * view.size;
            camera.viewport = Some(Viewport {
                physical_position: view.pos * size.as_uvec2(),
                physical_size: size.as_uvec2(),
                ..default()
            });
        }
    }
}
