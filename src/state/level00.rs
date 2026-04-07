use bevy::prelude::*;
use avian3d::prelude::*;

use crate::components::base::*;
use crate::components::player::*;
use crate::objects::door::components::*;
use crate::objects::ground::components::*;
use crate::objects::torch::components::*;
use crate::objects::wall::components::*;


pub fn setup(
    mut p_commands : Commands,
    mut p_meshes   : ResMut<Assets<Mesh>>,
    mut p_materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_map(&mut p_commands, &mut p_meshes, &mut p_materials);
    spawn_player(&mut p_commands, &mut p_meshes, &mut p_materials);
}


pub fn setup_debug(
    mut p_commands : Commands,
    mut p_meshes   : ResMut<Assets<Mesh>>,
    mut p_materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_axis(&mut p_commands, &mut p_meshes, &mut p_materials);
    spawn_global_light(&mut p_commands);
}


fn spawn_map(
    p_commands : &mut Commands,
    mut p_meshes   : &mut ResMut<Assets<Mesh>>,
    mut p_materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    p_commands.spawn(GroundBundle::builder()
        .with_size(GROUND_SIZE, GROUND_SIZE)
        .move_to(GROUND_SIZE/2., GROUND_SIZE/2.)
        .build(&mut p_meshes, &mut p_materials));

    // Walls
    // North wall with out door
    // W01
    p_commands.spawn(WallBundle::builder()
        .with_length(9.5)
        .move_to(WALL_THICKNESS, 0.)
        .build(&mut p_meshes, &mut p_materials));
    // W02
    p_commands.spawn(WallBundle::builder()
        .with_length(1.)
        .with_height(0.5)
        .move_to(WALL_THICKNESS + 9.5, 0.)
        .move_to_y(2.)
        .build(&mut p_meshes, &mut p_materials));
    // D01
    p_commands.spawn(DoorBundle::builder()
        .move_to(WALL_THICKNESS + 9.5, 0.25)
        .build(&mut p_meshes, &mut p_materials));
    // W03
    p_commands.spawn(WallBundle::builder()
        .with_length(7.5)
        .move_to(WALL_THICKNESS + 10.5, 0.)
        .build(&mut p_meshes, &mut p_materials));

    // W04 - South wall
    p_commands.spawn(WallBundle::builder()
        .with_length(GROUND_SIZE - 2.*WALL_THICKNESS)
        .move_to(WALL_THICKNESS, GROUND_SIZE - WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    // W05 - West wall
    p_commands.spawn(WallBundle::builder()
        .with_length(GROUND_SIZE)
        .rotate_90()
        .build(&mut p_meshes, &mut p_materials));

    // W06 - East wall
    p_commands.spawn(WallBundle::builder()
        .with_length(GROUND_SIZE)
        .rotate_90()
        .move_to(GROUND_SIZE - WALL_THICKNESS, 0.)
        .build(&mut p_meshes, &mut p_materials));

    // Inner walls
    // W07
    p_commands.spawn(WallBundle::builder()
        .with_length(1.)
        .rotate_90()
        .move_to(1., 0.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W08
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(0., 1.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    // W09
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(11., 0.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W10
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(14., 1.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W11
    p_commands.spawn(WallBundle::builder()
        .with_length(10.)
        .move_to(5., 2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W12
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .rotate_90()
        .move_to(4., 2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W13
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .rotate_90()
        .move_to(5., 3.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W14
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(2., 4.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W15
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .move_to(2., 5.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W16
    p_commands.spawn(WallBundle::builder()
        .with_length(10.)
        .rotate_90()
        .move_to(15., 2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W17
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(13., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W18
    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .rotate_90()
        .move_to(13., 12.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W19
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(14., 15.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W20
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(15., 13.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // D02
    p_commands.spawn(DoorBundle::builder()
        .with_height(2.5)
        .with_thickness(1.0)
        .rotate_90()
        .move_to(15., 12.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    // W21
    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .move_to(9., 5.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W22
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .rotate_90()
        .move_to(8., 5.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W23
    p_commands.spawn(WallBundle::builder()
        .with_length(9.)
        .move_to(4., 8.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W24
    p_commands.spawn(WallBundle::builder()
        .with_length(9.)
        .rotate_90()
        .move_to(10., 9.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W25
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(4., 9.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W26
    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .move_to(3., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W27
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(2., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W28
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .with_height(0.5)
        .rotate_90()
        .move_to(2., 13.)
        .move_to_y(2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // D03
    p_commands.spawn(DoorBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(2.25, 13.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W29
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(2., 15.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W30
    p_commands.spawn(WallBundle::builder()
        .with_length(6.)
        .rotate_90()
        .move_to(7., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W31
    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .move_to(3., 16.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    // W32
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(0., 8.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    // W33
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .with_height(0.5)
        .move_to(2., 8.)
        .move_to_y(2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
}


fn spawn_player(
    p_commands : &mut Commands,
    mut p_meshes   : &mut ResMut<Assets<Mesh>>,
    mut p_materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let player = p_commands
        // Player
        .spawn(PlayerBundle::builder()
            .with_collider(Collider::capsule(0.25, 0.65))
            .move_to(11., 0.01, 8.)
            .look_at(12., 0.0 , 8.)
            .build()
        )
        .with_children(|parent| {
            // Torch
            parent.spawn(TorchBundle::builder()
                .move_to(0.25, 1., -0.2)
                .build(&mut p_meshes, &mut p_materials)
            );
            // Body
            parent.spawn(PlayerBodyBundle::builder()
                .with_radius(0.25)
                .with_height(1.8)
                .build(&mut p_meshes, &mut p_materials)
            );
        })
        .id();

    // Camera for first person view
    let camera = p_commands
        .spawn((
            Camera3d::default(),
            Name::new("Player camera"),
            Projection::from(PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }),
            Transform::from_xyz(0., 0.775, 0.),
            Camera {
                order: 0 as isize,
                ..default()
            },
            CameraView {
                pos: UVec2::new(0, 0),
                size: Vec2::new(0.75, 1.),
            },
        ))
        .insert(ChildOf(player))
        .id();

    // Top Camera
    let top_camera = p_commands.spawn((
        Camera3d::default(),
        Name::new("Top camera"),
        AbovePlayer::new(player, 15.),
        Transform::IDENTITY,
        Camera {
            order: 1 as isize,
            ..default()
        },
        CameraView {
            pos: UVec2::new(3, 0),
            size: Vec2::new(0.25, 1.),
        },
    )).id();

    // Setup UI
    p_commands.spawn((
        UiTargetCamera(camera),
        Node {
            width:  percent(100),
            height: percent(100),
            ..default()
        },
    ));
    p_commands.spawn((
        UiTargetCamera(top_camera),
        Node {
            width:  percent(50),
            height: percent(50),
            ..default()
        },
        children![
            (
                Text::new("Top view"),
                Node {
                    position_type: PositionType::Absolute,
                    top:  px(12),
                    left: px(12),
                ..default()
                },
            ),
        ],
    ));
}


fn spawn_axis(
    p_commands : &mut Commands,
    p_meshes   : &mut ResMut<Assets<Mesh>>,
    p_materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let axis_length = 5.0;
    let axis_radius = 0.1;

    // X axis
    p_commands.spawn((
        Mesh3d(p_meshes.add(Cuboid::new(axis_length, axis_radius, axis_radius))),
        MeshMaterial3d(p_materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(axis_length/2., axis_radius/2., axis_radius/2.),
    ));

    // Y axis
    p_commands.spawn((
        Mesh3d(p_meshes.add(Cuboid::new(axis_radius, axis_length, axis_radius))),
        MeshMaterial3d(p_materials.add(Color::srgb_u8(0, 255, 0))),
        Transform::from_xyz(axis_radius/2., axis_length/2., axis_radius/2.),
    ));

    // Z axis
    p_commands.spawn((
        Mesh3d(p_meshes.add(Cuboid::new(axis_radius, axis_radius, axis_length))),
        MeshMaterial3d(p_materials.add(Color::srgb_u8(0, 0, 255))),
        Transform::from_xyz(axis_radius/2., axis_radius/2., axis_length/2.),
    ));
}


fn spawn_global_light(p_commands: &mut Commands) {
    p_commands.spawn((
        DirectionalLight{
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(9., 5., 11.).looking_at(Vec3::new(10., 1.5, 10.), Vec3::Y),
    ));
}
