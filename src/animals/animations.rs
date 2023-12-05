use bevy::{gltf::Gltf, prelude::*};
use bevy_rapier2d::dynamics::Velocity;



pub struct AnimalAnimationPlugin;
impl Plugin for AnimalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (switch_animation, update_state));
    }
}

#[derive(Component, Default)]
pub enum AnimalState {
    #[default]
    Idle,
    Dead,
    Running,
    Walking,
    Jumping,
}

impl AnimalState {
    fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }
    fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
    fn is_jumping(&self) -> bool {
        matches!(self, Self::Jumping)
    }
    fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    pub fn animation_name(&self) -> &str {
        match self {
            Self::Idle => "Idle",
            Self::Dead => "Death",
            Self::Walking => "Walk",
            Self::Running => "Run",
            Self::Jumping => "Jump",
        }
    }
}

fn update_state(mut query: Query<(&Velocity, &mut AnimalState)>) {
    query.iter_mut().for_each(|(velocity, mut state)| {
        if state.is_dead() {
            return;
        }

        let speed = velocity.linvel.length();

        if speed > 10. {
            *state = AnimalState::Running;
            return;
        }

        if speed > 1.0 {
            *state = AnimalState::Walking;
            return;
        }

        *state = AnimalState::Idle;
    });
}

fn switch_animation(
    mut animation_player: Query<&mut AnimationPlayer>,
    query: Query<(Entity, &Handle<Gltf>, &AnimalState), Changed<AnimalState>>,
    children: Query<&Children>,
    _server: Res<AssetServer>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    query.iter().for_each(|(entity, gltf, state)| {
        let Some(gltf) = gltf_assets.get(gltf) else {
            return;
        };

        let Some(ent) = find_child_rec(entity, &children, &animation_player.to_readonly()) else {
            return;
        };

        let mut player = animation_player.get_mut(ent).unwrap();

        let Some(clip) = gltf.named_animations.get(state.animation_name()) else {
            return;
        };

        if player.animation_clip() != clip {
            player.play(clip.clone()).set_speed(2.).repeat();
        }
    });
}

fn find_child_rec(
    current: Entity,
    children_query: &Query<&Children>,
    aniplayer: &Query<&AnimationPlayer>,
) -> Option<Entity> {
    let Ok(children) = children_query.get(current) else {
        return None;
    };

    for child in children.iter() {
        if aniplayer.get(*child).is_ok() {
            return Some(*child);
        } else {
            return find_child_rec(*child, children_query, aniplayer);
        }
    }

    None
}
