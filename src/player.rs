// use crate::{
//     components::{AngularVelocity, Movable, Player, Velocity, ThrustEngine},
//     GameTextures, WinSize, BASE_SPEED, LASER_SCALE, TIME_STEP, PLAYER_SCALE,
// };
// use ::bevy::prelude::*;
// use bevy::{ecs::system::Command, transform, math::Vec2Swizzles};
// use bevy_prototype_lyon::{
//     entity::ShapeBundle,
//     prelude::{
//         tess::{geom::Rotation, math::Angle},
//         *,
//     },
//     shapes::Polygon,
// };
// pub struct PlayerPlugin;

// impl Plugin for PlayerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
//             .add_system(player_keyboard_event_system)
//             .add_system(thrust_system)
//             .add_system(player_fire_system);
//     }
// }

// fn player_spawn_system(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     game_textures: Res<GameTextures>,
//     win_size: Res<WinSize>,
// ) {
//     let bottom = -win_size.h / 2.;
//     commands
//         .spawn_bundle(GeometryBuilder::build_as(
//             &{
//                 let mut path_builder = PathBuilder::new();
//                 path_builder.move_to(Vec2::ZERO);
//                 path_builder.line_to(Vec2::new(-8.0, -8.0));
//                 path_builder.line_to(Vec2::new(0.0, 12.0));
//                 path_builder.line_to(Vec2::new(8.0, -8.0));
//                 path_builder.line_to(Vec2::ZERO);
//                 let mut line = path_builder.build();
//                 line.0 = line.0.transformed(&Rotation::new(Angle::degrees(-90.0)));
//                 line
//             },
//             DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.0)),
//             Transform {
//                 translation: Vec3::new(0., bottom, 10.),
//                 scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
//                 ..Default::default()
//             },
//         ))
//         .insert(Player)
//         .insert(Velocity { x: 0., y: 0. })
//         .insert(Movable {
//             auto_despawn: false,
//             steerable: true,
//         })
//         .insert(AngularVelocity { angle: 0. })
//         .insert(ThrustEngine { on: false, force: 0.001 });
// }

// fn player_keyboard_event_system(
//     keyboard: Res<Input<KeyCode>>,
//     mut query: Query<(&mut Velocity, &mut AngularVelocity, &mut ThrustEngine), With<Player>>,
// ) {
//     if let Ok((mut velocity, mut angular_velocity, mut thrust_engine )) = query.get_single_mut() {
//         if keyboard.pressed(KeyCode::Up) {
//             thrust_engine.on = true;
//             if thrust_engine.force <= 0.005 {
//                 thrust_engine.force += 0.0001;
//             }
//         } else {
//             thrust_engine.on = false;
//             thrust_engine.force = 0.001;
//         };
//         if keyboard.pressed(KeyCode::Down) {
//             if velocity.x.abs() > velocity.y.abs() {
//                 if velocity.x < 0. { velocity.x += 0.01 };
//                 if velocity.x > 0. { velocity.x -= 0.01 };
//             } else {
//                 if velocity.y < 0. { velocity.y += 0.01 };
//                 if velocity.y > 0. { velocity.y -= 0.01 };
//             }
//         };
//         angular_velocity.angle = if keyboard.pressed(KeyCode::Left) {
//             0.1
//         } else if keyboard.pressed(KeyCode::Right) {
//             -0.1
//         } else {
//             0.
//         };

//     }
// }

// fn player_fire_system(
//     mut commands: Commands,
//     keyboard: Res<Input<KeyCode>>,
//     game_textures: Res<GameTextures>,
//     query: Query<&Transform, With<Player>>,
// ) {
//     if let Ok(player_tf) = query.get_single() {
//         if keyboard.just_pressed(KeyCode::Space) {
//             let (x, y) = (player_tf.translation.x, player_tf.translation.y);            
//             let dir = player_tf.rotation * Vec3::X;
//             commands
//                 .spawn_bundle(SpriteBundle {
//                     texture: game_textures.player_laser.clone(),
//                     transform: Transform {
//                         translation: Vec3::new(x, y, 0.),
//                         rotation: player_tf.rotation.mul_quat(Quat::from_rotation_z((-90.0_f32).to_radians())),
//                         scale: Vec3::new(LASER_SCALE, LASER_SCALE, 1.),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 })
//                 .insert(Movable {
//                     auto_despawn: true,
//                     steerable: false,
//                 })
//                 .insert(Velocity { x: dir.x, y: dir.y })
//                 .insert(AngularVelocity { angle: 0. });
//         }
//     }
// }


// fn thrust_system(mut query: Query<(&mut Velocity, &Transform, &ThrustEngine), With<Player>>) {
//     if let Ok((mut velocity, transform, thrust_engine)) = query.get_single_mut(){
//         if thrust_engine.on {
//             let dir = transform.rotation * Vec3::X;
//             velocity.x += dir.x * thrust_engine.force;
//             velocity.y += dir.y * thrust_engine.force;
//         }
//     }
// }