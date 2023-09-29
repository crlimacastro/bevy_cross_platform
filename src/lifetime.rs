use bevy::prelude::*;
use std::time::Duration;

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lifetime_system);
    }
}

#[derive(Component)]
pub struct Lifetime {
    timer: Timer,
}

impl Lifetime {
    pub fn from_duration(duration: Duration) -> Self {
        Self {
            timer: Timer::from_seconds(duration.as_secs_f32(), TimerMode::Once),
        }
    }

    pub fn from_seconds(seconds: f32) -> Self {
        Lifetime::from_duration(Duration::from_secs_f32(seconds))
    }
}

pub fn lifetime_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
