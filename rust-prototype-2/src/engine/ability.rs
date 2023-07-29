use crate::engine::event::{Filter, FilterableEvent, PhaseEndFilter, PhaseStart, PhaseStartFilter};
use crate::engine::phase::Phase;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ability<Filter> {
    pub trigger: Filter,
    // cost: () // todo
    // effect: Effect,
}

// pub enum Trigger {
//     Activation, //...
//     CardMoved,
// }
//
// pub enum Effect {
//     MoveCard {
//         to: Entity, //Zone
//     },
// }

//todo: rm
pub fn spawn_test_abilities(mut commands: Commands) {
    commands.spawn(Ability {
        trigger: PhaseStartFilter {
            phase_filter: Filter::Exact(Phase::Setup),
        },
    });
    commands.spawn(Ability {
        trigger: PhaseEndFilter {
            phase_filter: Filter::Exact(Phase::None),
        },
    });
}
