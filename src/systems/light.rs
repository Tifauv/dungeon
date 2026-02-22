use bevy::prelude::*;
use std::ops::DerefMut;

use crate::components::torch::*;


pub fn flicker_torch(mut torch_query: Single<(&mut PointLight, &Torch)>) {
    let (torch_light, torch) = torch_query.deref_mut();
    torch_light.intensity = torch.base_intensity + rand::random_range(-torch.intensity_delta..=torch.intensity_delta);
}
