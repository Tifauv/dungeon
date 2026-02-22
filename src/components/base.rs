use bevy::prelude::*;


#[derive(Component)]
pub struct CameraView {
    pub pos: UVec2,
    pub size: Vec2,
}


/// Marks the entity is on the ground.
#[derive(Component)]
#[component(storage="SparseSet")]
pub struct Grounded;
