use self::ball_data::BallData;
use self::ball_spawn::BallSpawn;
use self::ball_spawn::shoot_ball;
use self::ball_spawn::spawn_ball;
use self::bounce::bounce;
use self::camera::spawn_camera;
use self::grab_event::apply_grab;
use self::grab_event::focus_events;
use self::grab_event::toggle_grab;
use self::gravity::apply_gravity;
use self::map::spawn_map;
use self::player::player_look;
use self::player::player_move;
use self::power::Power;
use self::power_bar::update_power_bar;
use self::velocity::apply_velocity;
use ::bevy::input::common_conditions::input_just_released;
use ::bevy::prelude::*;

mod ball_data;
mod ball_spawn;
mod bounce;
mod camera;
mod constants;
mod grab_event;
mod gravity;
mod map;
mod player;
mod player_speed;
mod power;
mod power_bar;
mod velocity;

fn main() -> AppExit {
  let mut app: App = App::new();

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

  let app_exit: AppExit = app.run();

  app_exit
}
