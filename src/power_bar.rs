use super::constants::EMPTY_SPACE;
use super::{
  constants::{MIN_FILL, NOT_CHARGING},
  power::Power,
};
use ::bevy::prelude::*;

#[derive(Component)]
pub struct PowerBar {
  pub max: f32,
  pub min: f32,
}

pub fn update_power_bar(
  mut bars: Query<(&mut Node, &PowerBar, &mut BackgroundColor)>,
  power: Res<Power>,
) {
  for (mut bar, config, mut bg) in &mut bars {
    if !power.charging {
      bg.0 = NOT_CHARGING;

      bar.width = Val::VMax(MIN_FILL);
    } else {
      let percent: f32 =
        (power.current - config.min) / (config.max - config.min);

      bg.0 = Color::linear_rgb(1. - percent, percent, 0.);

      bar.width = Val::VMax(MIN_FILL + percent * EMPTY_SPACE);
    }
  }
}
