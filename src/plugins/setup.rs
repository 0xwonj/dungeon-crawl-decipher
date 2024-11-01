use crate::systems::setup::*;
use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (
                SetupSystem::LoadAssets,
                SetupSystem::SpawnCamera,
                SetupSystem::SpawnWorld,
            )
                .chain(),
        )
        .add_systems(
            Startup,
            ((
                setup_assets.in_set(SetupSystem::LoadAssets),
                setup_camera,
                setup_world,
            )
                .chain()),
        );
    }
}
