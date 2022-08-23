#![allow(unused)]

mod components;
mod player;
mod game;
mod checksum;
mod menu;
mod rollback_systems;

use ::bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{geom::Rotation, math::Angle},
        *,
    },
    shapes::Polygon,
};
use components::{AngularVelocity, Movable, Velocity};
use ggrs::Config;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use rollback_systems::movable_system;
// use game::{
//     apply_inputs, check_win, increase_frame_count, move_players, print_p2p_events, setup_round,
//     spawn_players, update_velocity, FrameCount, Velocity,
// };



const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SCALE: f32 = 1.2;
const LASER_SPRITE: &str = "laser_a_01.png";
const LASER_SIZE: (f32, f32) = (9., 54.);
const LASER_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const NUM_PLAYERS: usize = 2;
const FPS: usize = 60;
const ROLLBACK_SYSTEMS: &str = "rollback_systems";
const CHECKSUM_UPDATE: &str = "checksum_update";
const MAX_PREDICTION: usize = 12;
const INPUT_DELAY: usize = 2;
const CHECK_DISTANCE: usize = 2;

const DISABLED_BUTTON: Color = Color::rgb(0.8, 0.5, 0.5);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);


pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "images/skull.png")]
    pub ggrs_logo: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default_font: Handle<Font>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
    MenuOnline,
    MenuConnect,
    RoundLocal,
    RoundOnline,
    Win,
}

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = game::Input;
    type State = u8;
    type Address = String;
}




fn main() {
    let mut app = App::new();

    AssetLoader::new(AppState::AssetLoading)
        .continue_to_state(AppState::MenuMain)
        .with_collection::<ImageAssets>()
        .with_collection::<FontAssets>()
        .build(&mut app);
    
    // app
    //     .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    //     .insert_resource(WindowDescriptor {
    //         title: "Rust test".to_string(),
    //         width: 600.,
    //         height: 700.,
    //         ..Default::default()
    //     })
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(ShapePlugin)
    //     .add_plugin(PlayerPlugin)
    //     .add_startup_system(setup_system)
    //     .add_system(movable_system);
    
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
