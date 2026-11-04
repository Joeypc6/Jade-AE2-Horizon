fn jade_ae2_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "jade_ae2".into(),
        title: "Jade AE2 Plugin 15.4.12".into(),
        description: "AE2's network, channel, P2P, charger and power lines in Jade.".into(),
        keys: Vec::new(),
    });
}
