//! Custom asset loader for [`TileMap`]
//!

use bevy::{
    asset::{io::Reader, Asset, AssetLoader, LoadContext},
    reflect::TypePath,
};
use thiserror::Error;

use crate::{HEIGHT, WIDTH};

use super::TileType;

#[derive(Asset, TypePath)]
pub struct RoomLayout(pub [[TileType; WIDTH as usize]; HEIGHT as usize]);

struct RoomAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum RoomLayoutError {
    #[error("Could not load room asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid character in room asset: {0}")]
    Ascii(String),
    #[error("Invalid tile character in room asset: {0}")]
    TileType(char),
}

impl AssetLoader for RoomAssetLoader {
    type Asset = RoomLayout;
    type Settings = ();
    type Error = RoomLayoutError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;

        if !bytes.is_ascii() {
            return Err(RoomLayoutError::Ascii(
                load_context.path().to_str().unwrap().to_string(),
            ));
        }

        let string = String::from_utf8(bytes).unwrap();

        let mut tile_map = vec![vec![TileType::Wall; WIDTH as usize]];

        for line in string.lines() {
            let mut curr = vec![TileType::Wall];
            for c in line.chars() {
                curr.push(match c {
                    '.' => TileType::Ground,
                    '#' => TileType::Wall,
                    c => return Err(RoomLayoutError::TileType(c)),
                });
            }
            curr.push(TileType::Wall);
            tile_map.push(curr);
        }

        let len = tile_map.len();

        Ok(RoomLayout(
            tile_map
                .into_iter()
                .map(|row| {
                    let len = row.len();
                    row.try_into().unwrap_or_else(|_| {
                        panic!(
                            "Tile map have incorrect width, expected {WIDTH}, but recieved {len}",
                        )
                    })
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| {
                    panic!("Tile map have incorrect height, expected {HEIGHT}, but recieved {len}")
                }),
        ))
    }

    fn extensions(&self) -> &[&str] {
        &["room"]
    }
}
