use crate::components::light::*;
use bevy::prelude::*;
use std::ops::DerefMut;


pub fn flicker_torch(
   mut torch_query: Single<(&mut PointLight, &Torch)>,
) {
    let (torch_light, torch) = torch_query.deref_mut();
    torch_light.intensity = torch.base_intensity + rand::random_range(-torch.intensity_variation..=torch.intensity_variation);
}
