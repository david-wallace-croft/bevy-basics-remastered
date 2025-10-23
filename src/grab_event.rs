use ::bevy::prelude::*;
use ::bevy::window::{
  CursorGrabMode, CursorOptions, PrimaryWindow, WindowFocused,
};

#[derive(Deref, Event)]
pub struct GrabEvent(bool);

pub fn apply_grab(
  grab: On<GrabEvent>,
  mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
  if **grab {
    cursor_options.visible = false;

    cursor_options.grab_mode = CursorGrabMode::Locked;
  } else {
    cursor_options.visible = true;

    cursor_options.grab_mode = CursorGrabMode::None;
  }
}

pub fn focus_events(
  mut messages: MessageReader<WindowFocused>,
  mut commands: Commands,
) {
  if let Some(event) = messages.read().last() {
    commands.trigger(GrabEvent(event.focused));
  }
}

pub fn toggle_grab(
  mut window: Single<&mut Window, With<PrimaryWindow>>,
  mut commands: Commands,
) {
  window.focused = !window.focused;

  commands.trigger(GrabEvent(window.focused));
}
