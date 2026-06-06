use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::state::{GameState, PauseState};
use crate::loading::GameAssets;
use crate::utils::despawn_screen;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_SCALE: f32 = 2.0;
const SPRITE_SIZE: u32 = 32;
const SPRITE_COLS: u32 = 4;
const SPRITE_ROWS: u32 = 4;
const IDLE_FRAME_DURATION: f32 = 0.4;
const WALK_FRAME_DURATION: f32 = 0.15;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<Player>)
            .add_systems(
                Update,
                (
                    player_movement,
                    player_animation_controller,
                    animate_sprite,
                )
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

#[derive(Component)]
struct Player {
    pub speed: f32,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
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
    game_assets: Res<GameAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(SPRITE_SIZE, SPRITE_SIZE), SPRITE_COLS, SPRITE_ROWS, None, None);
    let player_layout = texture_atlas_layouts.add(layout);

    // Spawn the player with the idle texture by default
    commands.spawn((
        SpriteBundle {
            texture: game_assets.player_idle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(PLAYER_SCALE)),
            ..default()
        },
        TextureAtlas {
            layout: player_layout,
            index: 0,
        },
        Player { speed: PLAYER_SPEED }, // Initialize player speed here
        PlayerAnimationState::IdleDown,
        AnimationTimer(Timer::from_seconds(IDLE_FRAME_DURATION, TimerMode::Repeating)),
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (PlayerAction::Move, VirtualDPad::wasd()),
                (PlayerAction::Move, VirtualDPad::arrow_keys()),
            ]),
        },
    ));
}

// Logic Only: Reads inputs, updates Transform, and sets the intended PlayerAnimationState
fn player_movement(
    mut query: Query<(
        &Player,
        &mut Transform,
        &mut PlayerAnimationState,
        &ActionState<PlayerAction>,
    )>,
    time: Res<Time>,
) {
    let Ok((player, mut player_transform, mut animation_state, action_state)) = query.get_single_mut() else {
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

    if let Some(axis) = action_state.clamped_axis_pair(&PlayerAction::Move) {
        direction = axis.xy().extend(0.0);
    }

    if direction != Vec3::ZERO {
        if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                new_state = PlayerAnimationState::WalkRight;
            } else {
                new_state = PlayerAnimationState::WalkLeft;
            }
        } else {
            if direction.y > 0.0 {
                new_state = PlayerAnimationState::WalkUp;
            } else {
                new_state = PlayerAnimationState::WalkDown;
            }
        }
    }

    // Mutate the state only if it actually changed to prevent triggering Change Detection every frame
    if *animation_state != new_state {
        *animation_state = new_state;
    }

    // Apply movement
    direction = direction.normalize_or_zero();

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
    animations: Res<GameAssets>,
) {
    for (state, mut atlas, mut timer, mut texture) in &mut query {
        // Swap out the underlying sprite sheet image if we are crossing action boundaries
        if state.is_walk() {
            *texture = animations.player_walk.clone();
        } else {
            *texture = animations.player_idle.clone();
        }

        // Snap to the correct starting frame for the new state
        atlas.index = state.indices().0;

        // Adjust animation speed based on the action
        let duration = match state {
            PlayerAnimationState::IdleDown
            | PlayerAnimationState::IdleLeft
            | PlayerAnimationState::IdleUp
            | PlayerAnimationState::IdleRight => IDLE_FRAME_DURATION,
            _ => WALK_FRAME_DURATION,
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
