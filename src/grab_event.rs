use ::bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};

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

pub fn focus_events(
  mut events: EventReader<WindowFocused>,
  mut commands: Commands,
) {
  if let Some(event) = events.read().last() {
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
