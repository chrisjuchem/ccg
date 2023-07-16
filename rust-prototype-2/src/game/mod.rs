use crate::engine::phase::Phase;
use bevy::prelude::*;

mod init;
mod print;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Phase>();
        app.add_systems(Startup, init::setup);
        app.add_systems(PostUpdate, print::print_game.run_if(print::needs_print));
    }
}
