use crate::core::state::{GameState, PauseState};
use crate::core::utils::despawn_screen;
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(PauseState::Paused), setup_pause_menu)
            .add_systems(
                Update,
                resume_button_system.run_if(in_state(PauseState::Paused)),
            )
            .add_systems(
                OnExit(PauseState::Paused),
                despawn_screen::<PauseMenuEntity>,
            );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 65.0;
const FONT_SIZE: f32 = 40.0;
const MENU_BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);

#[derive(Component)]
struct PauseMenuEntity;

fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.get() {
            PauseState::Running => next_state.set(PauseState::Paused),
            PauseState::Paused => next_state.set(PauseState::Running),
        }
    }
}

fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(MENU_BG_COLOR),
            PauseMenuEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: FONT_SIZE * 2.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(BUTTON_WIDTH),
                        height: Val::Px(BUTTON_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Resume"),
                        TextFont {
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn resume_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(PauseState::Running);
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
