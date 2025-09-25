use super::ball_data::BallData;
use super::player::Player;
use super::velocity::Velocity;
use ::bevy::prelude::*;
use ::bevy::window::PrimaryWindow;

#[derive(Event)]
pub struct BallSpawn {
  position: Vec3,
  power: f32,
  velocity: Vec3,
}

pub fn shoot_ball(
  input: Res<ButtonInput<MouseButton>>,
  player: Single<&Transform, With<Player>>,
  mut power: Local<Option<f32>>,
  mut spawner: EventWriter<BallSpawn>,
  time: Res<Time>,
  window: Single<&Window, With<PrimaryWindow>>,
) {
  if window.cursor_options.visible {
    return;
  }

  if let Some(current) = power.as_mut() {
    if input.just_released(MouseButton::Left) {
      spawner.write(BallSpawn {
        position: player.translation,
        power: *current,
        velocity: player.forward().as_vec3() * 15.,
      });
    }

    if input.pressed(MouseButton::Left) {
      *current += time.delta_secs();
    } else {
      *power = None
    }
  }

  if input.just_pressed(MouseButton::Left) {
    *power = Some(1.);
  }
}

pub fn spawn_ball(
  mut events: EventReader<BallSpawn>,
  mut commands: Commands,
  ball_data: Res<BallData>,
) {
  for spawn in events.read() {
    commands.spawn((
      Transform::from_translation(spawn.position),
      Mesh3d(ball_data.mesh()),
      MeshMaterial3d(ball_data.material()),
      Velocity(spawn.velocity * spawn.power * 10.),
    ));
  }
}
