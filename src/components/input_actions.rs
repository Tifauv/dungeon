use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;


#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Move;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct LookAround;

#[derive(InputAction)]
#[action_output(bool)]
pub struct CaptureCursor;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ReleaseCursor;
