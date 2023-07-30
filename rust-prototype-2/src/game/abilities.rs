use crate::engine::ability::{Ability, Effect};
use crate::engine::event::{Filter, PhaseEndFilter, PhaseStartFilter};
use crate::engine::phase::Phase;
use crate::engine::zone::Zones;
use crate::engine::Card;
use crate::game::zone::GameZones;
use bevy::prelude::*;
use bevy::utils::HashMap;

fn nothing() {}

fn move_n_everyone<Z: Zones>(n: usize, from: Z, to: Z) -> impl FnMut(Query<&mut Card<Z>>) {
    move |mut cards: Query<&mut Card<Z>>| {
        let mut counts = HashMap::<Entity, usize>::new();
        for mut card in &mut cards {
            if let Some(p) = card.owner {
                if card.zone == Some(from) {
                    let count = counts.entry(p).or_insert(0);
                    if *count < n {
                        *count += 1;
                        card.zone.replace(to);
                    }
                }
            }
        }
    }
}

pub fn spawn_abilities(mut commands: Commands) {
    commands.spawn(Ability {
        trigger: PhaseStartFilter {
            phase_filter: Filter::Exact(Phase::Setup),
        },
        effect: Effect::new(nothing),
    });
    commands.spawn(Ability {
        trigger: PhaseEndFilter {
            phase_filter: Filter::Exact(Phase::None),
        },
        effect: Effect::new(move_n_everyone(3, GameZones::Deck, GameZones::Hand)),
    });
}
