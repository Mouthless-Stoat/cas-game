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

use crate::prelude::*;

mod wall;

mod generator;
pub use generator::*;

mod asset;
pub use asset::*;

type NeighbourTile = OctCompass<bool>;

/// Marker component for a sub tile.
#[derive(Component)]
pub struct SubTile;

/// Resource holding the Global map and current loaded room.
#[derive(Resource)]
pub struct Map(pub Handle<RoomLayout>);

/// Insert the resource for the global [`Map`]
pub fn setup_tile_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_map = asset_server.load("rooms/test.room");
    commands.insert_resource(Map(tile_map));
}
