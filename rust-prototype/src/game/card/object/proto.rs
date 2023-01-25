enum CardType {
    Creature,
}

trait Zone {}
enum Battlefield {}
impl Zone for Battlefield {}

//========

pub struct Cost {
    pub mana: u8,
}

pub struct CardProto {
    pub cost: Cost,
    pub abilities: Vec<Ability>,
    pub stats: Stats,
}

pub struct Stats {
    pub attack: u8,
    pub max_health: u8,
}

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

pub enum Trigger {
    Timing(Timing),
    Event {/*...*/},
}

pub enum StaticEffect {
    StatChange {
        attack: i8,
        health: i8,
        affects: CreatureTarget,
    },
}
pub enum InstantEffect {
    Damage { amount: usize, target: Target },
    // Temporary effect
}

pub enum Target {
    Creatures(CreatureTarget),
    Players(PlayerTarget),
    CreaturesAndPlayers {
        creatures: CreatureTarget,
        players: PlayerTarget,
    },
}
pub struct CreatureTarget {
    pub owner: Ownership,
    // filter:
}
pub struct PlayerTarget {
    pub owner: Ownership,
    // filter:
}

pub enum Ownership {
    Any,
    You,
    Opponent,
}

pub struct Timing {
    pub phase: Phase,
}
pub enum Phase {
    Upkeep,
    EoT,
}
