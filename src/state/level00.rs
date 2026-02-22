use bevy::prelude::*;
use crate::components::base::*;
use crate::components::ground::*;
use crate::components::wall::*;
use crate::components::light::*;
use crate::components::player::*;


pub fn spawn_map(
    mut p_commands : Commands,
    mut p_meshes   : ResMut<Assets<Mesh>>,
    mut p_materials: ResMut<Assets<StandardMaterial>>
) {
    // Ground
    p_commands.spawn(GroundBundle::builder()
        .with_size(GROUND_SIZE, GROUND_SIZE)
        .move_to(GROUND_SIZE/2., GROUND_SIZE/2.)
        .build(&mut p_meshes, &mut p_materials));

    // Walls
    // North wall with out door
    p_commands.spawn(WallBundle::builder()
        .with_length(9.5)
        .move_to(WALL_THICKNESS, 0.)
        .build(&mut p_meshes, &mut p_materials));

    p_commands.spawn(WallBundle::builder()
        .with_length(1.)
        .with_height(0.5)
        .move_to(WALL_THICKNESS + 9.5, 0.)
        .move_to_y(2.)
        .build(&mut p_meshes, &mut p_materials));

    p_commands.spawn(WallBundle::builder()
        .with_length(7.5)
        .move_to(WALL_THICKNESS + 10.5, 0.)
        .build(&mut p_meshes, &mut p_materials));

    // South wall
    p_commands.spawn(WallBundle::builder()
        .with_length(GROUND_SIZE - 2.*WALL_THICKNESS)
        .move_to(WALL_THICKNESS, GROUND_SIZE - WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    // West wall
    p_commands.spawn(WallBundle::builder()
        .with_length(GROUND_SIZE)
        .rotate_90()
        .build(&mut p_meshes, &mut p_materials));

    // East wall
    p_commands.spawn(WallBundle::builder()
        .with_length(GROUND_SIZE)
        .rotate_90()
        .move_to(GROUND_SIZE - WALL_THICKNESS, 0.)
        .build(&mut p_meshes, &mut p_materials));

    // Inner walls
    p_commands.spawn(WallBundle::builder()
        .with_length(1.)
        .rotate_90()
        .move_to(1., 0.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(0., 1.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(11., 0.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(14., 1.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(10.)
        .move_to(5., 2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .rotate_90()
        .move_to(4., 2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .rotate_90()
        .move_to(5., 3.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(2., 4.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .move_to(2., 5.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(10.)
        .rotate_90()
        .move_to(15., 2.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(13., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .rotate_90()
        .move_to(13., 12.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(14., 15.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(15., 13.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .move_to(9., 5.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .rotate_90()
        .move_to(8., 5.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(9.)
        .move_to(4., 8.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(9.)
        .rotate_90()
        .move_to(10., 9.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(4., 9.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .move_to(3., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(3.)
        .rotate_90()
        .move_to(2., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .rotate_90()
        .move_to(2., 15.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(6.)
        .rotate_90()
        .move_to(7., 11.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
    p_commands.spawn(WallBundle::builder()
        .with_length(4.)
        .move_to(3., 16.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));

    p_commands.spawn(WallBundle::builder()
        .with_length(2.)
        .move_to(0., 8.)
        .with_xz_offset(WALL_THICKNESS)
        .build(&mut p_meshes, &mut p_materials));
}


pub fn spawn_player(mut p_commands: Commands, mut p_meshes: ResMut<Assets<Mesh>>, mut p_materials: ResMut<Assets<StandardMaterial>>) {
    let player = p_commands
        // Player
        .spawn(PlayerBundle::builder()
            .move_to(11., 0.01, 8.)
            .look_at(12., 0.0 , 8.)
            .build()
        )
        .with_children(|parent| {
            // Body
            parent.spawn(PlayerBodyBundle::builder()
                .with_radius(0.25)
                .with_height(1.8)
                .build(&mut p_meshes, &mut p_materials)
            );
        })
        .id();

    // Torch light
    p_commands
        .spawn((
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
            MeshMaterial3d(p_materials.add(Color::srgba_u8(255, 170, 0, 240))),
            Transform::from_xyz(0.25, 0.5, -0.20),
        ))
        .insert(ChildOf(player));

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


pub fn spawn_global_light(mut p_commands: Commands) {
    p_commands.spawn((
        DirectionalLight{
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(9., 5., 11.).looking_at(Vec3::new(10., 1.5, 10.), Vec3::Y),
    ));
}


pub fn spawn_axis(mut p_commands: Commands, mut p_meshes: ResMut<Assets<Mesh>>, mut p_materials: ResMut<Assets<StandardMaterial>>) {
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
