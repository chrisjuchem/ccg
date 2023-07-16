use crate::engine::{event, Player};
use ::bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Resource, Deref, DerefMut)]
pub struct ActivePlayer(Entity);
#[derive(Resource, Deref, DerefMut, Default)]
pub struct TurnOrder(Vec<Entity>);
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Resource)]
pub enum Phase {
    #[default]
    None,
    Setup,
    Upkeep,
    Main,
    End,
}

impl Phase {
    pub fn next_phase(&self) -> Self {
        match self {
            Phase::None => Phase::Setup,
            Phase::Setup => Phase::Upkeep,
            Phase::Upkeep => Phase::Main,
            Phase::Main => Phase::End,
            Phase::End => Phase::Upkeep,
        }
    }
}

#[derive(Resource, Default)]
pub struct Priority {
    pub current: Option<Entity>,
    pub passed: HashSet<Entity>,
}

pub fn current_priority_is_none(priority: Res<Priority>) -> bool {
    priority.current.is_none()
}

pub fn assign_priority(mut priority: ResMut<Priority>, order: Res<TurnOrder>) {
    for player in order.iter() {
        if !priority.passed.contains(player) {
            priority.current = Some(*player);
            return;
        }
    }
}

pub fn all_passed_priority(priority: Res<Priority>, turn_order: Res<TurnOrder>) -> bool {
    priority.passed.len() == turn_order.len()
}

pub fn move_to_next_phase(
    mut priority: ResMut<Priority>,
    mut phase: ResMut<Phase>,
    mut ends: EventWriter<event::PhaseEnd>,
    mut starts: EventWriter<event::PhaseStart>,
) {
    priority.passed.clear();
    ends.send(event::PhaseEnd { phase: *phase });
    *phase = phase.next_phase();
    starts.send(event::PhaseStart { phase: *phase });
}

pub fn do_setup(
    mut start: EventReader<event::PhaseStart>,
    mut order: ResMut<TurnOrder>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    for _ in start.iter().filter(|ev| ev.phase == Phase::Setup) {
        let mut first = true;
        order.0 = players
            .iter()
            //.random()
            .map(|p| {
                if first {
                    commands.insert_resource(ActivePlayer(p));
                    first = false;
                }
                p
            })
            .collect();
    }
}
