use std::time::Duration;

use bevy::prelude::*;
pub struct UtilPlugin;
impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (lifetime_system, cooldown_system, visibility_timer_system),
        );
    }
}

#[derive(Component)]
pub struct LifeTime(Timer);
impl LifeTime {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct VisibilityTimer(Timer);
impl VisibilityTimer {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct Cooldown(Timer);

impl Cooldown {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }

    pub fn progress(&self) -> f32 {
        self.0.percent()
    }
}

fn lifetime_system(mut cmd: Commands, mut life_q: Query<(Entity, &mut LifeTime)>, time: Res<Time>) {
    life_q.iter_mut().for_each(|(ent, mut lifetime)| {
        lifetime.0.tick(time.delta()).finished().then(|| {
            cmd.entity(ent).despawn_recursive();
        });
    });
}

fn visibility_timer_system(
    mut cmd: Commands,
    mut visibility_q: Query<(Entity, &mut Visibility, &mut VisibilityTimer)>,
    time: Res<Time>,
) {
    visibility_q
        .iter_mut()
        .for_each(|(_ent, mut visibilty, mut timer)| {
            timer.0.tick(time.delta());

            match timer.0.finished() {
                true => {
                    timer.0.finished().then(|| {
                        *visibilty = Visibility::Hidden;
                    });
                    cmd.entity(_ent).remove::<VisibilityTimer>();
                }
                false => {
                    timer.0.finished().then(|| {
                        *visibilty = Visibility::Visible;
                    });
                }
                _ => {}
            }
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
