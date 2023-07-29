use bevy::prelude::*;

// #[derive(Component)]
// pub struct Zone {
//     pub zone: Box<dyn Zones>, //todo not pub
// }
//
// impl Zone {
//     pub fn new<Z: Zones + 'static>(zone: Z) -> Self {
//         Self {
//             zone: Box::new(zone),
//         }
//     }
// }

pub trait Zones: Send + Sync + Eq {
    fn all() -> Vec<Self>
    where
        Self: Sized;
}
