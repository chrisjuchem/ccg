use bevy::ecs::system::BoxedSystem;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct Ability<Filter> {
    pub trigger: Filter,
    // cost: () // todo
    pub effect: Effect,
}

#[derive(DerefMut, Deref, Clone)]
pub struct Effect(Arc<Mutex<BoxedSystem>>);
impl Effect {
    pub fn new<T: IntoSystem<(), (), M>, M>(system: T) -> Self {
        Effect(Arc::new(Mutex::new(Box::new(IntoSystem::into_system(
            system,
        )))))
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Effects(Vec<Effect>);

pub fn resolve_effects(world: &mut World) {
    let mut effects = world.remove_resource::<Effects>().unwrap();
    for mut effect in effects.0.iter_mut() {
        effect.lock().unwrap().initialize(world); //todo only init once
        effect.lock().unwrap().run((), world)
    }
    world.init_resource::<Effects>();
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

// pub trait IntoEffect<Marker>: IntoSystem<(), (), Marker> {
//     fn into(self) -> BoxedSystem {
//         Box::new(IntoSystem::into_system(self))
//     }
// }
// impl<T, Marker> IntoEffect<Marker> for T where T: IntoSystem<(), (), Marker> {}

// impl<T, M> From<T> for Effect
// where
//     T: IntoSystem<(), (), M>,
// {
//     fn from(cloneable_sys: T) -> Self {
//         Effect(Box::new(IntoSystem::into_system(cloneable_sys)))
//     }
// }
