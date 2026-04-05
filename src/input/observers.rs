use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_enhanced_input::prelude::*;

use crate::input::components as actions;


pub fn capture_cursor(
    _on: On<Complete<actions::CaptureCursor>>,
    mut p_cursor: Single<&mut CursorOptions>,
) {
    grab_cursor(&mut p_cursor);
}


pub fn release_cursor(
    _on: On<Complete<actions::ReleaseCursor>>,
    mut p_cursor: Single<&mut CursorOptions>,
) {
    ungrab_cursor(&mut p_cursor);
}


fn grab_cursor(p_cursor: &mut CursorOptions) {
    p_cursor.grab_mode = CursorGrabMode::Confined;
    p_cursor.visible = false;
}


fn ungrab_cursor(p_cursor: &mut CursorOptions) {
    p_cursor.grab_mode = CursorGrabMode::None;
    p_cursor.visible = true;
}
