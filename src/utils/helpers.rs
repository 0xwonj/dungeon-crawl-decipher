use crate::components::entities::Position;
use bevy::prelude::*;

impl From<UVec2> for Position {
    fn from(pos: UVec2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<&Position> for UVec2 {
    fn from(pos: &Position) -> Self {
        UVec2::new(pos.x, pos.y)
    }
}

impl From<Position> for UVec2 {
    fn from(pos: Position) -> Self {
        UVec2::new(pos.x, pos.y)
    }
}
