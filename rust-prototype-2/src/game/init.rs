use bevy::prelude::*;

use crate::engine::{Card, Player};

pub fn setup(mut commands: Commands) {
    let p1 = commands.spawn(Player::new("Player1", 20)).id();
    let p2 = commands.spawn(Player::new("Player2", 20)).id();

    for _ in 0..5 {
        commands.spawn(Card { owner: Some(p1) });
        commands.spawn(Card { owner: Some(p2) });
    }
}
