use crate::systems::player::*;
use crate::systems::setup::SetupSystem;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player.after(SetupSystem::LoadAssets))
            // Input handling in Update schedule for responsiveness
            .add_systems(Update, handle_input)
            // Movement application in FixedUpdate for consistency
            .add_systems(FixedUpdate, apply_movement);
    }
}
