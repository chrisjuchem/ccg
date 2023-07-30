use crate::engine::zone::Zones;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum GameZones {
    Deck,
    Hand,
    Battlefield,
    Graveyard,
}

impl Zones for GameZones {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            GameZones::Deck,
            GameZones::Hand,
            GameZones::Battlefield,
            GameZones::Graveyard,
        ]
    }
}
