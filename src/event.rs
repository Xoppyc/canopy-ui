//! DOM-inspired UI event types, bubbling control, and input state.

use crate::event::EventResult::{Handled, Ignored};

/// Mouse buttons recognized by the UI system.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Keyboard modifier flags active during an event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl Modifiers {
    pub const NONE: Self = Self {
        shift: false,
        ctrl: false,
        alt: false,
    };

    /// Returns true if any modifier key is currently held down.
    #[inline]
    pub fn any(&self) -> bool {
        return self.shift || self.ctrl || self.alt;
    }
}

/// Specific payload of a UI event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIEventKind {
    MouseMove {
        x: f32,
        y: f32,
    },
    MouseDown {
        x: f32,
        y: f32,
        button: MouseButton,
        modifiers: Modifiers,
    },
    MouseUp {
        x: f32,
        y: f32,
        button: MouseButton,
        modifiers: Modifiers,
    },
    MouseWheel {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
        modifiers: Modifiers,
    },
    KeyDown {
        key_code: u32,
        modifiers: Modifiers,
    },
    KeyUp {
        key_code: u32,
        modifiers: Modifiers,
    },
    CharInput {
        ch: char,
    },
}

/// Indicates whether an event was consumed by the UI tree or should pass through to the game.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EventResult {
    #[default]
    Ignored,
    Handled,
}

impl EventResult {
    #[inline]
    pub fn is_handled(&self) -> bool {
        return *self == Handled;
    }

    /// Combines two results: returns `Handled` if either `self` or `other` is `Handled`.
    #[inline]
    pub fn or(self, other: Self) -> Self {
        return if self.is_handled() || other.is_handled() {
            Handled
        } else {
            Ignored
        };
    }
}

/// Event wrapper that travels (bubbles) through the component tree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UIEvent {
    pub kind: UIEventKind,
    propagation_stopped: bool,
}

impl UIEvent {
    pub const fn new(kind: UIEventKind) -> Self {
        Self {
            kind,
            propagation_stopped: false,
        }
    }

    /// Stops the event from bubbling up to parent components (like `e.stopPropagation()` in JS).
    #[inline]
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true
    }

    /// Returns true if `stop_propagation()` has been called on this event.
    #[inline]
    pub fn is_propagation_stopped(&self) -> bool {
        return self.propagation_stopped;
    }

    /// If this is a mouse/spatial event, returns `Some((x, y))`.
    /// For keyboard events, returns `None`.
    #[inline]
    pub fn coords(&self) -> Option<(f32, f32)> {
        match self.kind {
            UIEventKind::MouseMove { x, y } => Some((x, y)),
            UIEventKind::MouseDown {
                x,
                y,
                button: _,
                modifiers: _,
            } => Some((x, y)),
            UIEventKind::MouseUp {
                x,
                y,
                button: _,
                modifiers: _,
            } => Some((x, y)),
            UIEventKind::MouseWheel {
                x,
                y,
                delta_x: _,
                delta_y: _,
                modifiers: _,
            } => Some((x, y)),
            UIEventKind::KeyDown {
                key_code: _,
                modifiers: _,
            } => None,
            UIEventKind::KeyUp {
                key_code: _,
                modifiers: _,
            } => None,
            UIEventKind::CharInput { ch: _ } => None,
        }
    }
}
