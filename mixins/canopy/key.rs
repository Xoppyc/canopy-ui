fn canopy_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::canopy_mod as m;
    if !game.native_on(m::ID) || !pressed {
        return;
    }
    if key == m::KEY_TOGGLE {
        let state = m::toggle_playground();
        log::info!("[canopy] Playground toggled: {state}");
    }
}
