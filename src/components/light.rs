use bevy::prelude::*;


#[derive(Component)]
pub struct Torch {
    pub base_intensity: f32,
    pub intensity_variation: f32,
}
