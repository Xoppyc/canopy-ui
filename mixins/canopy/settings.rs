fn canopy_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    let _ = game;
    use crate::game::canopy_mod as m;
    if mod_id != m::ID {
        return;
    }
    let pg = if m::is_playground_open() { "ON" } else { "OFF" };
    let insp = if m::is_inspector_open() { "ON" } else { "OFF" };
    buttons.push(format!("Playground Overlay: {pg}"));
    buttons.push(format!("DOM Inspector: {insp}"));
}
