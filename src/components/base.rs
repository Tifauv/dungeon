use bevy::prelude::*;


#[derive(Component)]
pub struct Size(Vec3);

#[derive(Component)]
pub struct Position(Vec3);

#[derive(Component)]
pub struct Rotation(Vec3);

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct CameraPosition {
    pub pos: UVec2,
}
