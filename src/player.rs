use super::constants::SPEED;
use ::bevy::input::mouse::AccumulatedMouseMotion;
use ::bevy::math::EulerRot;
use ::bevy::prelude::*;
use ::bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Player;

pub fn player_look(
  mut player: Single<&mut Transform, With<Player>>,
  mouse_motion: Res<AccumulatedMouseMotion>,
  time: Res<Time>,
  window: Single<&Window, With<PrimaryWindow>>,
) {
  if !window.focused {
    return;
  }

  let dt: f32 = time.delta_secs();

  let sensitivity: f32 = 100. / window.width().min(window.height());

  let (mut yaw, mut pitch, _) = player.rotation.to_euler(EulerRot::YXZ);

  pitch -= mouse_motion.delta.y * dt * sensitivity;

  yaw -= mouse_motion.delta.x * dt * sensitivity;

  pitch = pitch.clamp(-1.57, 1.57);

  player.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
}

pub fn player_move(
  mut player: Single<&mut Transform, With<Player>>,
  input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
  let mut delta = Vec3::ZERO;

  if input.pressed(KeyCode::KeyA) {
    delta.x -= 1.;
  }

  if input.pressed(KeyCode::KeyD) {
    delta.x += 1.;
  }

  if input.pressed(KeyCode::KeyW) {
    delta.z += 1.;
  }

  if input.pressed(KeyCode::KeyS) {
    delta.z -= 1.;
  }

  let forward = player.forward().as_vec3() * delta.z;

  let right = player.right().as_vec3() * delta.x;

  let mut to_move = forward + right;

  to_move.y = 0.;

  to_move = to_move.normalize_or_zero();

  player.translation = to_move * time.delta_secs() * SPEED;
}
