use crate::components::*;
use crate::resources::GameAssets;
use bevy::prelude::*;

/// Base movement speed in units per second
const PLAYER_SPEED: f32 = 300.0;

/// Component to store the player's movement velocity
#[derive(Component, Default)]
struct Velocity(Vec3);

/// Component to store the player's input state
#[derive(Component, Default)]
pub struct MovementInput {
    direction: Vec2,
}

/// Spawns the player entity with necessary components
pub fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        PlayerBundle {
            marker: Player,
            health: Health {
                current: 100,
                max: 125,
            },
            xp: Xp(0),
            sprite: SpriteBundle {
                texture: game_assets.player_sprite.clone(),
                transform: Transform {
                    translation: Vec3::ZERO,
                    scale: Vec3::new(0.1, 0.1, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        },
        MovementInput::default(),
        Velocity::default(),
    ));
}

/// System that handles keyboard input and updates movement input component
/// Runs in the Update schedule as input handling should be as responsive as possible
#[allow(clippy::type_complexity)]
pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementInput, With<Player>>,
) {
    let Ok(mut movement) = query.get_single_mut() else {
        return;
    };

    let direction = Vec2::new(
        get_axis_input(&keyboard, KeyCode::ArrowLeft, KeyCode::ArrowRight),
        get_axis_input(&keyboard, KeyCode::ArrowDown, KeyCode::ArrowUp),
    );

    movement.direction = direction.normalize_or_zero();
}

/// System that applies movement based on input
/// Runs in the FixedUpdate schedule for consistent physics
#[allow(clippy::type_complexity)]
pub fn apply_movement(
    mut query: Query<(&MovementInput, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let Ok((movement, mut transform)) = query.get_single_mut() else {
        return;
    };

    if movement.direction == Vec2::ZERO {
        return;
    }

    let movement_delta = movement.direction.extend(0.0) * PLAYER_SPEED * time.delta_seconds();
    transform.translation += movement_delta;
}

/// Helper function to get input axis value from keyboard
fn get_axis_input(
    keyboard: &Res<ButtonInput<KeyCode>>,
    negative: KeyCode,
    positive: KeyCode,
) -> f32 {
    let mut axis = 0.0;
    if keyboard.pressed(negative) {
        axis -= 1.0;
    }
    if keyboard.pressed(positive) {
        axis += 1.0;
    }
    axis
}
