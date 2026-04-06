use bevy::prelude::*;


#[derive(Component)]
pub struct Flickering {
    pub base_intensity : f32,
    pub intensity_delta: f32,
}
