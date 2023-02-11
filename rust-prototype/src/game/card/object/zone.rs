use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_mod_picking::focus::HoverMap;
use std::cmp::Ordering;

#[derive(Default, Eq, PartialEq, Debug)]
pub enum Zone {
    #[default]
    Deck,
    Hand,
    Battlefield,
}

#[derive(Component, Debug)]
pub struct InZone {
    //todo maybe not pub?
    pub zone: Zone,
    pub rel_order: i16,
}

pub fn arrange_hand(
    mut all_cards: Query<(&InZone, &mut Transform, Entity)>,
    hover_map: Res<HoverMap>,
) {
    let mut cards: Vec<_> = all_cards
        .iter_mut()
        .filter(|(z, _t, _e)| z.zone == Zone::Hand) // todo filter ownership
        .collect();
    cards.sort_by(|(z1, _, _), (z2, _, _)| match z1.rel_order - z1.rel_order {
        0 => Ordering::Equal,
        x if x > 0 => Ordering::Greater,
        _ => Ordering::Less,
    });

    let spacing = 2.;
    let mut next_pos = Vec3 {
        x: -spacing / 2. * (cards.len() - 1) as f32,
        y: -4.,
        z: 6.,
    };
    let mut offset = Vec3 {
        x: spacing,
        y: 0.,
        z: 0.8,
    };

    let hovered = hover_map
        .0
        .values()
        .fold(HashSet::<Entity>::new(), |mut a, b| {
            a.extend(b);
            a
        });

    for (_z, mut t, e) in cards.into_iter() {
        t.translation = next_pos.clone();

        // dbg!(&hovered);
        // dbg!(&e);
        if hovered.contains(&e) {
            println!("flip!");
            offset.z *= -1.;
        }
        next_pos += offset
    }
}
