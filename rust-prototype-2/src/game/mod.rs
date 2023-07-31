use crate::engine::action::{ChosenAction, PossibleActions};
use crate::engine::phase::Phase;
use bevy::prelude::*;

use crate::engine::CcgEnginePlugin;

mod abilities;
mod init;
mod print;
mod zone;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CcgEnginePlugin::<zone::GameZones>::new());
        app.init_resource::<Phase>();
        app.add_systems(Startup, init::setup);
        app.add_systems(PostUpdate, print::print_game.run_if(print::needs_print));
        app.add_systems(
            PostUpdate,
            print::process_input.run_if(resource_exists::<PossibleActions>()),
        );
        app.add_systems(PostStartup, abilities::spawn_abilities);
    }
}
