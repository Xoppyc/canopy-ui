fn canopy_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::canopy_mod as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("Canopy {}", m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[
            m::KEY_TOGGLE.into(),
            "Toggle Canopy Playground".into(),
            m::DEFAULT_KEY.into(),
        ]],
    });
}
