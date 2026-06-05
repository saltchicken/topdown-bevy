use bevy::prelude::*;
use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct Player;

fn setup_game(mut commands: Commands) {
    // Spawn a fresh camera for the active game world
    commands.spawn(Camera2dBundle::default());

    // Spawn our player (a simple blue square)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.6, 0.8),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player, // Tag this entity as the player
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // Ensure we only try to move the player if they actually exist
    let Ok(mut player_transform) = query.get_single_mut() else {
        return;
    };

    let speed = 300.0;
    let mut direction = Vec3::ZERO;

    // Check inputs mapping to a 2D top-down grid
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    // Normalize so diagonal movement isn't twice as fast
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    // Apply movement delta
    player_transform.translation += direction * speed * time.delta_seconds();
}
