use super::player::Player;
use ::bevy::prelude::*;
use ::bevy::window::PrimaryWindow;

#[derive(Event)]
pub struct BallSpawn {
  position: Vec3,
}

pub fn shoot_ball(
  input: Res<ButtonInput<MouseButton>>,
  player: Single<&Transform, With<Player>>,
  mut spawner: EventWriter<BallSpawn>,
  window: Single<&Window, With<PrimaryWindow>>,
) {
  if window.cursor_options.visible {
    return;
  }

  if !input.just_pressed(MouseButton::Left) {
    return;
  }

  spawner.write(BallSpawn {
    position: player.translation,
  });
}

pub fn spawn_ball(
  mut events: EventReader<BallSpawn>,
  mut commands: Commands,
  mut mesh_assets: ResMut<Assets<Mesh>>,
  mut mat_assets: ResMut<Assets<StandardMaterial>>,
) {
  for spawn in events.read() {
    commands.spawn((
      Transform::from_translation(spawn.position),
      Mesh3d(mesh_assets.add(Sphere::new(1.))),
      MeshMaterial3d(mat_assets.add(StandardMaterial::default())),
    ));
  }
}
