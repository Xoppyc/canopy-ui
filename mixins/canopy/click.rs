fn canopy_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    let _ = game;
    use crate::game::canopy_mod as m;
    if mod_id != m::ID {
        return;
    }
    match button {
        0 => {
            m::toggle_playground();
        }
        1 => {
            m::toggle_inspector();
        }
        _ => {}
    }
}
