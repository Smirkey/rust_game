use bevy::{
    prelude::{Component, Timer, Vec2, Vec3},
    reflect::Reflect,
};
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
    pub team: bool,
    pub size: Vec2,
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

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

#[derive(Component)]
pub struct Laser {
    pub player_handle: usize,
    pub player_team: bool,
    pub size: Vec2,
}

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn {
    pub translation: Vec3
}

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, true))
    }
}
