use ::bevy::prelude::*;

#[derive(Component)]
struct PowerBar {
  max: f32,
  min: f32,
}
