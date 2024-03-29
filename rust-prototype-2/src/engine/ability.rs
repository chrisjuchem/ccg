use bevy::ecs::component::TableStorage;
use bevy::prelude::*;
use std::any::Any;
use std::fmt::{Debug, Formatter};

// alternative to dyn:
//macro_rules! make_enum {
//     {$($var:tt),* } => { make_enum!{@ $($var $var),* }};
//     {@ $($var:ident $ty:ty),*} => {
//         enum E {
//             $($var($ty)),*
//         }
//     };
// }

#[derive(Component)]
pub struct Ability<Filter> {
    pub trigger: Filter,
    // cost: () // todo
    pub effects: Vec<Box<dyn Effect>>,
}

/// This is the thing that goes on the whiteboard.
/// https://magic.wizards.com/en/news/mtg-arena/on-whiteboards-naps-and-living-breakthrough
pub trait Effect: Send + Sync + Any {
    fn resolver(&self) -> Box<dyn System<In = (), Out = ()>>;
    fn dyn_clone(&self) -> Box<dyn Effect>;
}
impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}
impl Debug for dyn Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("dyn Effect{ ... }")
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Effects(Vec<Box<dyn Effect>>);

pub trait EffectModifier: Send + Sync {
    fn would_modify(&self, effect: &mut Box<dyn Effect>) -> bool;
    fn modify_effect(&self, effect: &mut Box<dyn Effect>);
}
impl Component for Box<dyn EffectModifier> {
    type Storage = TableStorage;
}

pub fn modify_effects(modifiers: Query<()>, mut effects: ResMut<Effects>) {
    for effect in effects.iter_mut() {}
}

pub fn resolve_effects(world: &mut World) {
    let mut effects = world.remove_resource::<Effects>().unwrap();
    for effect in effects.0.iter_mut() {
        let mut resolver = effect.resolver();
        resolver.initialize(world);
        resolver.run((), world)
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
