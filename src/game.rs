use bevy::{
    math::Vec3,
    prelude::OrthographicCameraBundle,
    render::camera::{DepthCalculation, OrthographicProjection, ScalingMode},
};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

use crate::{
    checksum::Checksum,
    components::{
        AngularVelocity, FrameCount, Input, Movable, Player, PlayerEntity, PlayerType, RoundEntity,
        ThrustEngine, Velocity, Ego, Others,
    },
    menu::{connect::LocalHandles, win::MatchData},
    AppState, GGRSConfig, ImageAssets, NUM_ALLIES, NUM_ENNEMIES, NUM_PLAYERS, PLAYER_SCALE,
};

pub(crate) const INPUT_UP: u8 = 0b0001;
pub(crate) const INPUT_LEFT: u8 = 0b0100;
pub(crate) const INPUT_RIGHT: u8 = 0b1000;
pub(crate) const INPUT_SPACE: u8 = 0b0010;
pub(crate) const LASER_SPEED: f32 = 50.;
pub(crate) const ARENA_SIZE: f32 = 720.0;
const PLAYER_SIZE: f32 = 50.;

pub fn input(handle: In<PlayerHandle>, keyboard_input: Res<bevy::input::Input<KeyCode>>) -> Input {
    let mut inp: u8 = 0;
    if keyboard_input.pressed(KeyCode::Up) {
        inp |= INPUT_UP
    }
    if keyboard_input.pressed(KeyCode::Left) {
        inp |= INPUT_LEFT
    }
    if keyboard_input.pressed(KeyCode::Right) {
        inp |= INPUT_RIGHT
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        inp |= INPUT_SPACE
    }

    Input { inp }
}

pub fn setup_round(mut commands: Commands) {
    commands.insert_resource(FrameCount::default());
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(RoundEntity);

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

pub fn spawn_players(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    game_textures: Res<ImageAssets>,
    local_handles: Res<LocalHandles>,
) {
    let r = ARENA_SIZE / 4.;

    let mut spawn_player = |transform: &Transform, player_type: PlayerType, handle: &usize| {
        let texture: Handle<Image>;
        if player_type == PlayerType::Ennemy {
            texture = game_textures.ennemy.clone();
        } else {
            texture = game_textures.ally.clone();
        }
        if handle == local_handles.handles.first().unwrap() {
            whoami = Ego;
        } else {
            whoami = Others;
        }

        commands
            .spawn_bundle(SpriteBundle {
                transform: *transform,
                texture: texture,
                ..Default::default()
            })
            .insert(Player { handle: *handle })
            .insert(Velocity::default())
            .insert(Movable {
                auto_despawn: false,
                steerable: true,
            })
            .insert(AngularVelocity { angle: 0. })
            .insert(ThrustEngine {
                on: false,
                force: 0.001,
            })
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(PlayerEntity)
            .insert(RoundEntity)
            .insert(player_type)
            .insert(whoami);
    };

    let get_spawn_location = |handle: usize| -> Transform {
        let rot = handle as f32 / NUM_PLAYERS as f32 * 2. * std::f32::consts::PI;
        let x = r * rot.cos();
        let y = r * rot.sin();

        let mut transform = Transform::from_translation(Vec3::new(x, y, 1.));
        transform
    };

    let mut handle: usize = 0;

    for _ in 0..NUM_ALLIES {
        spawn_player(&get_spawn_location(handle), PlayerType::Ally, &handle);
        handle += 1;
    }
    for _ in 0..NUM_ENNEMIES {
        spawn_player(&get_spawn_location(handle), PlayerType::Ennemy, &handle);
        handle += 1;
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
