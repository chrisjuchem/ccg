pub mod proto;
pub mod spawn;
pub mod text;
pub mod zone;

use bevy::prelude::*;
use proto::CardProto;

#[derive(Component)]
pub struct Card {
    proto: CardProto,
}
