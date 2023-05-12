#![feature(let_else)]

mod engine;
mod game;

use bevy::prelude::*;
use engine::EnginePlugin;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugin(EnginePlugin)
        .add_plugin(GamePlugin)
        .run();
}
