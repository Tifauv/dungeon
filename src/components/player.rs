use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


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
    transform         : Transform,
}

impl PlayerBundle {
    pub fn builder() -> PlayerBundleBuilder {
        PlayerBundleBuilder::default()
    }
}

pub struct PlayerBundleBuilder {
    camera_sensitivity: CameraSensitivity,
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
            transform         : Transform::from_xyz(self.x, self.y, self.z)
                                .looking_at(Vec3::new(self.look_at_x, self.look_at_y, self.look_at_z), Vec3::Y),
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
