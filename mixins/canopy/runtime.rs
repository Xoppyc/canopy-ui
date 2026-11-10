//! Horizon Native Mod runtime for Canopy.
//! Bridges Horizon's `HudBatch` and `HudAtlas` to Canopy's `RenderBackend` and `TextMeasurer`,
//! and hosts the in-game Canopy Playground & Inspector overlay!

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

/// Called every frame from `hud.rs` when Canopy Playground is active.
/// TODO (Phase 2.5): Implement `HorizonBackend` wrapping `batch` + `atlas`,
/// build a demo `Container` tree, run `measure` -> `layout` -> `render`!
pub fn render_playground(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    let _ = (game, batch);
    // Once Phase 2 (Container in src/node.rs) is ready, we will wire this up
    // to render your live Canopy DOM tree directly onto the Minecraft HUD!
}
