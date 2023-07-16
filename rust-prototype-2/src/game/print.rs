use crate::engine::phase::{ActivePlayer, Phase, Priority};
use crate::engine::{Card, Player};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct PrintArgs<'w, 's> {
    players: Query<'w, 's, (Ref<'static, Player>, Entity)>,
    cards: Query<'w, 's, Ref<'static, Card>>,
    phase: Res<'w, Phase>,
    priority: Res<'w, Priority>,
    active_player: Option<Res<'w, ActivePlayer>>,
}

pub fn needs_print(args: PrintArgs) -> bool {
    let debug = false;

    (args.phase.is_changed() && {
        if debug {
            println!("phase");
        }
        true
    }) || (args.priority.is_changed() && {
        if debug {
            println!("prio");
        }
        true
    }) || (args.active_player.is_some_and(|ap| ap.is_changed()) && {
        if debug {
            println!("active");
        }
        true
    }) || (args.players.iter().any(|(p, _)| p.is_changed()) && {
        if debug {
            println!("players");
        }
        true
    }) || (args.cards.iter().any(|c| c.is_changed()) && {
        if debug {
            println!("cards");
        }
        true
    })
}

pub fn print_game(args: PrintArgs) {
    let PrintArgs {
        players,
        cards,
        phase,
        priority,
        active_player,
    } = args;

    let player_name = |e: Entity| -> &str { players.get(e).unwrap().0.into_inner().name.as_str() };

    players.for_each(|(p, e)| {
        println!("{}:", p.name);
        println!(
            "{} HP, {} cards in deck",
            p.health,
            cards.iter().filter(|c| c.owner == Some(e)).count()
        )
    });
    if let Some(ap) = active_player {
        println!();
        println!("{}'s {:?} Phase", player_name(**ap), *phase);
    }
    if let Some(plr) = priority.current {
        println!();
        println!("{}, make a choice:", player_name(plr));
    }

    for _ in 0..20 {
        println!()
    }
}
