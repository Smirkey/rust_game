use bevy::prelude::*;

use crate::{
    components::{
        AngularVelocity, LaserEntity, Movable, PlayerEntity, PlayerType, ThrustEngine, Velocity,
    },
    components::{FrameCount, Input, Player, RoundEntity},
    game::{ARENA_SIZE, INPUT_LEFT, INPUT_RIGHT, INPUT_SPACE, INPUT_UP, LASER_SPEED},
    ImageAssets, BASE_SPEED, LASER_SCALE, TIME_STEP,
};
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::InputStatus;

pub fn increase_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

pub fn apply_inputs(
    mut query: Query<
        (
            &mut Velocity,
            &mut Transform,
            &mut ThrustEngine,
            &mut AngularVelocity,
            &Player,
        ),
        With<PlayerEntity>,
    >,
    inputs: Res<Vec<(Input, InputStatus)>>,
) {
    for (mut velocity, mut transform, mut thrust_engine, mut angular_velocity, player) in
        query.iter_mut()
    {
        let input = match inputs[player.handle].1 {
            InputStatus::Confirmed => inputs[player.handle].0.inp,
            InputStatus::Predicted => inputs[player.handle].0.inp,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        angular_velocity.angle = if input & INPUT_LEFT != 0 {
            0.1
        } else if input & INPUT_RIGHT != 0 {
            -0.1
        } else {
            0.
        };

        if input & INPUT_UP != 0 {
            let dir = transform.rotation * Vec3::X;
            velocity.x += dir.x * thrust_engine.force;
            velocity.y += dir.y * thrust_engine.force;
        } else {
            if velocity.x.abs() > velocity.y.abs() {
                if velocity.x < 0. {
                    velocity.x += 0.01
                };
                if velocity.x > 0. {
                    velocity.x -= 0.01
                };
            } else {
                if velocity.y < 0. {
                    velocity.y += 0.01
                };
                if velocity.y > 0. {
                    velocity.y -= 0.01
                };
            }
        };
    }
}

pub fn camera_system(
    mut camera: Query<&mut Transform, (With<Camera>, Without<PlayerEntity>)>,
    mut player: Query<(&mut Transform, &mut PlayerType), (With<PlayerEntity>, Without<Camera>)>,
) {
    for mut transform in camera.iter_mut() {
        for (player_tf, player_type) in player.iter() {
            if player_type == &PlayerType::Ego {
                transform.translation = player_tf.translation;
            }
        }
    }
}

pub fn movable_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Velocity,
            &mut Transform,
            &Movable,
            &AngularVelocity,
        ),
        With<Rollback>,
    >,
) {
    for (entity, velocity, mut transform, movable, angular_velocity) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 100.;
            if translation.y > ARENA_SIZE / 2. + MARGIN
                || translation.y < -ARENA_SIZE / 2. - MARGIN
                || translation.x > ARENA_SIZE / 2. + MARGIN
                || translation.x < -ARENA_SIZE / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        } else {
            if translation.y > ARENA_SIZE / 2. {
                translation.y = -ARENA_SIZE / 2.
            } else if translation.y < -ARENA_SIZE / 2. {
                translation.y = -ARENA_SIZE / 2.
            } else if translation.x > ARENA_SIZE {
                translation.x = -ARENA_SIZE / 2.
            } else if translation.x < -ARENA_SIZE / 2. {
                translation.x = ARENA_SIZE / 2.
            }
        }
        if movable.steerable {
            transform.rotate(Quat::from_rotation_z(angular_velocity.angle));
        }
    }
}

pub fn player_fire_system(
    mut commands: Commands,
    inputs: Res<Vec<(Input, InputStatus)>>,
    game_textures: Res<ImageAssets>,
    mut query: Query<(&Transform, &Player, &Velocity, &PlayerType), With<Rollback>>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    for (player_tf, player, player_velocity, player_type) in query.iter_mut() {
        let input = match inputs[player.handle].1 {
            InputStatus::Confirmed => inputs[player.handle].0.inp,
            InputStatus::Predicted => inputs[player.handle].0.inp,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };
        if input & INPUT_SPACE != 0 {
            let laser_texture: Handle<Image>;
            if player_type == &PlayerType::Ennemy {
                laser_texture = game_textures.ennemy_laser.clone();
            } else {
                laser_texture = game_textures.ally_laser.clone();
            }
            let dir = player_tf.rotation * Vec3::X;
            commands
                .spawn_bundle(SpriteBundle {
                    texture: laser_texture,
                    transform: Transform {
                        translation: Vec3::new(
                            player_tf.translation.x,
                            player_tf.translation.y,
                            0.,
                        ),
                        rotation: player_tf
                            .rotation
                            .mul_quat(Quat::from_rotation_z((-90.0_f32).to_radians())),
                        scale: Vec3::new(LASER_SCALE, LASER_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Movable {
                    auto_despawn: true,
                    steerable: false,
                })
                .insert(Velocity {
                    x: dir.x + player_velocity.x,
                    y: dir.y + player_velocity.y,
                })
                .insert(AngularVelocity { angle: 0. })
                .insert(LaserEntity)
                .insert(Rollback::new(rip.next_id()))
                .insert(RoundEntity);
        }
    }
}
