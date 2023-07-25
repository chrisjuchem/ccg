use bevy::prelude::*;

#[derive(Component)]
pub struct Zone {
    zone: Box<dyn Zones>,
}

impl Zone {
    pub fn new<Z: Zones + 'static>(zone: Z) -> Self {
        Self {
            zone: Box::new(zone),
        }
    }
}

pub trait Zones: Send + Sync {
    fn all() -> Vec<Self>
    where
        Self: Sized;
}
