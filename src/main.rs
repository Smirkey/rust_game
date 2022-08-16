#![allow(unused)]

use::bevy::prelude::*;

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
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(150., 150.)),
            ..Default::default()
        },
        ..Default::default()
    });
}