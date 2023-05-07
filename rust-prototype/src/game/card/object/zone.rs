use crate::game::grab::Hovered;
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

pub fn arrange_hand(mut all_cards: Query<(&InZone, &mut Transform, Entity, Option<&Hovered>)>) {
    let mut cards: Vec<_> = all_cards
        .iter_mut()
        .filter(|(z, _t, _e, _h)| z.zone == Zone::Hand) // todo filter ownership
        .collect();
    cards.sort_by(
        |(z1, _, _, _), (z2, _, _, _)| match z1.rel_order - z2.rel_order {
            0 => Ordering::Equal,
            x if x > 0 => Ordering::Greater,
            _ => Ordering::Less,
        },
    );

    let z = 6.;
    let base_pt = Vec3::new(0., -30., z);
    let mut base_tf = Transform::from_xyz(0., -8., z);
    let angle = 0.5; //radians
    let n = cards.len() as f32;

    let mut i = 0;
    for (_z, mut t, e, h) in cards.into_iter() {
        base_tf.translation.z -= 0.2;
        *t = base_tf.clone();
        t.rotate_around(
            base_pt,
            Quat::from_rotation_z(angle * ((i as f32 / (1. - n)) + 0.5)),
        );
        t.scale = Vec3::new(1., 1., 1.);
        i += 1;

        if h.is_some() {
            t.scale *= 1.2;
            let offset = (t.translation - base_pt).normalize() + Vec3::new(0., 0., 0.5);
            t.translation += offset;
        }
    }
}
