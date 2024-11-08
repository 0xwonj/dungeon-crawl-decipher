use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub slots: Option<TileSlots>,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            slots: None,
        }
    }

    pub fn add_entity_to_slot(
        &mut self,
        slot_type: TileSlotType,
        entity: Entity,
    ) -> Result<(), &str> {
        if self.slots.is_none() {
            self.slots = Some(TileSlots::new());
        }

        let slots = self.slots.as_mut().unwrap();

        match slot_type {
            TileSlotType::Character => slots.character.add_entity(entity),
            TileSlotType::Item => slots.items.add_entity(entity),
            TileSlotType::Underground => slots.underground.add_entity(entity),
            TileSlotType::Aboveground => slots.aboveground.add_entity(entity),
        }
    }
}

#[derive(Component, Clone)]
pub struct TileSlots {
    pub character: TileSlot<8>,
    pub underground: TileSlot<8>,
    pub aboveground: TileSlot<8>,
    pub items: TileSlot<64>,
}

impl TileSlots {
    pub fn new() -> Self {
        Self {
            character: TileSlot::new(TileSlotType::Character),
            items: TileSlot::new(TileSlotType::Item),
            underground: TileSlot::new(TileSlotType::Underground),
            aboveground: TileSlot::new(TileSlotType::Aboveground),
        }
    }
}

#[derive(Component, Clone)]
pub struct TileSlot<const CAPACITY: usize> {
    pub slot_type: TileSlotType,
    pub entity_count: usize,
    pub entities: [Option<Entity>; CAPACITY],
}

impl<const CAPACITY: usize> TileSlot<CAPACITY> {
    pub fn new(slot_type: TileSlotType) -> Self {
        Self {
            slot_type,
            entity_count: 0,
            entities: [None; CAPACITY],
        }
    }

    /// Adds an entity if the slot has space
    pub fn add_entity(&mut self, entity: Entity) -> Result<(), &str> {
        if self.entity_count < CAPACITY {
            self.entities[self.entity_count] = Some(entity);
            self.entity_count += 1;
            Ok(())
        } else {
            Err("Slot is full")
        }
    }

    /// Removes an entity if it exists in the slot
    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), &str> {
        if let Some(index) = self
            .entities
            .iter()
            .position(|e| e.as_ref() == Some(&entity))
        {
            self.entities[index] = None;
            self.entity_count -= 1;
            Ok(())
        } else {
            Err("Entity not found in slot")
        }
    }

    /// Clears all entities in the slot
    pub fn clear(&mut self) {
        self.entities.fill(None);
        self.entity_count = 0;
    }

    /// Returns the number of entities in the slot
    pub fn entity_count(&self) -> usize {
        self.entities.iter().filter(|e| e.is_some()).count()
    }

    /// Checks if the slot is empty
    pub fn is_empty(&self) -> bool {
        self.entity_count == 0
    }

    /// Checks if the slot is full
    pub fn is_full(&self) -> bool {
        self.entity_count >= CAPACITY
    }
}

#[derive(Component, Clone)]
pub enum TileSlotType {
    Character,
    Item,
    Underground,
    Aboveground,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
}
