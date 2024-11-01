use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_sprite: Handle<Image>,
    pub font: Handle<Font>,
}
