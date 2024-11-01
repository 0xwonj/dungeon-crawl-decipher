use bevy::prelude::*;

/// Health with current and max fields
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// Xp
#[derive(Component)]
pub struct Xp(pub i32);

/// Marker for the player
#[derive(Component)]
pub struct Player;

/// Bundle to make it easy to spawn the player entity
/// with all the correct components:
#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub health: Health,
    pub xp: Xp,
    // including all the components from another bundle
    pub sprite: SpriteBundle,
}
