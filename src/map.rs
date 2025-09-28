use super::constants::MIN_FILL;
use super::constants::NOT_CHARGING;
use super::power_bar::PowerBar;
use ::bevy::prelude::*;

pub fn spawn_map(
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

    commands
      .spawn((
        Node {
          bottom: Val::Px(20.),
          height: Val::VMax(5.),
          left: Val::Px(20.),
          position_type: PositionType::Absolute,
          width: Val::VMax(30.),
          ..Default::default()
        },
        BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5)),
        BorderRadius::all(Val::VMax(5.)),
      ))
      .with_child((
        Node {
          height: Val::Percent(95.),
          margin: UiRect::all(Val::VMax(0.125)),
          min_width: Val::VMax(MIN_FILL),
          position_type: PositionType::Absolute,
          ..Default::default()
        },
        BackgroundColor(NOT_CHARGING),
        BorderRadius::all(Val::VMax(5.)),
        PowerBar {
          max: 6.,
          min: 1.,
        },
      ));
  }
}
