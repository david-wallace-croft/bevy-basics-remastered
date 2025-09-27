use self::ball_data::BallData;
use self::ball_spawn::BallSpawn;
use self::ball_spawn::shoot_ball;
use self::ball_spawn::spawn_ball;
use self::bounce::bounce;
use self::constants::MIN_FILL;
use self::constants::NOT_CHARGING;
use self::grab_event::apply_grab;
use self::grab_event::focus_events;
use self::grab_event::toggle_grab;
use self::gravity::apply_gravity;
use self::player::Player;
use self::player::player_look;
use self::player::player_move;
use self::power::Power;
use self::power_bar::PowerBar;
use self::power_bar::update_power_bar;
use self::velocity::apply_velocity;
use ::bevy::input::common_conditions::input_just_released;
use ::bevy::prelude::*;

mod ball_data;
mod ball_spawn;
mod bounce;
mod constants;
mod grab_event;
mod gravity;
mod player;
mod player_speed;
mod power;
mod power_bar;
mod velocity;

fn main() {
  let mut app = App::new();

  app.add_plugins(DefaultPlugins);

  app.add_systems(Startup, (spawn_camera, spawn_map));

  app.insert_resource(Time::<Fixed>::from_hz(30.));

  app.add_systems(
    FixedUpdate,
    (
      apply_velocity,
      apply_gravity.before(apply_velocity),
      bounce.after(apply_velocity),
    ),
  );

  app.add_systems(
    Update,
    (
      player_look,
      player_move.after(player_look),
      focus_events,
      toggle_grab.run_if(input_just_released(KeyCode::Escape)),
      spawn_ball,
      shoot_ball.before(spawn_ball).before(focus_events),
      update_power_bar,
    ),
  );

  app.add_observer(apply_grab);

  app.add_event::<BallSpawn>();

  app.init_resource::<BallData>();

  app.insert_resource(Power {
    charging: false,
    current: 0.,
  });

  let _app_exit: AppExit = app.run();
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((Camera3d::default(), Player));
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
