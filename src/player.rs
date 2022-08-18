use crate::{
    components::{AngularVelocity, Movable, Player, Velocity},
    GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP,
};
use ::bevy::prelude::*;
use bevy::{ecs::system::Command, transform};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{geom::Rotation, math::Angle},
        *,
    },
    shapes::Polygon,
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.h / 2.;
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
                line
            },
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.0)),
            Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE, 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
        ))
        .insert(Player)
        .insert(Velocity { x: 0., y: 0. })
        .insert(Movable {
            auto_despawn: false,
            steerable: true,
        })
        .insert(AngularVelocity { angle: 180. });
}

fn player_keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if keyboard.pressed(KeyCode::Left) {
            -1.
        } else if keyboard.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
    }
}

fn player_fire_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if keyboard.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            let mut spawn_lasers = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 0.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Movable {
                        auto_despawn: true,
                        steerable: true,
                    })
                    .insert(Velocity { x: 0., y: 1. })
                    .insert(AngularVelocity { angle: 0. });
            };

            spawn_lasers(x_offset);
            spawn_lasers(-x_offset);
        }
    }
}
