use bevy::prelude::*;


#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct CameraView {
    pub pos: UVec2,
    pub size: Vec2,
}
