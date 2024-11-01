// src/systems/setup.rs
use crate::resources::*;
use bevy::prelude::*;

/// Setup system set for organizing initialization systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SetupSystem {
    LoadAssets,
    SpawnCamera,
    SpawnWorld,
}

/// Loads and initializes game assets
pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_assets = GameAssets {
        player_sprite: asset_server.load("sprites/player.png"),
        font: asset_server.load("fonts/PressStart2p-Regular.ttf"),
    };

    commands.insert_resource(game_assets);
}

/// Sets up the camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Sets up the game world
pub fn setup_world(mut commands: Commands) {
    // Spawn initial world entities
    // Set up any world-specific resources
}
