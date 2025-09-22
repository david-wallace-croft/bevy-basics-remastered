use super::constants::GRAVITY;
use super::velocity::Velocity;
use ::bevy::prelude::*;

pub fn apply_gravity(
  mut objects: Query<&mut Velocity>,
  time: Res<Time>,
) {
  let g: Vec3 = GRAVITY * time.delta_secs();

  for mut v in &mut objects {
    **v += g;
  }
}
