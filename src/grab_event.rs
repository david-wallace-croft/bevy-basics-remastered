use ::bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Deref, Event)]
pub struct GrabEvent(bool);

pub fn apply_grab(
  grab: Trigger<GrabEvent>,
  mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
  if **grab {
    window.cursor_options.visible = false;

    window.cursor_options.grab_mode = CursorGrabMode::Locked;
  } else {
    window.cursor_options.visible = true;

    window.cursor_options.grab_mode = CursorGrabMode::None;
  }
}
