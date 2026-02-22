use bevy::prelude::*;
use avian3d::prelude::*;


pub const TORCH_INTENSITY_BASE : f32 = 2800.;
pub const TORCH_INTENSITY_DELTA: f32 = 1400.;
pub const TORCH_LIGHT_RANGE    : f32 = 10.;
pub const TORCH_LIGHT_RADIUS   : f32 = 0.1;
pub const TORCH_RADIUS         : f32 = 0.05;


#[derive(Component)]
pub struct Torch {
    pub base_intensity: f32,
    pub intensity_delta: f32,
}


#[derive(Bundle)]
pub struct TorchBundle {
    marker   : Torch,
    name     : Name,
    mesh     : Mesh3d,
    material : MeshMaterial3d<StandardMaterial>,
    light    : PointLight,
    /*    body     : RigidBody,
    collider : Collider,*/
    transform: Transform,
}

impl TorchBundle {
    pub fn builder() -> TorchBundleBuilder {
        TorchBundleBuilder::default()
    }
}


pub struct TorchBundleBuilder {
    intensity      : f32,
    intensity_delta: f32,
    light_range    : f32,
    light_radius   : f32,
    radius         : f32,
    x              : f32,
    y              : f32,
    z              : f32,
}

impl Default for TorchBundleBuilder {
    fn default() -> Self {
        Self {
            intensity      : TORCH_INTENSITY_BASE,
            intensity_delta: TORCH_INTENSITY_DELTA,
            light_range    : TORCH_LIGHT_RANGE,
            light_radius   : TORCH_LIGHT_RADIUS,
            radius         : TORCH_RADIUS,
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl TorchBundleBuilder {
    pub fn with_intensity(mut self, p_intensity: f32, p_delta: f32) -> Self {
        self.intensity       = p_intensity;
        self.intensity_delta = p_delta;
        self
    }

    pub fn with_light_range(mut self, p_range: f32, p_radius: f32) -> Self {
        self.light_range = p_range;
        self.light_radius = p_radius;
        self
    }

    pub fn with_radius(mut self, p_radius: f32) -> Self {
        self.radius = p_radius;
        self
    }

    pub fn move_to(mut self, p_x: f32, p_y: f32, p_z: f32) -> Self {
        self.x = p_x;
        self.y = p_y;
        self.z = p_z;
        self
    }

    pub fn build(
        self,
        p_meshes   : &mut ResMut<Assets<Mesh>>,
        p_materials: &mut ResMut<Assets<StandardMaterial>>
    ) -> TorchBundle {
        TorchBundle {
            marker   : Torch {
                base_intensity : self.intensity,
                intensity_delta: self.intensity_delta,
            },
            name     : Name::new("Torch"),
            mesh     : Mesh3d(p_meshes.add(Sphere::new(self.radius))),
            material : MeshMaterial3d(p_materials.add(StandardMaterial {
                base_color: Color::srgba_u8(255, 170, 0, 60),
                unlit: true,
                ..default()
            })),
            light    : PointLight {
                color          : Color::srgb_u8(255, 170, 0),
                intensity      : self.intensity,
                range          : self.light_range,
                radius         : self.light_radius,
                shadows_enabled: true,
                ..default()
            },
            /*body     : RigidBody::Static,
            collider : Collider::sphere(self.radius),*/
            transform: Transform::from_xyz(self.x, self.y, self.z),
        }
    }
}
