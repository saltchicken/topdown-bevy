use bevy::prelude::*;

// 1. Define our Application States
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
}

// --- MAIN ---
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        
        // Main Menu Systems
        .add_systems(OnEnter(GameState::MainMenu), setup_menu)
        .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
        
        // In-Game Systems
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
        
        .run();
}

// --- UI / MAIN MENU ---
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// A marker component so we know which entities to destroy when leaving the menu
#[derive(Component)]
struct MenuEntity;

fn setup_menu(mut commands: Commands) {
    // We need a camera to see the UI
    commands.spawn((Camera2dBundle::default(), MenuEntity));

    // Spawn the UI Layout
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MenuEntity, // Tag the root node so we can clean it up later
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Game",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // Trigger the transition to the Playing state
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// Despawns the menu layout and menu camera when the state changes
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// --- TOP-DOWN GAME ---
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
