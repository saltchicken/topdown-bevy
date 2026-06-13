use bevy::prelude::*;

#[derive(Event)]
pub struct GoldGained(pub u32);

#[derive(Component)]
pub struct CoinText;

#[derive(Resource, Default)]
pub struct PlayerWallet {
    pub coins: u32,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerWallet>()
            .add_systems(Startup, setup_ui)
            .add_observer(on_gold_gained);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Coins: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.8, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
        CoinText,
    ));
}

fn on_gold_gained(
    trigger: On<GoldGained>,
    mut wallet: ResMut<PlayerWallet>,
    mut text_query: Query<&mut Text, With<CoinText>>,
) {
    let event = trigger.event();
    wallet.coins += event.0;
    for mut text in &mut text_query {
        text.0 = format!("Coins: {}", wallet.coins);
    }
}
