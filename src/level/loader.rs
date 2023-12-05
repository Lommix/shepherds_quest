use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::Deserialize;

use super::TILE_SIZE;

pub struct LevelLoaderPlugin;
impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelAsset>();
        app.register_asset_loader(LevelAssetLoader);
    }
}

pub struct LevelAssetLoader;
impl AssetLoader for LevelAssetLoader {
    type Asset = LevelAsset;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            let mut asset = ron::de::from_bytes::<LevelAsset>(bytes.as_slice()).unwrap();
            let parsed = LayoutIterator::new(&asset).collect::<Result<Vec<_>, _>>()?;

            let size = parsed
                .iter()
                .max_by(|(a, _), (b, _)| {
                    let a = a.x + a.y;
                    let b = b.x + b.y;
                    a.partial_cmp(&b).unwrap()
                })
                .expect("Empty level");

            asset.size = Some(size.0);
            asset.parsed = Some(parsed);
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level.ron"]
    }
}
#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LevelAsset {
    pub sheeps_per_spawn: usize,
    pub name: String,
    pub intro: String,
    pub win: String,
    pub loose: String,
    pub win_percent: f32,
    pub llama_stomp_rate: f32,

    pub layout: String,
    #[serde(skip)]
    pub parsed: Option<Vec<(Vec2, Tiles)>>,
    #[serde(skip)]
    pub size: Option<Vec2>,
}

#[derive(Debug)]
pub enum Tiles {
    Empty,
    Wall,
    Sheep,
    Dog,
    Trap,
    Goal,
    Llama,
}

struct LayoutIterator<'a> {
    level: &'a LevelAsset,
    ptr: usize,
    row: usize,
    col: usize,
}

impl<'a> LayoutIterator<'a> {
    fn new(level: &'a LevelAsset) -> Self {
        Self {
            level,
            row: 0,
            ptr: 0,
            col: 0,
        }
    }

    fn advance_row(&mut self) {
        self.row += 1;
        self.ptr += 1;
        self.col = 0;
    }

    fn advance_col(&mut self) {
        self.ptr += 1;
        self.col += 1;
    }
}

impl Iterator for LayoutIterator<'_> {
    type Item = anyhow::Result<(Vec2, Tiles)>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.level.layout.as_bytes().get(self.ptr) {
            Some(b'\n') => {
                self.advance_row();
                self.next()
            }
            Some(b'L') => {
                let pos = Vec2::new(self.row as f32, self.col as f32) * TILE_SIZE;
                self.advance_col();
                Some(Ok((pos, Tiles::Llama)))
            }
            Some(b'G') => {
                let pos = Vec2::new(self.row as f32, self.col as f32) * TILE_SIZE;
                self.advance_col();
                Some(Ok((pos, Tiles::Goal)))
            }
            Some(b'T') => {
                let pos = Vec2::new(self.row as f32, self.col as f32) * TILE_SIZE;
                self.advance_col();
                Some(Ok((pos, Tiles::Trap)))
            }
            Some(b'S') => {
                let pos = Vec2::new(self.row as f32, self.col as f32) * TILE_SIZE;
                self.advance_col();
                Some(Ok((pos, Tiles::Sheep)))
            }
            Some(b'D') => {
                let pos = Vec2::new(self.row as f32, self.col as f32) * TILE_SIZE;
                self.advance_col();
                Some(Ok((pos, Tiles::Dog)))
            }
            Some(b'#') => {
                let pos = Vec2::new(self.row as f32, self.col as f32) * TILE_SIZE;
                self.advance_col();
                Some(Ok((pos, Tiles::Wall)))
            }
            Some(b'-') => {
                let pos = Vec2::new(self.row as f32, self.col as f32) * TILE_SIZE;
                self.advance_col();
                Some(Ok((pos, Tiles::Empty)))
            }
            _ => {
                if self.ptr >= self.level.layout.len() {
                    None
                } else {
                    Some(Err(anyhow::anyhow!("Invalid character in level layout")))
                }
            }
        }
    }
}
