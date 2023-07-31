use crate::engine::ability::{Ability, Effect};
use crate::engine::event::{Filter, PhaseEndFilter, PhaseStartFilter};
use crate::engine::phase::Phase;
use crate::engine::{Card, Player};
use crate::game::zone::GameZones;
use bevy::prelude::*;

#[derive(Clone, Copy)]
struct DrawCard {
    player: Entity, // todo: player filter (one, each, vec)
}
impl Effect for DrawCard {
    fn resolver(&self) -> Box<dyn System<In = (), Out = ()>> {
        let this = self.clone();
        Box::new(IntoSystem::into_system(
            move |mut cards: Query<&mut Card<GameZones>>| {
                for mut card in &mut cards {
                    if card.owner == Some(this.player) {
                        if card.zone == Some(GameZones::Deck) {
                            card.zone.replace(GameZones::Hand);
                            return;
                        }
                    }
                }
            },
        ))
    }

    fn dyn_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}

const STARTING_HAND_SIZE: usize = 3;

pub fn spawn_abilities(mut commands: Commands, players: Query<Entity, With<Player>>) {
    commands.spawn(Ability {
        trigger: PhaseStartFilter {
            phase_filter: Filter::Exact(Phase::Setup),
        },
        effects: players
            .into_iter()
            .map::<Vec<Box<dyn Effect>>, _>(|p| {
                vec![Box::new(DrawCard { player: p }); STARTING_HAND_SIZE]
            })
            .flatten()
            .collect(),
    });
    commands.spawn(Ability {
        trigger: PhaseEndFilter {
            phase_filter: Filter::Exact(Phase::None),
        },
        effects: vec![],
    });
}
