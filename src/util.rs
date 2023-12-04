use std::time::Duration;

use bevy::prelude::*;
pub struct UtilPlugin;
impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (lifetime_system, cooldown_system));
    }
}

#[derive(Component)]
pub struct LifeTime(Timer);

impl LifeTime {
    pub fn new(time: f32) -> Self {
        Self(Timer::from_seconds(time, TimerMode::Once))
    }
    pub fn remaining(&self) -> std::time::Duration {
        self.0.remaining()
    }
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct Cooldown(Timer);

impl Cooldown {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

fn lifetime_system(mut cmd: Commands, mut life_q: Query<(Entity, &mut LifeTime)>, time: Res<Time>) {
    life_q.iter_mut().for_each(|(ent, mut lifetime)| {
        lifetime.0.tick(time.delta()).finished().then(|| {
            cmd.entity(ent).despawn_recursive();
        });
    });
}

fn cooldown_system(
    mut cmd: Commands,
    mut cooldown_q: Query<(Entity, &mut Cooldown)>,
    time: Res<Time>,
) {
    cooldown_q.iter_mut().for_each(|(ent, mut cd)| {
        cd.0.tick(time.delta());
        cd.0.finished().then(|| {
            cmd.entity(ent).remove::<Cooldown>();
        });
    });
}

pub fn quad_formation(count: usize, padding: f32) -> Vec<Vec3> {
    let extend = (count as f32).sqrt() as usize;
    let mut positions = Vec::with_capacity(count);
    for i in 0..count {
        let x = (i % extend) as f32 * padding;
        let y = (i / extend) as f32 * padding;
        positions.push(Vec3::new(x, y, 0.));
    }
    positions
}
