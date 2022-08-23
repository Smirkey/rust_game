use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

use crate::{
    checksum::Checksum,
    menu::{connect::LocalHandles, win::MatchData},
    AppState, GGRSConfig, NUM_PLAYERS, PLAYER_SCALE, components::{ThrustEngine, AngularVelocity, Movable},
};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{geom::Rotation, math::Angle},
        *,
    },
    shapes::Polygon,
};
pub(crate) const INPUT_UP: u8 = 0b0001;
pub(crate) const INPUT_DOWN: u8 = 0b0010;
pub(crate) const INPUT_LEFT: u8 = 0b0100;
pub(crate) const INPUT_RIGHT: u8 = 0b1000;
const ARENA_SIZE: f32 = 720.0;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
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

pub fn setup_round(mut commands: Commands, mut windows: ResMut<Windows>,) {
    commands.insert_resource(FrameCount::default());
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(RoundEntity);
    
    let window = windows.get_primary_mut().unwrap();

    let win_size = WinSize {
        w: window.width(),
        h: window.height(),
    };
    commands.insert_resource(win_size);
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(ARENA_SIZE, ARENA_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RoundEntity);
}


pub fn spawn_players(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>, win_size: Res<WinSize>,) {
    let r = ARENA_SIZE / 4.;

    for handle in 0..NUM_PLAYERS {
        commands
        .spawn_bundle(GeometryBuilder::build_as(
            &{
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(Vec2::ZERO);
                path_builder.line_to(Vec2::new(-8.0, -8.0));
                path_builder.line_to(Vec2::new(0.0, 12.0));
                path_builder.line_to(Vec2::new(8.0, -8.0));
                path_builder.line_to(Vec2::ZERO);
                let mut line = path_builder.build();
                line.0 = line.0.transformed(&Rotation::new(Angle::degrees(-90.0)));
                line.0
            },
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.0)),
            Transform {
                translation: Vec3::new(0., -win_size.h / 2., 10.),
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
                ..Default::default()
            },
        ))
            .insert(Player { handle })
            .insert(Velocity::default())
            .insert(Movable {
                auto_despawn: false,
                steerable: true,
            })
            .insert(AngularVelocity { angle: 0. })
            .insert(ThrustEngine { on: false, force: 0.001 })
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(RoundEntity);
    }
}

pub fn print_p2p_events(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
    }
}

pub fn check_win(mut state: ResMut<State<AppState>>, mut commands: Commands) {
    let condition = false;
    let confirmed = false;

    if condition && confirmed {
        state.set(AppState::Win).expect("Could not change state.");
        commands.insert_resource(MatchData {
            result: "Orange won!".to_owned(),
        });
    }
}

pub fn cleanup(query: Query<Entity, With<RoundEntity>>, mut commands: Commands) {
    commands.remove_resource::<FrameCount>();
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<P2PSession<GGRSConfig>>();
    commands.remove_resource::<SessionType>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
