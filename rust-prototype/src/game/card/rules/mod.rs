use bevy::prelude::*;

trait EventType {
    type Player;
    type Card;
}


#[derive(Deref, Eq, PartialEq)]
struct PlayerRef(Entity);
#[derive(Deref, Eq, PartialEq)]
struct CardRef(Entity);



#[derive(Resource)]
struct Actions(pub Vec<Action>);

/// A choice a user can make during a game
enum Action {
    PlayCard(CardRef),
    PassPriority,
}

// #[derive(Event)]
enum GameEvent {
    Draw(PlayerRef),
    Played(CardRef),
    Etb(CardRef),
    Dies(CardRef),
}

#[derive(Component)]
struct Ability {
    trigger:
}
