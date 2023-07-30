use crate::engine::phase::Priority;
use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    PassPriority,
    PlayCard(Entity),
}

#[derive(Resource, Deref)]
pub struct PossibleActions(Vec<Action>); // todo add id to support networking
#[derive(Resource, Deref)]
pub struct ChosenAction(pub usize);

pub fn needs_choices_generated(
    choices: Option<Res<PossibleActions>>,
    priority: Option<Res<Priority>>,
) -> bool {
    priority.and_then(|pri| pri.current).is_some() && choices.is_none()
}

pub fn generate_choices(mut commands: Commands, priority: Res<Priority>) {
    let _cur_player = priority.current.unwrap();
    commands.insert_resource(PossibleActions(vec![Action::PassPriority]))
}

pub fn apply_choice(
    actions: Res<PossibleActions>,
    choice: Res<ChosenAction>,
    mut priority: ResMut<Priority>,
    mut commands: Commands,
) {
    if let Some(action) = actions.get(**choice) {
        match action {
            Action::PassPriority => {
                let current = priority.current.unwrap();
                priority.passed.insert(current);
                priority.current = None;
            }
            _ => {}
        }
    } else {
        println!("Choice out of bounds, try again.")
    }
    commands.remove_resource::<ChosenAction>();
}
