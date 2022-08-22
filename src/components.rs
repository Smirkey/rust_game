use bevy::prelude::Component;
use ggrs::{Config, Frame, GGRSRequest, GameStateCell, InputStatus, PlayerHandle, NULL_FRAME};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use bytemuck::{Pod, Zeroable};


#[derive(Component, Debug)]
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

pub struct Input {
    pub inp: u8,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub frame: i32,
    pub num_players: usize,
    pub positions: Vec<(f32, f32)>,
    pub velocities: Vec<(f32, f32)>,
    pub rotations: Vec<f32>,
}
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = Input;
    type State = State;
    type Address = SocketAddr;
}

