use ::bevy::input::mouse::AccumulatedMouseMotion;
use ::bevy::math::EulerRot;
use ::bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn player_look(
  mut player: Single<&mut Transform, With<Player>>,
  mouse_motion: Res<AccumulatedMouseMotion>,
  time: Res<Time>,
) {
  let dt: f32 = time.delta_secs();

  let sensitivity: f32 = 0.1;

  let (mut yaw, mut pitch, _) = player.rotation.to_euler(EulerRot::YXZ);

  pitch -= mouse_motion.delta.y * dt * sensitivity;

  yaw -= mouse_motion.delta.x * dt * sensitivity;

  pitch = pitch.clamp(-1.57, 1.57);

  player.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
}
