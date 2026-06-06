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

// Holds the texture handles so we can dynamically swap them on the player entity
#[derive(Resource)]
struct PlayerAnimations {
    idle: Handle<Image>,
    walk: Handle<Image>,
}

#[derive(Component)]
struct Player;

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
    // Both 4x4 sprite sheets follow the exact same layout mappings now
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
        Player,
        PlayerAnimationState::IdleDown,
        AnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut PlayerAnimationState,
        &mut TextureAtlas,
        &mut AnimationTimer,
        &mut Handle<Image>, // Grab the image handle directly off the entity so we can mutate it
    ), With<Player>>,
    animations: Res<PlayerAnimations>,
    time: Res<Time>,
) {
    let Ok((mut player_transform, mut animation_state, mut atlas, mut timer, mut texture)) = query.get_single_mut() else {
        return;
    };

    let speed = 300.0;
    let mut direction = Vec3::ZERO;
    
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

    // Check Y inputs
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
        new_state = PlayerAnimationState::WalkUp;
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
        new_state = PlayerAnimationState::WalkDown;
    }

    if *animation_state != new_state {
        // Swap out the underlying sprite sheet image if we are crossing action boundaries
        if animation_state.is_walk() != new_state.is_walk() {
            if new_state.is_walk() {
                *texture = animations.walk.clone();
            } else {
                *texture = animations.idle.clone();
            }
        }

        *animation_state = new_state;
        atlas.index = new_state.indices().0;

        let duration = match new_state {
            PlayerAnimationState::IdleDown
            | PlayerAnimationState::IdleLeft
            | PlayerAnimationState::IdleUp
            | PlayerAnimationState::IdleRight => 0.4,
            _ => 0.15,
        };
        timer.set_duration(std::time::Duration::from_secs_f32(duration));
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    player_transform.translation += direction * speed * time.delta_seconds();
}

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
