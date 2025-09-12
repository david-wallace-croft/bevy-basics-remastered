use self::player::Player;
// use self::player::player_look;
use ::bevy::prelude::*;

mod player;

fn main() {
  let mut app = App::new();

  app.add_plugins(DefaultPlugins);

  app.add_systems(Startup, (spawn_camera, spawn_map));

  //   app.add_systems(Update, (player_look));

  let _app_exit: AppExit = app.run();
}

fn spawn_camera(mut commands: Commands) {
  //   commands.spawn((Camera3d::default(), Player));
  commands.spawn(Camera3d::default());
}

fn spawn_map(
  mut commands: Commands,
  mut mesh_assets: ResMut<Assets<Mesh>>,
  mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(DirectionalLight::default());

  let ball_mesh: Handle<Mesh> = mesh_assets.add(Sphere::new(1.));

  for h in 0..16 {
    let color: Color = Color::hsl((h as f32 / 16.) * 360., 1., 0.5);

    let ball_material: Handle<StandardMaterial> =
      material_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
      });

    commands.spawn((
      Transform::from_translation(Vec3::new((-8. + h as f32) * 2., 0., -50.)),
      Mesh3d(ball_mesh.clone()),
      MeshMaterial3d(ball_material),
    ));
  }
}
