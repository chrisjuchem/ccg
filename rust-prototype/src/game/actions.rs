use crate::game::card::object::zone::{InZone, Zone};
use crate::game::grab::Draggable;
use crate::game::state::Stack;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub enum Action {
    PlayCard(Entity),
}

pub fn take_action(action: Action, mut params: ActionParams) {
    match action {
        Action::PlayCard(entity) => {
            let (mut in_zone,) = params.cards.get_mut(entity).unwrap();
            in_zone.zone = Zone::Stack;
            in_zone.rel_order = params.stack.len() as i16;

            params.commands.entity(entity).remove::<Draggable>()
        }
    };

    params.stack.push(action)
}

pub fn resolve_action(inputs: Res<Input<KeyCode>>, mut params: ActionParams) {
    if !inputs.just_pressed(KeyCode::Space) {
        return;
    }

    let Some(action): Option<Action> = params.stack.pop() else { return };
    match action {
        Action::PlayCard(card) => {
            let n = params
                .cards
                .iter()
                .filter(|(in_zone,)| in_zone.zone == Zone::Battlefield)
                .count();
            let (mut in_zone,) = params.cards.get_mut(card).unwrap();
            in_zone.zone = Zone::Battlefield;
            in_zone.rel_order = n as i16;
        }
    }
}

#[derive(SystemParam)]
pub struct ActionParams<'w, 's> {
    stack: ResMut<'w, Stack>,
    cards: Query<'w, 's, (&'static mut InZone,)>,
    commands: Commands<'w, 's>,
}
