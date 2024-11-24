use crate::components::*;
use crate::resources::{GameAssets, Map};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>, mut map: ResMut<Map>) {
    let player_position = Position::new(0, 0);

    let player = commands
        .spawn(PlayerBundle {
            marker: Player,
            character_bundle: CharacterBundle {
                position: player_position,
                health: Health {
                    current: 100,
                    max: 100,
                },
                xp: Xp(0),
            },
            field_of_view: FieldOfView::new(10),
            movement_input: MovementInput { direction: None },
            sprite_bundle: SpriteBundle {
                texture: game_assets.player_sprite.clone(),
                transform: Transform::from_translation(player_position.into()),
                ..Default::default()
            },
        })
        .id();

    map.insert_entity(player_position, player, TileSlotType::Character);
}

pub fn handle_input(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementInput>,
) {
    let mut player_input = query.single_mut();

    let key = keyboard_input.get_pressed().next();

    if let Some(key) = key {
        match key {
            KeyCode::ArrowUp => {
                player_input.direction = Some((0, 1));
            }
            KeyCode::ArrowDown => {
                player_input.direction = Some((0, -1));
            }
            KeyCode::ArrowLeft => {
                player_input.direction = Some((-1, 0));
            }
            KeyCode::ArrowRight => {
                player_input.direction = Some((1, 0));
            }
            _ => {}
        }
    } else {
        player_input.direction = None;
    }

    keyboard_input.reset_all();
}

pub fn apply_movement(
    mut query: Query<
        (
            &mut Position,
            &mut Transform,
            &MovementInput,
            &mut FieldOfView,
        ),
        With<Player>,
    >,
    map: Res<Map>,
) {
    let (mut position, mut transform, player_input, mut fov) = query.single_mut();

    if let Some((dx, dy)) = player_input.direction {
        let target_x = position.x as i32 + dx;
        let target_y = position.y as i32 + dy;

        // TODO: Check if target position is within the map bounds

        if let Some(tile) = map.get_tile(target_x as usize, target_y as usize) {
            if tile.tile_type == TileType::Floor {
                position.x = target_x as u32;
                position.y = target_y as u32;

                transform.translation = (*position).into();
                fov.is_dirty = true;
            }
        }
    }
    // println!("Player position: {:?}", position);
    // println!("Player transform: {:?}", transform);
}
