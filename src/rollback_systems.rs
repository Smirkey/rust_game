use bevy::prelude::*;

use crate::{game::{FrameCount, Input, Player, INPUT_UP, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, ARENA_SIZE}, TIME_STEP, BASE_SPEED, components::{AngularVelocity, Movable, ThrustEngine, Velocity}};
use bevy_ggrs::{Rollback};
use ggrs::{InputStatus};

pub fn increase_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

pub fn apply_inputs(
    mut query: Query<(&mut Velocity,
        &mut Transform,
        &mut ThrustEngine,
        &mut AngularVelocity,
        &Player)>,
    inputs: Res<Vec<(Input, InputStatus)>>,
) {
    for (
        mut velocity,
        mut transform,
        mut thrust_engine, 
        mut angular_velocity, 
        player
    ) in query.iter_mut() {
        let input = match inputs[player.handle].1 {
            InputStatus::Confirmed => inputs[player.handle].0.inp,
            InputStatus::Predicted => inputs[player.handle].0.inp,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        angular_velocity.angle = if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            0.1
        } else if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            -0.1
        } else {
            0.
        };

        if input & INPUT_DOWN != 0 && input & INPUT_UP == 0 {
            if velocity.x.abs() > velocity.y.abs() {
                if velocity.x < 0. { velocity.x += 0.01 };
                if velocity.x > 0. { velocity.x -= 0.01 };
            } else {
                if velocity.y < 0. { velocity.y += 0.01 };
                if velocity.y > 0. { velocity.y -= 0.01 };
            }
        } else if input & INPUT_DOWN == 0 && input & INPUT_UP != 0 {
            thrust_engine.on = true;
            if thrust_engine.force <= 0.005 {
                thrust_engine.force += 0.0001;
            }
        } else {
            thrust_engine.on = false;
            thrust_engine.force = 0.001;
        };
    }
}


pub fn movable_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Velocity,
        &mut Transform,
        &Movable,
        &AngularVelocity,
    ), With<Rollback>>
) {
    for (entity, velocity, mut transform, movable, angular_velocity) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > ARENA_SIZE / 2. + MARGIN
                || translation.y < -ARENA_SIZE / 2. - MARGIN
                || translation.x > ARENA_SIZE / 2. + MARGIN
                || translation.x < -ARENA_SIZE / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        } else {
            if translation.y > ARENA_SIZE / 2. {translation.y = -ARENA_SIZE / 2.} 
            else if translation.y < -ARENA_SIZE / 2. {translation.y = -ARENA_SIZE / 2.}
            else if translation.x > ARENA_SIZE {translation.x =  -ARENA_SIZE / 2.}
            else if translation.x < -ARENA_SIZE / 2. {translation.x = ARENA_SIZE / 2.}
        }
        if movable.steerable {
            transform.rotate(Quat::from_rotation_z(angular_velocity.angle));
        }
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
            let dir = player_tf.rotation * Vec3::X;
            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.),
                        rotation: player_tf.rotation.mul_quat(Quat::from_rotation_z((-90.0_f32).to_radians())),
                        scale: Vec3::new(LASER_SCALE, LASER_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Movable {
                    auto_despawn: true,
                    steerable: false,
                })
                .insert(Velocity { x: dir.x, y: dir.y })
                .insert(AngularVelocity { angle: 0. });
        }
    }
}


fn thrust_system(mut query: Query<(&mut Velocity, &Transform, &ThrustEngine), With<Player>>) {
    if let Ok((mut velocity, transform, thrust_engine)) = query.get_single_mut(){
        if thrust_engine.on {
            let dir = transform.rotation * Vec3::X;
            velocity.x += dir.x * thrust_engine.force;
            velocity.y += dir.y * thrust_engine.force;
        }
    }
}