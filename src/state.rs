use bevy::{asset::LoadState, prelude::*};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
        app.init_resource::<GameAssets>();
        app.add_systems(
            Update,
            await_asset_loading.run_if(in_state(GameState::Loading)),
        );
        app.add_systems(Last, despawn_unallowed);
    }
}

#[derive(Debug, Clone, Eq, States, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
    GameOver,
}

#[derive(Component)]
pub struct AllowedState(GameState);
impl AllowedState {
    pub fn new(state: GameState) -> Self {
        Self(state)
    }
}

#[derive(Resource, Default)]
pub struct GameAssets(Vec<UntypedHandle>);

impl GameAssets {
    pub fn add(&mut self, handle: UntypedHandle) {
        self.0.push(handle);
    }

    pub fn iter(&self) -> impl Iterator<Item = &UntypedHandle> {
        self.0.iter()
    }
}

fn despawn_unallowed(
    mut cmd: Commands,
    query: Query<(Entity, &AllowedState)>,
    next_state: Res<NextState<GameState>>,
) {
    let Some(next_state) = &next_state.0 else {
        return;
    };

    query.iter().for_each(|(ent, allowed_state)| {
        if allowed_state.0 != *next_state {
            cmd.entity(ent).despawn_recursive();
        }
    });
}

fn await_asset_loading(
    game_assets: Res<GameAssets>,
    server: Res<AssetServer>,
    mut state: ResMut<NextState<GameState>>,
) {
    game_assets
        .iter()
        .filter_map(|handle| server.get_load_state(handle.id()))
        .all(|state| state == LoadState::Loaded)
        .then(|| state.set(GameState::Menu));
}
