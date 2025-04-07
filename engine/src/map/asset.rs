//! Custom asset loader for [`TileMap`]
//!

use bevy::{
    asset::{io::Reader, Asset, AssetLoader, LoadContext},
    math::{IVec2, UVec2},
    reflect::TypePath,
};
use thiserror::Error;

use crate::prelude::*;

use super::NeighbourTile;

/// Enum holding type of tile that the tile map can display.
#[derive(Clone, Copy, Debug)]
pub enum TileType {
    /// Wall tile. Automatically connect to other wall tile.
    Wall,
    /// Ground tile. Visual picked at random with weights:
    /// - 70% Blank.
    /// - 30%: Soil, Flower or Grass.
    Ground,
    /// Door tile.
    Door,
}

impl TileType {
    /// Return if the tile is a [`TileType::Wall`]
    #[must_use]
    pub fn is_wall(self) -> bool {
        matches!(self, TileType::Wall)
    }
}

/// Asset for a room layout to be load by the engine.
#[derive(Asset, TypePath)]
pub struct RoomLayout {
    /// Rooms for this room layout.
    pub doors: QuadCompass<bool>,
    /// Layout of the room.
    pub layout: [[TileType; WIDTH as usize]; HEIGHT as usize],
}

/// Loader for [`RoomLayout`] asset
#[derive(Default)]
pub struct RoomLayoutLoader;

#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum RoomLayoutError {
    #[error("Could not load room asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid character in room asset: {0}")]
    Ascii(String),
    #[error("Invalid tile character in room asset: {0}")]
    TileType(char),
    #[error("Invalid door character in room asset: {0}")]
    DoorDir(char),
}

impl AssetLoader for RoomLayoutLoader {
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
        let mut lines = string.lines();

        let first = lines.next().unwrap();
        let mut doors = QuadCompass::default();

        for char in first.chars() {
            match char {
                'N' => doors.north = true,
                'E' => doors.east = true,
                'S' => doors.south = true,
                'W' => doors.west = true,
                _ => return Err(RoomLayoutError::DoorDir(char)),
            }
        }

        for line in lines {
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

        tile_map.push(vec![TileType::Wall; WIDTH as usize]);

        let horz_mid = ((WIDTH - 1) / 2) as usize;
        let vert_mid = ((HEIGHT - 1) / 2) as usize;

        if doors.north {
            tile_map[0][horz_mid] = TileType::Door;
        }
        if doors.east {
            tile_map[vert_mid][(WIDTH - 1) as usize] = TileType::Door;
        }
        if doors.south {
            tile_map[(HEIGHT - 1) as usize][horz_mid] = TileType::Door;
        }
        if doors.west {
            tile_map[vert_mid][0] = TileType::Door;
        }

        let len = tile_map.len();

        Ok(RoomLayout {
            doors,
            layout: tile_map
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
        })
    }

    fn extensions(&self) -> &[&str] {
        &["room"]
    }
}

impl RoomLayout {
    /// Get a tile at position. If an invalid position was given a wall tile will be return.
    #[must_use]
    pub fn get_tile(&self, position: UVec2) -> TileType {
        *self
            .layout
            .get(position.y as usize)
            .and_then(|v| v.get(position.x as usize))
            .unwrap_or(&TileType::Wall)
    }

    fn get_wall_status(&self, position: UVec2, shortcut: bool, offset: IVec2) -> bool {
        matches!(
            shortcut
                .then_some(TileType::Wall)
                .unwrap_or_else(|| self.get_tile((position.as_ivec2() + offset).as_uvec2())),
            TileType::Wall | TileType::Door
        )
    }

    /// Get neighbouring wall tile.
    #[must_use]
    #[rustfmt::skip] // the formatting making it a bit worst imo
    pub fn get_neighbour_wall(&self, position: UVec2) -> NeighbourTile {
        let is_top = position.y == 0;
        let is_left = position.x == 0;
        let is_bottom = position.y == HEIGHT.into();
        let is_right = position.x == WIDTH.into();

        OctCompass {
            north: self.get_wall_status(position, is_top, IVec2::NEG_Y),
            east: self.get_wall_status(position, is_right, IVec2::X),
            south: self.get_wall_status(position, is_bottom, IVec2::Y),
            west: self.get_wall_status(position, is_left, IVec2::NEG_X),

            north_east: self.get_wall_status(position, is_top && is_right, IVec2::NEG_Y + IVec2::X),
            south_east: self.get_wall_status(position, is_bottom && is_right, IVec2::ONE),
            south_west: self.get_wall_status( position, is_bottom && is_left, IVec2::Y + IVec2::NEG_X,),
            north_west: self.get_wall_status(position, is_top && is_left, IVec2::NEG_ONE),
        }
    }
}
