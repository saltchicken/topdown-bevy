use bevy::prelude::*;
use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(
                Update,
                (
                    player_movement,
                    player_animation_controller,
                    animate_sprite,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

// Holds the texture handles so we can dynamically swap them on the player entity
#[derive(Resource)]
struct PlayerAnimations {
    idle: Handle<Image>,
    walk: Handle<Image>,
}

#[derive(Component)]
struct Player {
    pub speed: f32,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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
    // Both 4x4 sprite sheets follow the exact same layout mappings
    fn indices(&self) -> (usize, usize) {
        match self {
            Self::IdleDown | Self::WalkDown => (0, 3),    // Row 0
            Self::IdleLeft | Self::WalkLeft => (4, 7),    // Row 1
            Self::IdleUp | Self::WalkUp => (8, 11),       // Row 2
            Self::IdleRight | Self::WalkRight => (12, 15),// Row 3
        }
    }

    // Helper function to figure out if we are currently walking
    fn is_walk(&self) -> bool {
        matches!(
            self,
            Self::WalkDown | Self::WalkLeft | Self::WalkUp | Self::WalkRight
        )
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Note: It is recommended to move camera setup to a main Startup system 
    // to prevent spawning multiple cameras if you re-enter this state.
    commands.spawn(Camera2dBundle::default());

    // Load both sheets
    let idle_texture = asset_server.load("player_idle.png");
    let walk_texture = asset_server.load("player_walk.png");

    // Since both sheets are 4x4, we only need to define one layout and can reuse it
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 4, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    // Save the image handles in a resource
    commands.insert_resource(PlayerAnimations {
        idle: idle_texture.clone(),
        walk: walk_texture,
    });

    // Spawn the player with the idle texture by default
    commands.spawn((
        SpriteBundle {
            texture: idle_texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        TextureAtlas {
            layout: layout_handle,
            index: 0,
        },
        Player { speed: 300.0 }, // Initialize player speed here
        PlayerAnimationState::IdleDown,
        AnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
    ));
}

// Logic Only: Reads inputs, updates Transform, and sets the intended PlayerAnimationState
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut PlayerAnimationState)>,
    time: Res<Time>,
) {
    let Ok((player, mut player_transform, mut animation_state)) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    // Default to idle based on current facing direction
    let mut new_state = match *animation_state {
        PlayerAnimationState::WalkDown | PlayerAnimationState::IdleDown => {
            PlayerAnimationState::IdleDown
        }
        PlayerAnimationState::WalkUp | PlayerAnimationState::IdleUp => PlayerAnimationState::IdleUp,
        PlayerAnimationState::WalkLeft | PlayerAnimationState::IdleLeft => {
            PlayerAnimationState::IdleLeft
        }
        PlayerAnimationState::WalkRight | PlayerAnimationState::IdleRight => {
            PlayerAnimationState::IdleRight
        }
    };

    // Check X inputs
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        new_state = PlayerAnimationState::WalkLeft;
    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        new_state = PlayerAnimationState::WalkRight;
    }

    // Check Y inputs
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
        new_state = PlayerAnimationState::WalkUp;
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
        new_state = PlayerAnimationState::WalkDown;
    }

    // Mutate the state only if it actually changed to prevent triggering Change Detection every frame
    if *animation_state != new_state {
        *animation_state = new_state;
    }

    // Apply movement
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    player_transform.translation += direction * player.speed * time.delta_seconds();
}

// Visuals Only: Listens for changes to the animation state and updates visual components
fn player_animation_controller(
    mut query: Query<
        (
            &PlayerAnimationState,
            &mut TextureAtlas,
            &mut AnimationTimer,
            &mut Handle<Image>,
        ),
        (With<Player>, Changed<PlayerAnimationState>),
    >,
    animations: Res<PlayerAnimations>,
) {
    for (state, mut atlas, mut timer, mut texture) in &mut query {
        // Swap out the underlying sprite sheet image if we are crossing action boundaries
        if state.is_walk() {
            *texture = animations.walk.clone();
        } else {
            *texture = animations.idle.clone();
        }

        // Snap to the correct starting frame for the new state
        atlas.index = state.indices().0;

        // Adjust animation speed based on the action
        let duration = match state {
            PlayerAnimationState::IdleDown
            | PlayerAnimationState::IdleLeft
            | PlayerAnimationState::IdleUp
            | PlayerAnimationState::IdleRight => 0.4,
            _ => 0.15,
        };
        timer.set_duration(std::time::Duration::from_secs_f32(duration));
    }
}

// Progresses the frames for whatever animation is currently playing
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&PlayerAnimationState, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (state, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let (start, end) = state.indices();

            if atlas.index < start || atlas.index >= end {
                atlas.index = start;
            } else {
                atlas.index += 1;
            }
        }
    }
}
