use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;


pub const WALL_LENGTH: f32    = 2.0;
pub const WALL_HEIGHT: f32    = 2.5;
pub const WALL_THICKNESS: f32 = 1.0;


#[derive(Component)]
pub struct Wall;


#[derive(Bundle)]
pub struct WallBundle {
    marker   : Wall,
    mesh     : Mesh3d,
    material : MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}


impl WallBundle {
    pub fn builder() -> WallBundleBuilder {
        WallBundleBuilder::default()
    }
}


#[derive(Default)]
pub struct WallBundleBuilder {
    length    : f32,
    height    : f32,
    thickness : f32,
    rotate: bool,
    x: f32,
    y: f32,
    z: f32,
    x_offset: f32,
    y_offset: f32,
    z_offset: f32,
}

impl WallBundleBuilder {
    pub fn new() -> Self {
        Self {
            length:    WALL_LENGTH,
            height:    WALL_HEIGHT,
            thickness: WALL_THICKNESS,
            rotate: false,
            x: 0.,
            y: 0.,
            z: 0.,
            x_offset: 0.,
            y_offset: 0.,
            z_offset: 0.,
        }
    }

    pub fn with_length(mut self, p_length: f32) -> Self {
        self.length = p_length;
        self
    }

    pub fn with_height(mut self, p_height: f32) -> Self {
        self.height = p_height;
        self
    }

    pub fn with_thickness(mut self, p_thickness: f32) -> Self {
        self.thickness = p_thickness;
        self
    }

    pub fn rotate_90(mut self) -> Self {
        self.rotate = true;
        self
    }

    pub fn move_to(mut self, p_x: f32, p_z: f32) -> Self {
        self.x = p_x;
        self.z = p_z;
        self
    }

    pub fn move_to_y(mut self, p_y: f32) -> Self {
        self.y = p_y;
        self
    }

    pub fn with_x_offset(mut self, p_offset: f32) -> Self {
        self.x_offset = p_offset;
        self
    }

    pub fn with_y_offset(mut self, p_offset: f32) -> Self {
        self.y_offset = p_offset;
        self
    }

    pub fn with_z_offset(mut self, p_offset: f32) -> Self {
        self.z_offset = p_offset;
        self
    }

    pub fn with_xz_offset(mut self, p_offset: f32) -> Self {
        self.x_offset = p_offset;
        self.z_offset = p_offset;
        self
    }

    pub fn build(
        self,
        p_meshes: &mut ResMut<Assets<Mesh>>,
        p_materials: &mut ResMut<Assets<StandardMaterial>>
    ) -> WallBundle {
        let transform = if !self.rotate {
            Transform::from_xyz(
                self.x_offset + self.x + self.length/2.,
                self.y_offset + self.y + self.height/2.,
                self.z_offset + self.z + self.thickness/2.
            )
        }
        else {
            Transform::from_xyz(
                self.x_offset + self.x + self.thickness/2.,
                self.y_offset + self.y + self.height/2.,
                self.z_offset + self.z + self.length/2.
            ).with_rotation(Quat::from_rotation_y(FRAC_PI_2))
        };

        WallBundle {
            marker: Wall,
            mesh: Mesh3d(p_meshes.add(Cuboid::new(
                self.length,
                self.height,
                self.thickness))),
            material: MeshMaterial3d(p_materials.add(Color::srgb_u8(255, 252, 167))),
            transform: transform,
        }
    }
}
