use ::bevy::prelude::*;

use EulerRot::XYZ;

pub struct Player;

// pub fn player_look(
//   mut player: Single<&mut Transform, With<Player>>,
//   mouse_motion: Res<AccumulatedMouseMotion>,
//   time: Res<Time>,
// ) {
//   let dt = time.delta_secs();

//   let sensitivity = 0.1;

//   let (mut yaw, mut pitch, _) = player.rotation.to_euler(YXZ);

//   pitch -= mouse_motion.delta.y * dt * sensitivity;

//   yaw -= mouse_motion.delta.x * dt * sensitivity;

//   pitch = pitch.clamp(-1.57, 1.57);

//   player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);
// }
