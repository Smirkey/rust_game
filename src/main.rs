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
use components::{AngularVelocity, Movable, Velocity};
use player::PlayerPlugin;
mod components;
mod player;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
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
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .run();
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
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
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
        }
        if movable.steerable {
            transform.rotate(Quat::from_rotation_z(angular_velocity.angle));
        }
    }
}
