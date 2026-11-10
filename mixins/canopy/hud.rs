fn canopy_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    let _ = corners;
    use crate::game::canopy_mod as m;
    if game.native_on(m::ID) && m::is_playground_open() {
        m::render_playground(game, batch);
    }
}
