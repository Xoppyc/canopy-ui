// Horizon Native Mod runtime for Canopy.
// Bridges Horizon's `HudBatch` and `HudAtlas` to Canopy's `RenderBackend` and `TextMeasurer`,
// and hosts the in-game Canopy Playground & Inspector overlay!

use crate::game::canopy::{
    hex, px, BoundingBox, Color, Component, Container, Edges, LayoutConstraints, MeasuredSize,
    RenderBackend, Style, TextMeasurer,
};
use std::cell::Cell;

pub const ID: &str = "canopy";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Declarative DOM GUI library & live UI Playground for Horizon";
pub const KEY_TOGGLE: &str = "toggle_playground";
pub const DEFAULT_KEY: &str = "KeyK";

thread_local! {
    static PLAYGROUND_OPEN: Cell<bool> = const { Cell::new(true) };
    static INSPECTOR_OPEN: Cell<bool> = const { Cell::new(false) };
}

pub fn is_playground_open() -> bool {
    PLAYGROUND_OPEN.with(|c| c.get())
}

struct HorizonBackend<'a, 'b> {
    game: &'a mut crate::game::Game<'b>,
    batch: &'a mut horizon_client::hud::HudBatch,
}

impl<'a, 'b> RenderBackend for HorizonBackend<'a, 'b> {
    fn fill_rect(&mut self, bounds: BoundingBox, color: Color) {
        // Horizon uses ARGB packed as u32: 0xAARRGGBB
        let argb = u32::from_be_bytes([color[3], color[0], color[1], color[2]]);
        self.game.native_fill(
            self.batch,
            [bounds.x, bounds.y, bounds.width, bounds.height],
            argb,
        );
    }

    fn draw_text(&mut self, x: f32, y: f32, text: &str, color: Color, _scale: f32) {
        // Horizon uses RGB packed as u32: 0x00RRGGBB
        let rgb = u32::from_be_bytes([0, color[0], color[1], color[2]]);
        self.game.native_text(self.batch, text, [x, y], rgb, true);
    }
}

pub fn toggle_playground() -> bool {
    PLAYGROUND_OPEN.with(|c| {
        let next = !c.get();
        c.set(next);
        next
    })
}

pub fn is_inspector_open() -> bool {
    INSPECTOR_OPEN.with(|c| c.get())
}

pub fn toggle_inspector() -> bool {
    INSPECTOR_OPEN.with(|c| {
        let next = !c.get();
        c.set(next);
        next
    })
}

struct HorizonMeasurer<'a, 'b> {
    game: &'a crate::game::Game<'b>,
}

impl<'a, 'b> TextMeasurer for HorizonMeasurer<'a, 'b> {
    fn measure_text(&self, text: &str, scale: f32) -> MeasuredSize {
        let width = self.game.native_text_width(text) * scale;
        let height = 9.0 * scale; // Standard Minecraft font height
        MeasuredSize::new(width, height)
    }
}

// Temporary harness for in-game HUD verification until CanopyRoot manages retained lifecycle.
pub(super) fn render_playground(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    let mut root = Container::new(
        Style::default()
            .with_width(px(320.0))
            .with_height(px(180.0))
            .with_bg(hex(0x181825EE))
            .with_border(1.5, hex(0x89B4FAFF))
            .with_padding(Edges::symmetric(16.0, 16.0)),
    );
    let measurer = HorizonMeasurer { game };
    root.measure(LayoutConstraints::new(1920.0, 1080.0), &measurer);
    root.layout(BoundingBox::new(40.0, 40.0, 320.0, 180.0));

    let mut backend = HorizonBackend { game, batch };
    root.render(&mut backend);
}
