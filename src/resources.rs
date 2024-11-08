use crate::components::{Position, Tile, TileSlotType};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_sprite: Handle<Image>,
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<Option<Tile>>,
    pub size: MapSize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MapSize {
    pub width: usize,
    pub height: usize,
}

impl MapSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn flatten(&self) -> usize {
        self.width * self.height
    }
}

impl Map {
    pub fn new(size: MapSize) -> Self {
        Self {
            tiles: vec![None; size.flatten()],
            size,
        }
    }

    pub fn insert_entity(&mut self, pos: Position, entity: Entity, slot_type: TileSlotType) {
        if let Some(tile) = self.get_tile_mut(pos.x as usize, pos.y as usize) {
            let _ = tile.add_entity_to_slot(slot_type, entity);
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.size.width && y < self.size.height {
            Some(y * self.size.width + x)
        } else {
            None
        }
    }

    fn index_unchecked(&self, x: usize, y: usize) -> usize {
        y * self.size.width + x
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.index(x, y).and_then(|i| self.tiles[i].as_ref())
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.index(x, y).and_then(|i| self.tiles[i].as_mut())
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if let Some(i) = self.index(x, y) {
            self.tiles[i] = Some(tile);
        }
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = &Option<Tile>> {
        self.tiles.iter()
    }

    pub fn iter_tiles_mut(&mut self) -> impl Iterator<Item = &mut Option<Tile>> {
        self.tiles.iter_mut()
    }
}
