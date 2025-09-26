use ::bevy::prelude::*;

pub const EMPTY_SPACE: f32 = 29.75 - MIN_FILL;

pub const GRAVITY: Vec3 = Vec3 {
  x: 0.,
  y: -9.8,
  z: 0.,
};

pub const MIN_FILL: f32 = 29.75 / 6.;

pub const NOT_CHARGING: Color = Color::linear_rgb(0.2, 0.2, 0.2);

pub const SPEED: f32 = 50.;
