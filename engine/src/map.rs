//! Map implementation.
//!
//! The engine hold a glocal resource [`Map`] that hold the current loaded room as
//! Currently each tile can only be a ground or wall tile.
//!
//! Wall tile are created using 4 sub wall tile, 2 for the top half and 2 for the bottom half. Each
//! sub tile get choosen by considering the 2 adjacent tile to it, above and left for the top left sub
//! tile, above and right for the top right sub tile, etc. The also consider the corner if all
//! their adjacent tile are wall.

use bevy::math::bool;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::prelude::*;

mod wall;
use wall::WallPiece;

mod asset;
pub use asset::*;

/// Enum holding type of tile that the tile map can display.
#[derive(Clone, Copy, Debug)]
pub enum TileType {
    /// Wall tile. Automatically connect to other wall tile.
    Wall,
    /// Ground tile. Visual picked at random with weights:
    /// - 70% Blank.
    /// - 30%: Soil, Flower or Grass.
    Ground,
}

impl TileType {
    /// Return if the tile is a [`TileType::Wall`]
    #[must_use]
    pub fn is_wall(self) -> bool {
        matches!(self, TileType::Wall)
    }
}

type NeighbourTile = Compass<bool>;

/// Marker Component for a tile.
#[derive(Component)]
pub struct Tile;

/// Marker component for a sub tile.
#[derive(Component)]
pub struct SubTile;

/// Resource holding the Global map and current loaded room.
#[derive(Resource)]
pub struct Map(pub Handle<RoomLayout>);

/// Render all the tile map tile. This system only run when the tile map is changes.
pub fn render_tile_map(
    mut commands: Commands,
    room_layouts: Res<Assets<RoomLayout>>,
    world: Res<Map>,
    tiles: Query<Entity, With<Tile>>,
) {
    if !world.is_changed() {
        return;
    }

    info!("Tile map changes rendering tile map");

    for tile in tiles.iter() {
        commands.entity(tile).despawn_recursive();
    }

    let mut ground_tile: Vec<(AtlasSprite, GridTransform, Transform, Tile)> =
        Vec::with_capacity((WIDTH * HEIGHT) as usize);

    let Some(tile_map) = room_layouts.get(&world.0) else {
        return;
    };

    for (y, row) in tile_map.0.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let position =
                GridTransform::from_xy(i32::try_from(x).unwrap(), i32::try_from(y).unwrap());

            if matches!(tile, TileType::Ground) {
                ground_tile.push((
                    AtlasSprite::new(
                        *[
                            Texture::Blank,
                            Texture::Blank,
                            Texture::Blank,
                            Texture::Blank,
                            Texture::Blank,
                            Texture::Blank,
                            Texture::Blank,
                            Texture::Soil,
                            Texture::Flower,
                            Texture::Grass,
                        ]
                        .choose(&mut thread_rng())
                        .unwrap(),
                    ),
                    position,
                    Transform::from_xyz(0.0, 0.0, -10.0),
                    Tile,
                ));
            }

            if matches!(tile, TileType::Wall) {
                let neighbour = tile_map.get_neighbour_wall(UVec2::new(
                    u32::try_from(x).unwrap(),
                    u32::try_from(y).unwrap(),
                ));

                commands
                    .spawn((Tile, position, Visibility::Inherited))
                    .with_children(|t| {
                        t.spawn(WallPiece::new(true, true, neighbour));
                        t.spawn(WallPiece::new(true, false, neighbour));

                        t.spawn(WallPiece::new(false, true, neighbour));
                        t.spawn(WallPiece::new(false, false, neighbour));
                    });
            }
        }
    }

    commands.spawn_batch(ground_tile);

    info!("Tile map finish rendering");
}

/// Insert the resource for the global [`TileMap`]
pub fn setup_tile_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_map = asset_server.load("rooms/test.room");
    commands.insert_resource(Map(tile_map));
}
