use bevy::prelude::*;
use avian3d::prelude::*;
use avian3d::math::Vector;
use leafwing_input_manager::prelude::*;

use crate::components::character_controller::*;


#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UserAction {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    LookAround,
}


#[derive(Component)]
pub struct Player;

impl Player {
    pub fn default_input_map() -> InputMap<UserAction> {
        InputMap::default()
            .with_dual_axis(
                UserAction::Move,
                VirtualDPad::wasd()
                    .with_circle_deadzone(0.1)
                    .inverted_y()
                    .reset_processing_pipeline(),
            )
            .with_dual_axis(
                UserAction::Move,
                GamepadStick::LEFT
                    .with_circle_deadzone(0.1)
            )
            .with_dual_axis(
                UserAction::LookAround,
                MouseMove::default().replace_processing_pipeline([
                    CircleDeadZone::new(0.1).into(),
                    DualAxisSensitivity::all(2.0).into(),
                ]),
            )
            .with_dual_axis(
                UserAction::LookAround,
                GamepadStick::RIGHT
                    .with_circle_deadzone(0.1)
                    .inverted_y()
            )
    }
}


#[derive(Bundle)]
pub struct PlayerBundle {
    marker            : Player,
    name              : Name,
    input_map         : InputMap<UserAction>,
    camera_sensitivity: CameraSensitivity,
    controller        : CharacterControllerBundle,
    collision_events  : CollisionEventsEnabled,
    visibility        : Visibility,
    transform         : Transform,
}

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

    pub fn build(self) -> PlayerBundle {
        PlayerBundle {
            marker            : Player,
            name              : Name::new("Player"),
            input_map         : Player::default_input_map(),
            camera_sensitivity: self.camera_sensitivity,
            controller        : CharacterControllerBundle::new(
                self.collider,
                self.gravity,
            ),
            collision_events  : CollisionEventsEnabled,
            visibility        : Visibility::default(),
            transform         : Transform::from_xyz(self.x, self.y, self.z)
                                .looking_at(Vec3::new(self.look_at_x, self.look_at_y, self.look_at_z), Vec3::Y),
        }
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
