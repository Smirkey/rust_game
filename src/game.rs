use crate::{
    checksum::Checksum,
    components::{
        AngularVelocity, FrameCount, Input, Movable, PlayerEntity, RoundEntity, ThrustEngine,
        Velocity,
    },
    menu::{connect::LocalHandles, win::MatchData},
    AppState, GGRSConfig, ImageAssets, NUM_ALLIES, NUM_ENNEMIES, NUM_PLAYERS, PLAYER_SCALE,
};
use bevy::render::camera::{CameraPlugin, CameraProjection, DepthCalculation};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;
use bevy::{math::Vec3, prelude::OrthographicCameraBundle};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

pub(crate) const INPUT_UP: u8 = 0b0001;
pub(crate) const INPUT_LEFT: u8 = 0b0100;
pub(crate) const INPUT_RIGHT: u8 = 0b1000;
pub(crate) const INPUT_SPACE: u8 = 0b0010;
pub(crate) const LASER_SPEED: f32 = 50.;
pub(crate) const ARENA_SIZE: f32 = 2000.0;
const PLAYER_SIZE: f32 = 50.;
const TILE_SIZE: f32 = 200.;
const TILE_COLORS: [Color; 2] = [Color::DARK_GRAY, Color::ANTIQUE_WHITE];

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

pub fn setup_camera(mut commands: Commands, local_handles: Res<LocalHandles>) {
    let far = 500.0;
    let orthographic_projection = OrthographicProjection {
        left: 0.0,
        right: 500.0,
        top: 500.0,
        bottom: 100.0,
        depth_calculation: DepthCalculation::ZDifference,
        ..Default::default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
    let view_projection =
        orthographic_projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        orthographic_projection.far(),
    );
    let camera_bundle = OrthographicCameraBundle {
        camera: Camera {
            name: Some(CameraPlugin::CAMERA_2D.to_string()),
            near: orthographic_projection.near,
            far: orthographic_projection.far,
            ..Default::default()
        },
        orthographic_projection,
        visible_entities: VisibleEntities::default(),
        frustum,
        transform,
        global_transform: Default::default(),
    };

    commands.spawn_bundle(camera_bundle).insert(RoundEntity);
}

pub fn setup_round(mut commands: Commands, game_textures: Res<ImageAssets>) {
    // map terrain generation
    commands.insert_resource(FrameCount::default());
    for i in -((ARENA_SIZE / 2.) / TILE_SIZE) as i32..((ARENA_SIZE / 2.) / TILE_SIZE) as i32 {
        for j in -((ARENA_SIZE / 2.) / TILE_SIZE) as i32..((ARENA_SIZE / 2.) / TILE_SIZE) as i32 {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        i as f32 * TILE_SIZE,
                        j as f32 * TILE_SIZE,
                        1.,
                    )),
                    sprite: Sprite {
                        color: if (i % 2 == 0 && j % 2 == 0) || (i % 2 != 0 && j % 2 != 0) {
                            TILE_COLORS[0]
                        } else {
                            TILE_COLORS[1]
                        },
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(RoundEntity);
        }
    }
}

pub fn spawn_players(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    game_textures: Res<ImageAssets>,
    local_handles: Res<LocalHandles>,
) {
    let r = ARENA_SIZE / 4.;

    let ego_handle = local_handles.handles.first().unwrap();
    let mut spawn_player = |transform: &Transform, team: bool, handle: &usize| {
        commands
            .spawn_bundle(SpriteBundle {
                transform: *transform,
                texture: if team {
                    game_textures.ennemy.clone()
                } else {
                    game_textures.ally.clone()
                },
                ..Default::default()
            })
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
            .insert(RoundEntity)
            .insert(PlayerEntity {
                ego: if handle == ego_handle { true } else { false },
                handle: *handle,
                team,
                size: match team {
                    true => Vec2::new(75.0, 98.0),
                    false => Vec2::new(84.0, 93.0),
                },
            });
    };

    let get_spawn_location = |handle: usize| -> Transform {
        let rot = handle as f32 / NUM_PLAYERS as f32 * 2. * std::f32::consts::PI;
        let x = r * rot.cos();
        let y = r * rot.sin();

        let mut transform = Transform::from_translation(Vec3::new(x, y, 3.));
        transform
    };

    let mut handle: usize = 0;

    for _ in 0..NUM_ALLIES {
        spawn_player(&get_spawn_location(handle), false, &handle);
        handle += 1;
    }
    for _ in 0..NUM_ENNEMIES {
        spawn_player(&get_spawn_location(handle), true, &handle);
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
