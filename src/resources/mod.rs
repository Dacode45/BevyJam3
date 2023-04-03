use bevy::prelude::*;

#[derive(Default, Clone, Resource)]
pub struct PlayerHand {
    cards: Vec<Card>,
}

#[derive(Clone, Component)]
pub struct PlayerOwned;

#[derive(Clone, Component)]
pub struct EnemyOwned;

#[derive(Default, Clone, Component)]
pub struct Card {}

impl Card {}
