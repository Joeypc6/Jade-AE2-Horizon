fn jade_ae2_register_server(game: &mut crate::game::Game<'_>) {
    if game.native_on("jade_ae2") {
        crate::game::jade_ae2::register();
    }
}
