use ::bevy::prelude::*;

#[derive(Resource)]
pub struct Power {
  pub charging: bool,
  pub current: f32,
}
