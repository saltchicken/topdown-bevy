use bevy::prelude::*;
use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(
                Update,
                (player_movement, animate_sprite).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct Player;

// Tracks the timing for frame updates
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// Tracks the current animation state
#[derive(Component, PartialEq, Clone, Copy)]
enum PlayerAnimationState {
    IdleDown,
    IdleLeft,
    IdleUp,
    IdleRight,
    WalkDown,
    WalkLeft,
    WalkUp,
    WalkRight,
}

impl PlayerAnimationState {
    // Defines the (start_index, end_index) based on our 4x8 Action-Sorted rows
    fn indices(&self) -> (usize, usize) {
        match self {
            // -- IDLE ANIMATIONS --
            Self::IdleDown => (0, 3),    // Row 0
            Self::IdleLeft => (4, 7),    // Row 1
            Self::IdleUp => (8, 11),     // Row 2
            Self::IdleRight => (12, 15), // Row 3
            
            // -- WALK ANIMATIONS --
            Self::WalkDown => (16, 19),  // Row 4
            Self::WalkLeft => (20, 23),  // Row 5
            Self::WalkUp => (24, 27),    // Row 6
            Self::WalkRight => (28, 31), // Row 7
        }
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn a fresh camera for the active game world
    commands.spawn(Camera2dBundle::default());

    // Load the sprite sheet from the `assets` folder
    let texture = asset_server.load("player_spritesheet.png");

    // Define the layout of the new 4x8 action-sorted sprite sheet
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Spawn our player
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)), // Scaled 2x for visibility
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        Player,
        PlayerAnimationState::IdleDown, // Default starting state
        // Slightly slower timer (0.15s) so the breathing isn't overly frantic
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerAnimationState, &mut TextureAtlas), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut player_transform, mut animation_state, mut atlas)) = query.get_single_mut() else {
        return;
    };

    let speed = 300.0;
    let mut direction = Vec3::ZERO;
    
    // Default the new_state to the idle animation of whatever direction we are currently facing
    let mut new_state = match *animation_state {
        PlayerAnimationState::WalkDown | PlayerAnimationState::IdleDown => PlayerAnimationState::IdleDown,
        PlayerAnimationState::WalkUp | PlayerAnimationState::IdleUp => PlayerAnimationState::IdleUp,
        PlayerAnimationState::WalkLeft | PlayerAnimationState::IdleLeft => PlayerAnimationState::IdleLeft,
        PlayerAnimationState::WalkRight | PlayerAnimationState::IdleRight => PlayerAnimationState::IdleRight,
    };

    // Check X inputs
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        new_state = PlayerAnimationState::WalkLeft;
    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        new_state = PlayerAnimationState::WalkRight;
    }

    // Check Y inputs (Evaluated after X so Up/Down animation takes priority on diagonal movement)
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
        new_state = PlayerAnimationState::WalkUp;
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
        new_state = PlayerAnimationState::WalkDown;
    }

    // If the animation state changed, immediately update it and snap to the first frame of that cycle
    if *animation_state != new_state {
        *animation_state = new_state;
        atlas.index = new_state.indices().0;
    }

    // Normalize so diagonal movement isn't twice as fast
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    // Apply movement delta
    player_transform.translation += direction * speed * time.delta_seconds();
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&PlayerAnimationState, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (state, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        
        // When the timer goes off, advance the frame
        if timer.just_finished() {
            let (start, end) = state.indices();
            
            // If we've reached the end of the animation bounds, loop back to the start
            if atlas.index < start || atlas.index >= end {
                atlas.index = start;
            } else {
                atlas.index += 1;
            }
        }
    }
}
