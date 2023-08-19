use crate::engine::action::{Action, ChoiceGenerator};
use crate::engine::phase::Priority;
use crate::engine::Player;
use bevy::prelude::*;

#[derive(Component)]
struct Mulligans {
    n: u8,
    finished: bool,
}

pub fn spawn_rules(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for p in &players {
        commands.entity(p).insert(Mulligans {
            n: 0,
            finished: false,
        });
    }

    commands.spawn(ChoiceGenerator(Box::new(IntoSystem::into_system(
        mulligan_choice_generator,
    ))));
}

fn mulligan_choice_generator(
    priority: Res<Priority>,
    players: Query<&Mulligans, With<Player>>,
) -> Vec<Action> {
    let Some(cur_player) = priority.current else { return vec![]; };
    let m = players.get(cur_player).unwrap();
    if m.finished {
        return vec![Action::PassPriority]; // todo remove
    }

    return vec![Action::Keep, Action::Mulligan];
}
