enum CardType {
    Creature,
}

trait Zone {}
enum Battlefield {}
impl Zone for Battlefield {}

//========

struct Cost {
    mana: u8,
}

struct Card {
    cost: Cost,
    abilities: Vec<Ability>,
    stats: Stats,
}

struct Stats {
    attack: u8,
    max_health: u8,
}

enum Ability {
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

enum Trigger {
    Timing(Timing),
    Event {/*...*/},
}

enum StaticEffect {
    StatChange {
        attack: i8,
        health: i8,
        affects: CreatureTarget,
    },
}
enum InstantEffect {
    Damage { amount: usize, target: Target },
    // Temporary effect
}

enum Target {
    Creatures(CreatureTarget),
    Players(PlayerTarget),
    CreaturesAndPlayers {
        creatures: CreatureTarget,
        players: PlayerTarget,
    },
}
struct CreatureTarget {
    owner: Ownership,
    // filter:
}
struct PlayerTarget {
    owner: Ownership,
    // filter:
}

enum Ownership {
    Any,
    You,
    Opponent,
}

struct Timing {
    phase: Phase,
}
enum Phase {
    Upkeep,
    EoT,
}
