use bevy::prelude::*;

use crate::{game::{WinSize, FrameCount, Input, Player, INPUT_UP, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT}, TIME_STEP, BASE_SPEED, components::{AngularVelocity, Movable, ThrustEngine, Velocity}};
use bevy_ggrs::{Rollback};
use ggrs::{InputStatus};

pub fn increase_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

pub fn apply_inputs(
    mut query: Query<(&Velocity,
        &mut Transform,
        &mut ThrustEngine,
        &AngularVelocity,
        &Player)>,
    inputs: Res<Vec<(Input, InputStatus)>>,
) {
    for (velocity, mut transform, thrust_engine, angular_velocity, player) in query.iter_mut() {
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
    win_size: Res<WinSize>,
    mut query: Query<(
        Entity,
        &Velocity,
        &mut Transform,
        &Movable,
        &AngularVelocity,
    )>
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