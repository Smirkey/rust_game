#![allow(unused)]

use::bevy::prelude::*;
use player::PlayerPlugin;

mod player;
mod components;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;


pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust test".to_string(),
            width: 600.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();

    let win_size = WinSize{ w: window.width(), h: window.height() };
    commands.insert_resource(win_size);

    let game_textures = GameTextures{
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);

    window.set_title(String::from("my rust game"));
}

fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    let bottom = -win_size.h / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE, 10.), 
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}