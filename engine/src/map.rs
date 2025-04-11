//! Map implementation.
//!
//! The engine hold a glocal resource [`Map`] that hold the current loaded room as
//! Currently each tile can only be a ground or wall tile.
//!
//! Wall tile are created using 4 sub wall tile, 2 for the top half and 2 for the bottom half. Each
//! sub tile get choosen by considering the 2 adjacent tile to it, above and left for the top left sub
//! tile, above and right for the top right sub tile, etc. The also consider the corner if all
//! their adjacent tile are wall.

use bevy::asset::LoadedFolder;
use bevy::math::bool;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::prelude::*;

mod wall;
pub(crate) use wall::*;

mod generator;
pub use generator::*;

mod asset;
pub use asset::*;

type NeighbourTile = OctCompass<bool>;

/// Marker component for a sub tile.
#[derive(Component)]
pub struct SubTile;

/// Preload list of room layout to use in mao generation.
#[derive(Resource, Debug)]
pub struct RoomList(pub Handle<LoadedFolder>);

impl RoomList {
    /// Pick a room from a loaded folder and some query about which door is open.
    #[must_use]
    pub fn pick_room(
        door: QuadCompass<bool>,
        room_list: &LoadedFolder,
        room_layouts: &Res<Assets<RoomLayout>>,
    ) -> Option<RoomLayout> {
        let mut vec = vec![];
        for handle in &room_list.handles {
            let id = handle.id().typed_unchecked::<RoomLayout>();
            if let Some(layout) = room_layouts.get(id) {
                if layout.doors == door {
                    vec.push(*layout);
                }
            }
        }

        vec.choose(&mut thread_rng()).copied()
    }
}

/// Resource holding the Global map and current loaded room.
#[derive(Resource, Debug)]
pub struct Map {
    /// Current room to interact with
    pub curr_room_pos: (i32, i32),
    /// Hashmap of room in the map.
    pub rooms: HashMap<(i32, i32), RoomLayout>,
}

impl Map {
    /// Get the current room layout.
    #[must_use]
    pub fn curr_room(&self) -> Option<&RoomLayout> {
        self.rooms.get(&self.curr_room_pos)
    }
}

/// Insert the resource for the global [`Map`]
pub fn setup_tile_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RoomList(asset_server.load_folder("rooms")));
    commands.insert_resource(Visited(HashMap::new()));
    commands.insert_resource(Map {
        curr_room_pos: (0, 0),
        rooms: HashMap::new(),
    });
}
