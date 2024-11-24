use crate::systems::{fov::*, player::*};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            // Input handling in Update schedule for responsiveness
            .add_systems(Update, (handle_input, apply_movement, fov));
    }
}
