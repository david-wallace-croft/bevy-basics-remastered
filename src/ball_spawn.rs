use super::ball_data::BallData;
use super::player::Player;
use super::power::Power;
use super::velocity::Velocity;
use ::bevy::prelude::*;
use ::bevy::window::CursorOptions;
use ::bevy::window::PrimaryWindow;

#[derive(Message)]
pub struct BallSpawn {
  position: Vec3,
  power: f32,
  velocity: Vec3,
}

pub fn shoot_ball(
  cursor_options: Single<&CursorOptions, With<PrimaryWindow>>,
  input: Res<ButtonInput<MouseButton>>,
  player: Single<&Transform, With<Player>>,
  mut power: ResMut<Power>,
  mut spawner: MessageWriter<BallSpawn>,
  time: Res<Time>,
) {
  if cursor_options.visible {
    return;
  }

  if power.charging {
    if input.just_released(MouseButton::Left) {
      spawner.write(BallSpawn {
        position: player.translation,
        power: power.current,
        velocity: player.forward().as_vec3() * 15.,
      });
    }

    if input.pressed(MouseButton::Left) {
      power.current += time.delta_secs();

      power.current = power.current.clamp(1., 6.);
    } else {
      power.charging = false;
    }
  }

  if input.just_pressed(MouseButton::Left) {
    power.charging = true;

    power.current = 1.;
  }
}

pub fn spawn_ball(
  mut events: MessageReader<BallSpawn>,
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
