use bevy::prelude::*;

mod engine;
mod game;
use engine::CcgEnginePlugin;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, CcgEnginePlugin, GamePlugin))
        .run();
}
