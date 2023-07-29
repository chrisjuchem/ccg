use crate::engine::action::{needs_choices_generated, ChosenAction};
use crate::engine::phase::{Phase, Priority, TurnOrder};
use bevy::prelude::*;
use std::marker::PhantomData;

pub mod ability;
pub mod action;
pub mod event;
pub mod phase;
pub mod zone;
use zone::Zones;

pub struct CcgEnginePlugin<Z: Zones> {
    p: PhantomData<Z>,
}
impl<Z: Zones + 'static> CcgEnginePlugin<Z> {
    pub fn new() -> Self {
        Self { p: PhantomData }
    }
}

impl<Z: Zones + 'static> Plugin for CcgEnginePlugin<Z> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                phase::do_setup.run_if(phase::needs_setup),
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
        event::register_events::<Z>(app);
        app.add_systems(Startup, ability::spawn_test_abilities);
    }
}

// fn spawn_zones<Z: Zones + 'static>(mut commands: Commands) {
//     for z in Z::all() {
//         commands.spawn(Zone::new(z));
//     }
// }

#[derive(Component)]
pub struct Card<Z: Zones + 'static> {
    pub owner: Option<Entity>,
    pub zone: Option<Z>,
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
