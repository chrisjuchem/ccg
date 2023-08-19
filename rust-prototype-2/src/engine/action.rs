use crate::engine::phase::Priority;
use bevy::ecs::system::BoxedSystem;
use bevy::prelude::*;

pub trait ChoiceGeneratorFn: System<In = (), Out = Vec<Action>> + dyn_clone::DynClone {}
dyn_clone::clone_trait_object!(ChoiceGeneratorFn);
impl<T> ChoiceGeneratorFn for T where T: System<In = (), Out = Vec<Action>> + dyn_clone::DynClone {}

#[derive(Component)]
pub struct ChoiceGenerator(pub Box<dyn ChoiceGeneratorFn>);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    PassPriority,
    PlayCard(Entity),

    Keep,
    Mulligan,
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

pub fn generate_choices(world: &mut World) {
    fn fetch_choice_gens(gens: Query<&ChoiceGenerator>) -> Vec<Box<dyn ChoiceGeneratorFn>> {
        return gens.iter().map(|g| g.0.clone()).collect();
    }

    let mut gens_fetch = IntoSystem::into_system(fetch_choice_gens);
    gens_fetch.initialize(world);
    let gens = gens_fetch.run((), world);

    let mut choices = vec![];
    gens.into_iter().for_each(|mut gen| {
        gen.initialize(world);
        choices.extend_from_slice(&gen.run((), world)); //todo convert to iters
    });

    world.insert_resource(PossibleActions(choices))
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
            Action::Keep => {
                println!("keeping")
            }
            _ => {}
        }
    } else {
        println!("Choice out of bounds, try again.")
    }
    commands.remove_resource::<ChosenAction>();
}
