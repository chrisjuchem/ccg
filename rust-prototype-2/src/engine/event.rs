use crate::engine::ability::Ability;
use bevy::prelude::*;

use crate::engine::phase::Phase;
use crate::engine::zone::Zones;

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct TriggerEvents;

pub fn register_events<Z: Zones + 'static>(app: &mut App) {
    register_event::<PhaseStart>(app);
    register_event::<PhaseEnd>(app);
    register_event::<CardMoved<Z>>(app);
}

fn register_event<E: FilterableEvent>(app: &mut App) {
    app.add_event::<E>();
    app.add_systems(Update, trigger_abilities::<E>.in_set(TriggerEvents));
}

pub trait FilterableEvent: Event {
    type Filter: Send + Sync; //todo, add filter trait, add bound to ability trigger
    fn matches(&self, filter: &Self::Filter) -> bool;
}

pub fn trigger_abilities<E: FilterableEvent>(
    mut evs: EventReader<E>,
    abilities: Query<&Ability<E::Filter>>,
) {
    for ev in evs.iter() {
        for ability in abilities.iter() {
            if ev.matches(&ability.trigger) {
                println!("Triggered {}!", std::any::type_name::<E>());
            }
        }
    }
}

#[derive(Event)]
pub struct PhaseStart {
    pub phase: Phase,
}
pub struct PhaseStartFilter {
    pub phase_filter: Filter<Phase>,
}
impl FilterableEvent for PhaseStart {
    type Filter = PhaseStartFilter;

    fn matches(&self, filter: &Self::Filter) -> bool {
        filter.phase_filter.matches(&self.phase)
    }
}

#[derive(Event)]
pub struct PhaseEnd {
    pub phase: Phase,
}
pub struct PhaseEndFilter {
    pub phase_filter: Filter<Phase>,
}
impl FilterableEvent for PhaseEnd {
    type Filter = PhaseEndFilter;

    fn matches(&self, filter: &Self::Filter) -> bool {
        filter.phase_filter.matches(&self.phase)
    }
}

#[derive(Event)]
pub struct CardMoved<Z: Zones + 'static> {
    pub card: Entity,
    pub from: Option<Z>,
    pub to: Z,
}
pub struct CardMovedFilter<Z: Zones + 'static> {
    pub card_filter: Filter<Entity>,
    pub from_filter: Filter<Option<Z>>,
    pub to_filter: Filter<Z>,
}
impl<Z: Zones + 'static> FilterableEvent for CardMoved<Z> {
    type Filter = CardMovedFilter<Z>;

    fn matches(&self, filter: &Self::Filter) -> bool {
        filter.card_filter.matches(&self.card)
            && filter.from_filter.matches(&self.from)
            && filter.to_filter.matches(&self.to)
    }
}

// pub struct CardMoved<D> where D: EventData<Z>, Z: Zones + 'static {
//     card: Entity,
//     from: Option<Z>,
//     to: D<Z>,
// }
//
// trait EventData<T> {}
//
// struct Instance<T> {
//     data: T,
// }
// impl<T> EventData<T> for Instance<T> {}
//
pub enum Filter<T> {
    Any,
    Exact(T),
    Inverse(Box<Filter<T>>),
    OneOf(Vec<Filter<T>>),
}
impl<T> Filter<T>
where
    T: PartialEq,
{
    pub fn matches(&self, t: &T) -> bool {
        match self {
            Filter::Any => true,
            Filter::Exact(inner) => inner == t,
            Filter::Inverse(inv) => !inv.matches(t),
            Filter::OneOf(choices) => choices.iter().any(|c| c.matches(t)),
        }
    }
}
// impl<T> EventData<T> for Filter<T> {}
//
// trait GameEvent<D: EventData<T>> {}
//
// impl<E, T> Event for E where E: GameEvent<Instance<T>> {}
