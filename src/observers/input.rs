use bevy::prelude::*;
use bevy::window::CursorOptions;
use bevy_enhanced_input::prelude::*;

use crate::components::input_actions::*;
use crate::resources::input::grab_cursor;


pub fn capture_cursor(
    _on: On<Complete<CaptureCursor>>,
    mut p_cursor: Single<&mut CursorOptions>,
) {
    grab_cursor(&mut p_cursor, true);
}


pub fn release_cursor(
    _on: On<Complete<ReleaseCursor>>,
    mut p_cursor: Single<&mut CursorOptions>,
) {
    grab_cursor(&mut p_cursor, false);
}
