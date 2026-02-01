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
/*{
    pub part:  WallPart,
    pub depth: u8
}

impl Wall {
    fn new(part: WallPart, depth: u8) -> Wall {
        Wall {
            part,
            depth
        }
    }
}
*/
/*
fn create(mut commands: Commands, part: WallPart, depth: u8, trans_x: f32, trans_y: f32, trans_z: f32, sprite_sheet_handle: Handle<SpriteSheet>, sprite_index: usize) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(trans_x, trans_y, trans_z);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_index
    };

    commands.spawn((
        Wall::new(part, depth),
        transform,
        sprite_render,
        Hidden {}
    ));
}
*/
