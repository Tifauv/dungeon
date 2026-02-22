use bevy::prelude::*;
use avian3d::prelude::*;


pub const GROUND_SIZE: f32 = 20.0;


#[derive(Component)]
pub struct Ground;


#[derive(Bundle)]
pub struct GroundBundle {
    marker   : Ground,
    name     : Name,
    mesh     : Mesh3d,
    material : MeshMaterial3d<StandardMaterial>,
    body     : RigidBody,
    collider : Collider,
    transform: Transform,
}

impl GroundBundle {
    pub fn builder() -> GroundBundleBuilder {
        GroundBundleBuilder::default()
    }
}


pub struct GroundBundleBuilder {
    x_size: f32,
    z_size: f32,
    x: f32,
    z: f32,
}

impl Default for GroundBundleBuilder {
    fn default() -> Self {
        Self {
            x_size: GROUND_SIZE,
            z_size: GROUND_SIZE,
            x: GROUND_SIZE/2.,
            z: GROUND_SIZE/2.,
        }
    }
}

impl GroundBundleBuilder {
    pub fn with_size(mut self, p_x_size: f32, p_z_size: f32) -> Self {
        self.x_size = p_x_size;
        self.z_size = p_z_size;
        self
    }

    pub fn move_to(mut self, p_x: f32, p_z: f32) -> Self {
        self.x = p_x;
        self.z = p_z;
        self
    }

    pub fn build(
        self,
        p_meshes   : &mut ResMut<Assets<Mesh>>,
        p_materials: &mut ResMut<Assets<StandardMaterial>>
    ) -> GroundBundle {
        let thickness: f32 = 0.1;

        GroundBundle {
            marker   : Ground,
            name     : Name::new("Ground"),
            mesh     : Mesh3d(p_meshes.add(Plane3d::new(Vec3::Y, Vec2::new(self.x_size, self.z_size)))),
            material : MeshMaterial3d(p_materials.add(Color::srgb(0.6, 0.3, 0.0))),
            body     : RigidBody::Static,
            collider : Collider::cuboid(self.x_size * 2., thickness, self.z_size * 2.),
            transform: Transform::from_xyz(self.x, -thickness/2., self.z),
        }
    }
}
