use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UserAction {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    LookAround,
}


#[derive(Debug, Component)]
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


#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            Vec2::new(0.03, 0.02),
        )
    }
}
