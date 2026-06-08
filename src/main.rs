mod core;
mod entities;
mod render;
mod ui;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use seldom_state::prelude::*;

use core::camera::CameraPlugin;
use core::input::InputPlugin;
use core::state::StatePlugin;
use entities::enemy::EnemyPlugin;
use entities::map::MapPlugin;
use entities::player::PlayerPlugin;
use render::y_sort::YSortPlugin;
use ui::loading::LoadingPlugin;
use ui::menu::MenuPlugin;
use ui::pause_menu::PauseMenuPlugin;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "topdown";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                title: WINDOW_TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            StatePlugin,
            CameraPlugin,
            InputPlugin,
            LoadingPlugin,
            MenuPlugin,
            PauseMenuPlugin,
            TilemapPlugin,
            MapPlugin,
            PlayerPlugin,
            EnemyPlugin,
            YSortPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            StateMachinePlugin::default(),
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}
