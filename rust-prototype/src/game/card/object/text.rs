use crate::game::card::object::Card;
use ::bevy::prelude::*;
use bevy_text_mesh::TextMesh;

impl Card {
    fn text(&self) -> String {
        let mana_cost = self.proto.cost.mana;
        format!(
            "[ {mana_cost} ]
some longer strings in my wonderful game"
        )
    }
}

pub fn update_text_meshes(cards: Query<&Card>, mut meshes: Query<(&mut TextMesh, &Parent)>) {
    for (mut mesh, parent) in &mut meshes {
        let card = cards.get(parent.get()).unwrap();

        let new_text = card.text();
        if mesh.as_ref().text != new_text {
            mesh.text = new_text
        }
    }
}
