use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;
mod utils;

use plugins::*;

fn main() {
    App::new()
        .add_plugins((SetupPlugin, PlayerPlugin, DefaultPlugins))
        .run();
}
