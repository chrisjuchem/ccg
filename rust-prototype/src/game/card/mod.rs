pub mod object;

use crate::game::card::object::zone::{arrange_hand, Zone};
use bevy::prelude::*;
use object::proto::*;
use object::spawn::{init_card_assets, CardSpawner};
use object::text::update_text_meshes;

pub struct CardPlugin;
impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, init_card_assets);
        app.add_startup_system(testing_cards);
        app.add_system(update_text_meshes);

        // todo normal system && detection
        app.add_system(arrange_hand);
    }
}

fn testing_cards(mut spawner: CardSpawner) {
    let mut card = CardProto {
        cost: Cost { mana: 0 },
        abilities: vec![],
        stats: Stats {
            attack: 2,
            max_health: 5,
        },
    };

    spawner.spawn(card.clone(), Zone::Hand);
    card.cost.mana += 1;
    spawner.spawn(card.clone(), Zone::Hand);
    card.cost.mana += 1;
    spawner.spawn(card.clone(), Zone::Hand);
    card.cost.mana += 1;
    spawner.spawn(card.clone(), Zone::Hand);
    card.cost.mana += 1;
    spawner.spawn(card.clone(), Zone::Battlefield);
    card.cost.mana += 1;
    spawner.spawn(card.clone(), Zone::Battlefield);
    card.cost.mana += 1;
    spawner.spawn(card.clone(), Zone::Battlefield);
}
