fn canopy_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::canopy_mod as m;
    if !game.native_on(m::ID) || !pressed {
        return;
    }
    let bound = game.native_mod_key(m::ID, m::KEY_TOGGLE, m::DEFAULT_KEY);
    if key == bound {
        m::toggle_playground();
    }
}
