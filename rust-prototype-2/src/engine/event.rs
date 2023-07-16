use bevy::prelude::*;

use crate::engine::phase::Phase;

pub fn register_events(app: &mut App) {
    app.add_event::<PhaseStart>();
    app.add_event::<PhaseEnd>();
}

#[derive(Event)]
pub struct PhaseStart {
    pub phase: Phase,
}

#[derive(Event)]
pub struct PhaseEnd {
    pub phase: Phase,
}
