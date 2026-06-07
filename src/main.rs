use avian2d::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use serde::Deserialize;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_TITLE: &str = "Physics Simulator Shell";

#[derive(Asset, TypePath, Debug, Deserialize, Clone)]
pub struct PhysicsConfig {
    pub gravity_y: f32,
    pub bounce: f32,
}

#[derive(Default, TypePath)]
pub struct ConfigLoader;

impl AssetLoader for ConfigLoader {
    type Asset = PhysicsConfig;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<PhysicsConfig>(&bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(Resource)]
struct ConfigHandle(Handle<PhysicsConfig>);

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
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .init_asset::<PhysicsConfig>()
        .init_asset_loader::<ConfigLoader>()
        .insert_resource(Gravity(Vec2::NEG_Y * 98.1))
        .add_systems(Startup, setup_physics_scene)
        .add_systems(Update, hot_reload_config)
        .run();
}

fn setup_physics_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let config_handle: Handle<PhysicsConfig> = asset_server.load("config.ron");
    commands.insert_resource(ConfigHandle(config_handle));

    // Static Ground Environment
    commands.spawn((
        Transform::from_xyz(0.0, -100.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(800.0, 20.0),
    ));

    // Dynamic Physics Object
    commands.spawn((
        Transform::from_xyz(0.0, 200.0, 0.0),
        RigidBody::Dynamic,
        Collider::circle(20.0),
        Restitution::new(0.8),
    ));
}

fn hot_reload_config(
    mut events: MessageReader<AssetEvent<PhysicsConfig>>,
    assets: Res<Assets<PhysicsConfig>>,
    mut gravity: ResMut<Gravity>,
    mut query: Query<&mut Restitution>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => {
                if let Some(config) = assets.get(*id) {
                    gravity.0 = Vec2::NEG_Y * config.gravity_y;
                    for mut restitution in query.iter_mut() {
                        restitution.coefficient = config.bounce;
                    }
                }
            }
            _ => {}
        }
    }
}
