use crate::systems::setup::*;
use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (setup_assets, setup_camera, setup_world).chain(),
        );
    }
}
