use crate::resources::MapSize;
use crate::utils::constants::TILE_SIZE;
use bevy::prelude::*;
use bracket_lib::prelude::Point;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl From<Point> for Position {
    fn from(point: Point) -> Self {
        Self {
            x: point.x as u32,
            y: point.y as u32,
        }
    }
}

impl From<Position> for Point {
    fn from(pos: Position) -> Self {
        Self::new(pos.x as i32, pos.y as i32)
    }
}

impl From<Vec2> for Position {
    fn from(vec: Vec2) -> Self {
        Self {
            x: (vec.x / TILE_SIZE) as u32,
            y: (vec.y / TILE_SIZE) as u32,
        }
    }
}

impl From<Position> for Vec2 {
    fn from(pos: Position) -> Self {
        Self::new(pos.x as f32 * TILE_SIZE, pos.y as f32 * TILE_SIZE)
    }
}

impl From<Vec3> for Position {
    fn from(vec: Vec3) -> Self {
        Self {
            x: (vec.x / TILE_SIZE) as u32,
            y: (vec.y / TILE_SIZE) as u32,
        }
    }
}

impl From<Position> for Vec3 {
    fn from(pos: Position) -> Self {
        Self::new(pos.x as f32 * TILE_SIZE, pos.y as f32 * TILE_SIZE, 0.0)
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
