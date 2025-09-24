use super::velocity::Velocity;
use ::bevy::prelude::*;

pub fn bounce(mut balls: Query<(&Transform, &mut Velocity)>) {
  for (transform, mut velocity) in &mut balls {
    if transform.translation.y < 0. && velocity.y < 0. {
      velocity.y *= -1.;
    }
  }
}
