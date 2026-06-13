use bevy::prelude::*;

#[derive(Component)]
pub struct DashTrail {
    pub timer: Timer,
}

#[derive(Component)]
pub struct CameraShake {
    pub intensity: f32,
    pub timer: Timer,
}

#[derive(Component)]
pub struct FloatingText {
    pub velocity: Vec2,
    pub timer: Timer,
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_dash_trails, update_floating_text, update_camera_shake));
    }
}

fn update_dash_trails(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DashTrail, &mut Sprite)>,
) {
    for (entity, mut trail, mut sprite) in &mut query {
        trail.timer.tick(time.delta());
        let percent = trail.timer.fraction_remaining();
        sprite.color = sprite.color.with_alpha(percent);

        if trail.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn update_floating_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FloatingText, &mut Transform, &mut TextColor)>,
) {
    for (entity, mut text, mut transform, mut color) in &mut query {
        text.timer.tick(time.delta());
        transform.translation += text.velocity.extend(0.0) * time.delta_secs();

        let percent = text.timer.fraction_remaining();
        color.0 = color.0.with_alpha(percent);

        if text.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn update_camera_shake(time: Res<Time>, mut query: Query<&mut CameraShake>) {
    for mut shake in &mut query {
        shake.timer.tick(time.delta());
    }
}
