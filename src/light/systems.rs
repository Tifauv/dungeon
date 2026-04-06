use bevy::prelude::*;
use std::ops::DerefMut;

use crate::light::components::*;


pub fn flicker(mut query: Single<(&mut PointLight, &Flickering)>) {
    let (light, flickering) = query.deref_mut();
    light.intensity = flickering.base_intensity + rand::random_range(-flickering.intensity_delta..=flickering.intensity_delta);
}
