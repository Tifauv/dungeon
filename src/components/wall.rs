use bevy::prelude::*;


#[derive(Component)]
pub struct Wall;/* {
    width: f32,
    height: f32,
    thickness: f32,
};

impl Wall {
    pub fn new(p_width: f32, p_height: f32, p_thickness: f32) -> Self {
        Wall {
            width:     p_width,
            height:    p_height,
            thickness: p_thickness,
        }
    }


    fn create_mesh(&self, mut p_meshes: ResMut<Assets<Mesh>>) -> Mesh3d {
        Mesh3d(p_meshes.add(Cuboid::new(self.width, self.height, self.thickness)))
    }


    fn create_material(&self, mut p_materials: ResMut<Assets<StandardMaterial>>) -> MeshMaterial3d<M> {
        MeshMaterial3d(p_materials.add(Color::srgb_u8(255, 252, 167)))
    }


    pub fn create_components(&self, mut p_meshes: ResMut<Assets<Mesh>>, mut p_materials: ResMut<Assets<StandardMaterial>>) -> (Wall, Mesh3d, MeshMaterial3d) {
        (
            *self,
            self.create_mesh(p_meshes),
            self.create_material(p_materials),
        )
    }
}*/
