use bevy::{prelude::Component, reflect::Reflect};
use bytemuck::{Pod, Zeroable};

#[derive(Default, Component, Debug, Reflect)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
    pub steerable: bool,
}

#[derive(Component)]
pub struct PlayerEntity {
    pub ego: bool,
    pub handle: usize,
}

#[derive(Component)]
pub struct AngularVelocity {
    pub angle: f32,
}

#[derive(Debug, Component)]
pub struct ThrustEngine {
    pub on: bool,
    pub force: f32,
}
#[derive(Component)]
pub struct MenuMainUI;

#[derive(Component)]
pub enum MenuMainBtn {
    OnlineMatch,
    LocalMatch,
    Quit,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Input {
    pub inp: u8,
}

#[derive(Component)]
pub struct RoundEntity;

#[derive(Default, Reflect, Component)]
pub struct CarControls {
    accel: f32,
    steer: f32,
}

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

#[derive(Component)]
pub struct EnnemyPlayer;
#[derive(Component)]
pub struct AllyPlayer;
#[derive(Component)]
pub struct EnnemyLaser;
#[derive(Component)]
pub struct AllyLaser;

#[derive(Component, PartialEq, Copy, Clone)]
pub enum LaserType {
    EnnemyLaser,
    AllyLaser,
}

#[derive(Component, PartialEq, Copy, Clone)]
pub enum PlayerType {
    EnnemyPlayer,
    AllyPlayer,
}
impl PlayerType {
    pub fn to_laser_type(self) -> LaserType {
        match self {
            PlayerType::EnnemyPlayer => LaserType::EnnemyLaser,
            PlayerType::AllyPlayer => LaserType::AllyLaser,
        }
    }
}
