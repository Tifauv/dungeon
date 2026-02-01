use crate::components::base::*;
use crate::components::wall::*;
use crate::components::light::*;
use crate::components::player::*;
use std::f32::consts::PI;
use bevy::prelude::*;

const GROUND_SIZE: f32    = 10.0;

const WALL_WIDTH: f32     = 1.0;
const WALL_HEIGHT: f32    = 2.5;


pub fn spawn_map(
    mut p_commands : Commands,
    mut p_meshes   : ResMut<Assets<Mesh>>,
    mut p_materials: ResMut<Assets<StandardMaterial>>
) {
    // Ground
    p_commands.spawn((
        Ground,
        Mesh3d(p_meshes.add(Plane3d::default().mesh().size(GROUND_SIZE, GROUND_SIZE))),
        MeshMaterial3d(p_materials.add(Color::srgb(0.6, 0.3, 0.0))),
        Transform::from_xyz(GROUND_SIZE/2., 0., GROUND_SIZE/2.),
    ));

    // Walls
    let wall_mesh     = p_meshes.add(Cuboid::new(WALL_WIDTH, WALL_HEIGHT, WALL_WIDTH));
    let wall_material = p_materials.add(Color::srgb_u8(255, 252, 167));

    // North & South outer walls
    for x in 0..10 {
        // North wall
        if x == 5 {
            p_commands.spawn((
                Wall,
                Name::new("Wall"),
                Mesh3d(p_meshes.add(Cuboid::new(WALL_WIDTH, 0.5, WALL_WIDTH))),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(x as f32 + WALL_WIDTH/2., WALL_HEIGHT-0.25, WALL_WIDTH/2.),
            ));
        }
        else {
            p_commands.spawn((
                Wall,
                Name::new("Wall"),
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(x as f32 + WALL_WIDTH/2., WALL_HEIGHT/2., WALL_WIDTH/2.),
            ));
        }

        // South wall
        p_commands.spawn((
            Wall,
            Name::new("Wall"),
            Mesh3d(wall_mesh.clone()),
            MeshMaterial3d(wall_material.clone()),
            Transform::from_xyz(x as f32 + WALL_WIDTH/2., WALL_HEIGHT/2., 9. + WALL_WIDTH/2.),
        ));
    }

    // West and East outer walls
    for z in 1..9 {
        // West wall
        p_commands.spawn((
            Wall,
            Name::new("Wall"),
            Mesh3d(wall_mesh.clone()),
            MeshMaterial3d(wall_material.clone()),
            Transform::from_xyz(WALL_WIDTH/2., WALL_HEIGHT/2., z as f32 + WALL_WIDTH/2.),
        ));

        // East wall
        p_commands.spawn((
            Wall,
            Name::new("Wall"),
            Mesh3d(wall_mesh.clone()),
            MeshMaterial3d(wall_material.clone()),
            Transform::from_xyz(9. + WALL_WIDTH/2., WALL_HEIGHT/2., z as f32 + WALL_WIDTH/2.),
        ));
    }

    // Inner walls
    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(1. + WALL_WIDTH/2., WALL_HEIGHT/2., 1. + WALL_WIDTH/2.),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(6. + WALL_WIDTH/2., WALL_HEIGHT/2., 1. + WALL_WIDTH/2.),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(p_meshes.add(Cuboid::new(5., 2.5, 1.))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(5.5, WALL_HEIGHT/2., 2.5),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(2. + WALL_WIDTH/2., WALL_HEIGHT/2., 3. + WALL_WIDTH/2.),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(p_meshes.add(Cuboid::new(3., 2.5, 1.))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(7.5, WALL_HEIGHT/2., 4.5).with_rotation(Quat::from_rotation_y(PI / 2.)),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(p_meshes.add(Cuboid::new(2., 2.5, 1.))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(5., WALL_HEIGHT/2., 4.5),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(1.5, WALL_HEIGHT/2., 5.5),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(p_meshes.add(Cuboid::new(2., 2.5, 1.))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(3.5, WALL_HEIGHT/2., 6.).with_rotation(Quat::from_rotation_y(PI / 2.)),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(p_meshes.add(Cuboid::new(2., 2.5, 1.))),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(4.5, WALL_HEIGHT/2., 7.).with_rotation(Quat::from_rotation_y(PI / 2.)),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(6.5, WALL_HEIGHT/2., 6.5),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(2.5, WALL_HEIGHT/2., 7.5),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(7.5, WALL_HEIGHT/2., 7.5),
    ));

    p_commands.spawn((
        Wall,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(5.5, WALL_HEIGHT/2., 8.5),
    ));
}


pub fn spawn_player(mut p_commands: Commands, mut p_meshes: ResMut<Assets<Mesh>>, mut p_materials: ResMut<Assets<StandardMaterial>>) {
    // Torch light
    p_commands.spawn((
        Torch {
            base_intensity: 2800.,
            intensity_variation: 1400.,
        },
        PointLight {
            color: Color::srgb_u8(255, 170, 0),
            intensity: 2800.,
            range: 10.,
            radius: 0.1,
            shadows_enabled: true,
            ..default()
        },
        Mesh3d(p_meshes.add(Sphere::new(0.05))),
        MeshMaterial3d(p_materials.add(Color::srgba_u8(255, 170, 0, 40))),
        Transform::from_xyz(4.5, 2.0, 5.75),
    ));

    // Camera
    let camera = p_commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_xyz(4.5, 1.5, 5.5).looking_at(Vec3::new(5.5, 1.5, 5.5), Vec3::Y),
        Camera {
            order: 0 as isize,
            ..default()
        },
        CameraPosition{
            pos: UVec2::new(0, 0),
        },
    )).id();

    // Setup UI
    p_commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![
            (
                Text::new("Player"),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(12),
                    left: px(12),
                    ..default()
                },
            ),
        ],
    ));
}


pub fn spawn_top_view(mut p_commands: Commands) {
    // Global light (debug)
    /*
    p_commands.spawn((
        DirectionalLight{
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.5, 5., 5.5).looking_at(Vec3::new(5., 1.5, 5.), Vec3::Y),
    ));
    */

    // Top Camera
    let camera = p_commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_xyz(4.5, 15., 5.5).looking_at(Vec3::new(4.5, 1.5, 5.5), Vec3::Y),
        Camera {
            order: 1 as isize,
            ..default()
        },
        CameraPosition{
            pos: UVec2::new(1, 0),
        },
    )).id();

    // Setup UI
    p_commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![
            (
                Text::new("Top view"),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(12),
                    left: px(12),
                    ..default()
                },
            ),
        ],
    ));
}


pub fn spawn_axis(mut p_commands: Commands, mut p_meshes: ResMut<Assets<Mesh>>, mut p_materials: ResMut<Assets<StandardMaterial>>) {
    // Y axis
    p_commands.spawn((
        Mesh3d(p_meshes.add(Cuboid::new(0.1, 5., 0.1))),
                      MeshMaterial3d(p_materials.add(Color::srgb_u8(255, 0, 0))),
                      Transform::from_xyz(0.05, 2.5, 0.05),
    ));
}
