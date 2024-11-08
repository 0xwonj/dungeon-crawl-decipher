use crate::resources::MapSize;
use crate::utils::constants::TILE_SIZE;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn to_translation(&self, map_size: MapSize) -> Vec3 {
        Vec3::new(
            (self.x as f32 - (map_size.width as f32 / 2.0)) * TILE_SIZE,
            (self.y as f32 - (map_size.height as f32 / 2.0)) * TILE_SIZE,
            0.0,
        )
    }
}

/// Health with current and max fields
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// Component to store the player's input state
#[derive(Component, Default)]
pub struct MovementInput {
    pub direction: Option<(i32, i32)>,
}

/// Xp
#[derive(Component)]
pub struct Xp(pub u32);

/// Marker for the player
#[derive(Component)]
pub struct Player;
