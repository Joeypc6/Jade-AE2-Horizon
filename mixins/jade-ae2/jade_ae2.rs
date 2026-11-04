// AE2's Jade plugin: AE2's own lines (waila.ae2.*) for its blocks, from the server's network state.
use crate::game::Game;
use crate::game::jade::{Line, Target};

pub fn register() {
    crate::game::jade::register_server(server_lines);
    crate::game::jade::register_client(client_lines);
    crate::game::jade::register_name(part_host_name);
}

/// `PartHostTooltips.getName` / `getIcon` / `getModName`: a cable bus shows the facade or part under the crosshair.
fn part_host_name(game: &Game<'_>, pos: [i32; 3], block: &str, hit: [f64; 3]) -> Option<(String, String, String)> {
    crate::game::port_ae2_me::part_host_name(game, pos, block, hit)
}

/// `GridNodeStateProvider`, `ChannelDataProvider`, `P2PStateDataProvider`, `ChargerDataProvider`,
/// `CraftingMonitorDataProvider`, `PowerStorageDataProvider`: what AE2 knows about the block.
fn server_lines(game: &mut Game<'_>, pos: [i32; 3], block: &str, lines: &mut Vec<String>) {
    if !block.starts_with("ae2:") {
        return;
    }
    lines.extend(crate::game::port_ae2_me::describe(game, pos));
}

/// A cell in the hand: nothing extra (its tooltip has it); kept for parts drawn on the client.
fn client_lines(_: &Game<'_>, _: &Target, _: &mut Vec<Line>) {}
