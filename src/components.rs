use bevy::{prelude::Component, reflect::Reflect};
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
pub struct Player;

#[derive(Component)]
pub struct AngularVelocity {
    pub angle: f32,
}

#[derive(Debug, Component)]
pub struct ThrustEngine {
    pub on: bool,
    pub force: f32
}
#[derive(Component)]
pub struct MenuMainUI;

#[derive(Component)]
pub enum MenuMainBtn {
    OnlineMatch,
    LocalMatch,
    Quit,
}