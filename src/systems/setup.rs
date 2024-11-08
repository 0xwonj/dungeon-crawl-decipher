use crate::components::tile::*;
use crate::resources::*;
use crate::utils::constants::MAP_SIZE;
use bevy::prelude::*;

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
    // Define the map size
    let mut map = Map::new(MAP_SIZE);

    // Initialize the map tiles
    for y in 0..map.size.height {
        for x in 0..map.size.width {
            map.set_tile(x, y, Tile::new(TileType::Floor));
        }
    }

    // Insert the map as a resource in Bevy's ECS
    commands.insert_resource(map);
}
