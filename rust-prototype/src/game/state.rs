use crate::game::actions::Action;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Stack(pub Vec<Action>);
