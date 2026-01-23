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

    let border_radius_0: BorderRadius = BorderRadius::all(Val::VMax(5.));

    let node_0: Node = Node {
      border_radius: border_radius_0,
      bottom: Val::Px(20.),
      height: Val::VMax(5.),
      left: Val::Px(20.),
      position_type: PositionType::Absolute,
      width: Val::VMax(30.),
      ..Default::default()
    };

    let background_color_0: BackgroundColor =
      BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5));

    let border_radius_1: BorderRadius = BorderRadius::all(Val::VMax(5.));

    let node_1: Node = Node {
      border_radius: border_radius_1,
      height: Val::Percent(95.),
      margin: UiRect::all(Val::VMax(0.125)),
      min_width: Val::VMax(MIN_FILL),
      position_type: PositionType::Absolute,
      ..Default::default()
    };

    let background_color_1: BackgroundColor = BackgroundColor(NOT_CHARGING);

    let power_bar_1: PowerBar = PowerBar {
      max: 6.,
      min: 1.,
    };

    commands.spawn((node_0, background_color_0)).with_child((
      node_1,
      background_color_1,
      power_bar_1,
    ));
  }
}
