use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

use crate::{
    checksum::Checksum,
    menu::{connect::LocalHandles, win::MatchData},
    AppState, GGRSConfig, NUM_PLAYERS,
};

const INPUT_UP: u8 = 0b0001;
const INPUT_DOWN: u8 = 0b0010;
const INPUT_LEFT: u8 = 0b0100;
const INPUT_RIGHT: u8 = 0b1000;

// const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
// const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
// const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
// const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);
// const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];

// const PLAYER_SIZE: f32 = 50.;
// const MOV_SPEED: f32 = 0.1;
// const ROT_SPEED: f32 = 0.05;
// const MAX_SPEED: f32 = 7.5;
// const FRICTION: f32 = 0.98;
// const DRIFT: f32 = 0.95;
// const ARENA_SIZE: f32 = 720.0;
// const CUBE_SIZE: f32 = 0.2;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Input {
    pub inp: u8,
}

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component)]
pub struct RoundEntity;

#[derive(Default, Reflect, Component)]
pub struct Velocity(pub Vec2);

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

pub fn input(
    handle: In<PlayerHandle>,
    keyboard_input: Res<bevy::input::Input<KeyCode>>,
    local_handles: Res<LocalHandles>,
) -> Input {
    let mut inp: u8 = 0;
    if keyboard_input.pressed(KeyCode::Up) {inp |= INPUT_UP}
    if keyboard_input.pressed(KeyCode::Left) {inp |= INPUT_LEFT}
    if keyboard_input.pressed(KeyCode::Down) {inp |= INPUT_DOWN}
    if keyboard_input.pressed(KeyCode::Right) {inp |= INPUT_RIGHT}

    Input { inp }
}