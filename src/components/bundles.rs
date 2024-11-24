use crate::components::{entities::*, fov::FieldOfView};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    pub position: Position,
    pub health: Health,
    pub xp: Xp,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub character_bundle: CharacterBundle,
    pub field_of_view: FieldOfView,
    pub movement_input: MovementInput,
    pub sprite_bundle: SpriteBundle,
}
