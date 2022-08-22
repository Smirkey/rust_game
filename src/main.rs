#![allow(unused)]

use ::bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{geom::Rotation, math::Angle},
        *,
    },
    shapes::Polygon,
};
use components::{AngularVelocity, Movable, Velocity, GGRSConfig};
use ggrs::{GGRSError, PlayerType, SessionBuilder, SessionState, UdpNonBlockingSocket};
use player::PlayerPlugin;
use std::net::SocketAddr;
use structopt::StructOpt;
use instant::{Duration, Instant};



mod components;
mod player;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SCALE: f32 = 1.2;
const LASER_SPRITE: &str = "laser_a_01.png";
const LASER_SIZE: (f32, f32) = (9., 54.);
const LASER_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const FPS: f64 = 60.0;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<SocketAddr>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let opt = Opt::from_args();
    let num_players = opt.players.len();
    let mut app = App::new();
    
    let mut sess_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_fps(FPS as usize)? // (optional) set expected update frequency
        .with_input_delay(2); // (optional) set input delay for the local player

    // add players
    for (i, player_addr) in opt.players.iter().enumerate() {
        // local player
        if player_addr == "localhost" {
            sess_build = sess_build.add_player(PlayerType::Local, i)?;
        } else {
            // remote players
            let remote_addr: SocketAddr = player_addr.parse()?;
            sess_build = sess_build.add_player(PlayerType::Remote(remote_addr), i)?;
        }
    }

    // optionally, add spectators
    for (i, spec_addr) in opt.spectators.iter().enumerate() {
        sess_build = sess_build.add_player(PlayerType::Spectator(*spec_addr), num_players + i)?;
    }

    // start the GGRS session
    let socket = UdpNonBlockingSocket::bind_to_port(opt.local_port)?;
    let mut sess = sess_build.start_p2p_session(socket)?;

    // Create a new box game
    // let mut game = Game::new(num_players); // TODO: Heavy refacto :/
    // game.register_local_handles(sess.local_player_handles());

    // time variables for tick rate
    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    app
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust test".to_string(),
            width: 600.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system);

    app.run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();

    let win_size = WinSize {
        w: window.width(),
        h: window.height(),
    };
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(LASER_SPRITE),
    };
    commands.insert_resource(game_textures);

    window.set_title(String::from("my rust game"));
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(
        Entity,
        &Velocity,
        &mut Transform,
        &Movable,
        &AngularVelocity,
    )>,
) {
    for (entity, velocity, mut transform, movable, angular_velocity) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        } else {
            if translation.y > win_size.h / 2. {translation.y = -win_size.h / 2.} 
            else if translation.y < -win_size.h / 2. {translation.y = -win_size.h / 2.}
            else if translation.x > win_size.w {translation.x =  -win_size.w / 2.}
            else if translation.x < -win_size.w / 2. {translation.x = win_size.w / 2.}
        }
        if movable.steerable {
            transform.rotate(Quat::from_rotation_z(angular_velocity.angle));
        }
    }
}
