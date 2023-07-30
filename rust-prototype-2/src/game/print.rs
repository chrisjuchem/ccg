use crate::engine::action::{ChosenAction, PossibleActions};
use crate::engine::phase::{ActivePlayer, Phase, Priority};
use crate::engine::{Card, Player};
use crate::game::zone::GameZones;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct PrintArgs<'w, 's> {
    players: Query<'w, 's, (Ref<'static, Player>, Entity)>,
    cards: Query<'w, 's, Ref<'static, Card<GameZones>>>,
    phase: Res<'w, Phase>,
    priority: Res<'w, Priority>,
    active_player: Option<Res<'w, ActivePlayer>>,
    choices: Option<Res<'w, PossibleActions>>,
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
    }) || (args.choices.is_some_and(|choices| choices.is_changed()) && {
        if debug {
            println!("choices");
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
    // print!("{esc}[1J{esc}[1;1H", esc = 27 as char); // clear screen
    // print!("{}[2J", 27 as char); // clear screen
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear screen
    for _ in 0..10 {
        println!()
    }

    let PrintArgs {
        players,
        cards,
        phase,
        priority,
        active_player,
        choices,
    } = args;

    let player_name = |e: Entity| -> &str { players.get(e).unwrap().0.into_inner().name.as_str() };

    players.for_each(|(p, e)| {
        println!("{}:", p.name);
        println!(
            "{} HP, {} cards in deck, {} cards in hand",
            p.health,
            cards
                .iter()
                .filter(|c| c.owner == Some(e) && c.zone.unwrap() == GameZones::Deck)
                .count(),
            cards
                .iter()
                .filter(|c| c.owner == Some(e) && c.zone.unwrap() == GameZones::Hand)
                .count()
        )
    });
    if let Some(ap) = active_player {
        println!();
        println!("{}'s {:?} Phase", player_name(**ap), *phase);
    }
    if let Some(plr) = priority.current {
        println!();
        println!("{}, make a choice:", player_name(plr));
        for (i, choice) in choices.unwrap().iter().enumerate() {
            println!("{i}. {choice:?}")
        }
        println!();
    }
}

use std::io::Read;
use std::str::FromStr;
use std::time::Duration;
use timeout_readwrite::TimeoutReadExt;
pub fn process_input(mut commands: Commands) {
    let mut bytes = [0; 10];
    let _res = std::io::stdin()
        .with_timeout(Duration::from_millis(5))
        .read(&mut bytes)
        .map(|n| {
            std::str::from_utf8(&bytes[0..n])
                .map_err(|_| ())
                .and_then(|n| {
                    println!("Choice: {:?}", n);
                    usize::from_str(n.trim()).map_err(|_| ())
                })
                .map(|n| {
                    commands.insert_resource(ChosenAction(n));
                })
                .map_err(|_| println!("Bad choice, try again."))
        });
}
