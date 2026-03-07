use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;
use avian3d::math::Vector;

use crate::components::character_controller::*;
use crate::components::input_actions;


#[derive(Component)]
pub struct Player;


#[derive(Bundle)]
pub struct PlayerBundle;

impl PlayerBundle {
    pub fn builder() -> PlayerBundleBuilder {
        PlayerBundleBuilder::default()
    }
}

pub struct PlayerBundleBuilder {
    camera_sensitivity: CameraSensitivity,
    collider          : Collider,
    gravity           : Vector,
    x                 : f32,
    y                 : f32,
    z                 : f32,
    look_at_x         : f32,
    look_at_y         : f32,
    look_at_z         : f32,
}

impl Default for PlayerBundleBuilder {
    fn default() -> Self {
        Self {
            camera_sensitivity: CameraSensitivity::default(),
            collider          : Collider::cuboid(1.0, 1.0, 1.0),
            gravity           : ControllerGravity::default_vector(),
            x                 : 0.,
            y                 : 0.,
            z                 : 0.,
            look_at_x         : 0.,
            look_at_y         : 0.,
            look_at_z         : 0.,
        }
    }
}

impl PlayerBundleBuilder {
    pub fn with_camera_sensitivity(mut self, p_camera_sensitivity: CameraSensitivity) -> Self {
        self.camera_sensitivity = p_camera_sensitivity;
        self
    }

    pub fn with_collider(mut self, p_collider: Collider) -> Self {
        self.collider = p_collider;
        self
    }

    pub fn with_gravity(mut self, p_gravity: Vector) -> Self {
        self.gravity = p_gravity;
        self
    }

    pub fn move_to(mut self, p_x: f32, p_y: f32, p_z: f32) -> Self {
        self.x = p_x;
        self.y = p_y;
        self.z = p_z;
        self
    }

    pub fn look_at(mut self, p_x: f32, p_y: f32, p_z: f32) -> Self {
        self.look_at_x = p_x;
        self.look_at_y = p_y;
        self.look_at_z = p_z;
        self
    }

    pub fn build(self) -> impl Bundle {
        (
            Player,
            Name::new("Player"),
            actions!(Player[
                (
                    Action::<input_actions::Move>::new(),
                    DeadZone::default(),
                    SmoothNudge::default(),
                    DeltaScale::default(),
                    Scale::splat(1.0),
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Axial::left_stick(),
                    )),
                ),
                (
                    Action::<input_actions::LookAround>::new(),
                    DeltaScale::default(),
                    Bindings::spawn((
                        Spawn((Binding::mouse_motion(), Negate::all())),
                        Axial::right_stick().with((Scale::splat(100.0), Negate::x())),
                    )),
                ),
                (
                    Action::<input_actions::CaptureCursor>::new(),
                    bindings![MouseButton::Left],
                ),
                (
                    Action::<input_actions::ReleaseCursor>::new(),
                    bindings![KeyCode::Escape]
                ),
            ]),
            self.camera_sensitivity,
            CharacterControllerBundle::new(
                self.collider,
                self.gravity,
            ),
            CollisionEventsEnabled,
            Visibility::default(),
            Transform::from_xyz(self.x, self.y, self.z)
                .looking_at(Vec3::new(self.look_at_x, self.look_at_y, self.look_at_z), Vec3::Y),
        )
    }
}


#[derive(Component)]
pub struct PlayerBody;

#[derive(Bundle)]
pub struct PlayerBodyBundle {
    marker   : PlayerBody,
    name     : Name,
    mesh     : Mesh3d,
    material : MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

impl PlayerBodyBundle {
    pub fn builder() -> PlayerBodyBundleBuilder {
        PlayerBodyBundleBuilder::default()
    }
}

pub struct PlayerBodyBundleBuilder {
    height: f32,
    radius: f32,
    x     : f32,
    y     : f32,
    z     : f32,
}

impl Default for PlayerBodyBundleBuilder {
    fn default() -> Self {
        Self {
            height: 0.7,
            radius: 0.2,
            x     : 0.,
            y     : 0.,
            z     : 0.,
        }
    }
}

impl PlayerBodyBundleBuilder {
    pub fn with_height(mut self, p_full_height: f32) -> Self {
        self.height = p_full_height;
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
    ) -> PlayerBodyBundle {
        PlayerBodyBundle {
            marker   : PlayerBody,
            name     : Name::new("Body"),
            mesh     : Mesh3d(p_meshes.add(Capsule3d::new(self.radius, self.height / 2. - self.radius))),
            material : MeshMaterial3d(p_materials.add(Color::srgb_u8(255, 0, 0))),
            transform: Transform::from_xyz(self.x, self.y, self.z),
        }
    }
}


#[derive(Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            Vec2::new(2.0, 1.2),
        )
    }
}


#[derive(Component)]
#[relationship(relationship_target = PlayerFollowers)]
pub struct AbovePlayer {
    #[relationship]
    pub parent: Entity,
    pub altitude: f32,
}

impl AbovePlayer {
    pub fn new(p_parent: Entity, p_altitude: f32) -> AbovePlayer {
        AbovePlayer {
            parent: p_parent,
            altitude: p_altitude,
        }
    }
}

#[derive(Component)]
#[relationship_target(relationship = AbovePlayer, linked_spawn)]
pub struct PlayerFollowers(Vec<Entity>);
