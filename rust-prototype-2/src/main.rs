use bevy::prelude::*;

mod engine;
mod game;
use game::GamePlugin;

fn main() {
    App::new().add_plugins((MinimalPlugins, GamePlugin)).run();
}
