use ::bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub fn apply_velocity(
  mut objects: Query<(&mut Transform, &Velocity)>,
  time: Res<Time>,
) {
  for (mut transform, velocity) in &mut objects {
    transform.translation += velocity.0 * time.delta_secs();
  }
}
