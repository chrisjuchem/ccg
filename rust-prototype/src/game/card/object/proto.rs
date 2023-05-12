enum CardType {
    Creature,
}
//========

#[derive(Clone)]
pub struct Cost {
    pub mana: u8,
}

#[derive(Clone)]
pub struct CardProto {
    pub cost: Cost,
    pub abilities: Vec<Ability>,
    pub stats: Stats,
}

#[derive(Clone)]
pub struct Stats {
    pub attack: u8,
    pub max_health: u8,
}

#[derive(Clone)]
pub enum Ability {
    Static {
        effect: StaticEffect,
    },
    Temporary {
        effect: StaticEffect,
        until: Timing,
    },
    Instant {
        effect: InstantEffect,
    },
    Delayed {
        effect: InstantEffect,
        at: Timing,
    },
    Triggered {
        effect: InstantEffect,
        trigger: Trigger,
    },
}

#[derive(Clone)]
pub enum Trigger {
    Timing(Timing),
    Event {/*...*/},
}

#[derive(Clone)]
pub enum StaticEffect {
    StatChange {
        attack: i8,
        health: i8,
        affects: CreatureTarget,
    },
}
#[derive(Clone)]
pub enum InstantEffect {
    Damage { amount: usize, target: Target },
    // Temporary effect
}

#[derive(Clone)]
pub enum Target {
    Creatures(CreatureTarget),
    Players(PlayerTarget),
    CreaturesAndPlayers {
        creatures: CreatureTarget,
        players: PlayerTarget,
    },
}
#[derive(Clone)]
pub struct CreatureTarget {
    pub owner: Ownership,
    // filter:
    // count: N, up to N(??), all
    // choice: chosen, random
}
#[derive(Clone)]
pub struct PlayerTarget {
    pub owner: Ownership,
    // filter:
    // count: N, up to N(??), all
    // choice: chosen, random
}

#[derive(Clone)]
pub enum Ownership {
    Any,
    You,
    Opponent,
}

#[derive(Clone)]
pub struct Timing {
    pub phase: Phase,
}
#[derive(Clone)]
pub enum Phase {
    Upkeep,
    EoT,
}
