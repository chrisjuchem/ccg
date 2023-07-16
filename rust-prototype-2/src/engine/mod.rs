use crate::engine::action::{needs_choices_generated, ChosenAction};
use crate::engine::phase::{do_setup, Phase, Priority, TurnOrder};
use bevy::prelude::*;

pub mod action;
pub mod event;
pub mod phase;

pub struct CcgEnginePlugin;
impl Plugin for CcgEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                do_setup,
                phase::assign_priority.run_if(phase::current_priority_is_none),
                (
                    action::generate_choices.run_if(needs_choices_generated),
                    action::apply_choice.run_if(resource_exists::<ChosenAction>()),
                    apply_deferred
                        .run_if(resource_exists::<ChosenAction>().or_else(needs_choices_generated)),
                )
                    .chain(),
                phase::move_to_next_phase.run_if(phase::all_passed_priority),
            )
                .chain()
                .run_if(resource_exists::<Phase>()),
        );
        // app.init_resource::<Phase>();
        app.init_resource::<Priority>();
        app.init_resource::<TurnOrder>();
        event::register_events(app);
    }
}

#[derive(Component)]
pub struct Card {
    pub owner: Option<Entity>,
}

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub health: u32,
}
impl Player {
    pub fn new(name: impl ToString, health: u32) -> Self {
        Player {
            name: name.to_string(),
            health,
        }
    }
}
