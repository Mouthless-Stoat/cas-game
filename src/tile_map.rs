//! Tile map implementation.
//!
//! The engine hold a glocal resource [`TileMap`] that hold every tile as [`Vec<Vec<TileType>>`].
//! Currently each tile can only be a ground or wall tile. Ground tile visual are also currently
//! picked from a weighted random.
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

/// Resource containing the global tile map.
#[derive(Resource, Debug)]
pub struct TileMap(pub [[TileType; WIDTH as usize]; HEIGHT as usize]);

type NeighbourTile = Compass<bool>;

impl TileMap {
    /// Get a tile at position. If an invalid position was given a wall tile will be return.
    #[must_use]
    pub fn get_tile(&self, position: UVec2) -> TileType {
        *self
            .0
            .get(position.y as usize)
            .and_then(|v| v.get(position.x as usize))
            .unwrap_or(&TileType::Wall)
    }

    fn get_wall_status(&self, position: UVec2, shortcut: bool, offset: IVec2) -> bool {
        shortcut
            .then_some(TileType::Wall)
            .unwrap_or_else(|| self.get_tile((position.as_ivec2() + offset).as_uvec2()))
            .is_wall()
    }

    #[rustfmt::skip] // the formatting making it a bit worst imo
    fn get_neighbour_wall(&self, position: UVec2) -> NeighbourTile {
        let is_top = position.y == 0;
        let is_left = position.x == 0;
        let is_bottom = position.y == HEIGHT.into();
        let is_right = position.x == WIDTH.into();

        Compass {
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

    /// Method to change the current tile map using a given string.
    pub fn change_tile_map(&mut self, tile_map: &str) {
        self.0 = gen_tile_map(tile_map);
    }
}

// Temporary function. TODO REPLACE WITH SOMETHING BETTER
/// Generate a [`TileMap`] using a given string. Currenty the string format is:
/// - `.` for ground tile.
/// - `#` for wall tile.
#[must_use]
pub fn gen_tile_map(input: &str) -> [[TileType; WIDTH as usize]; HEIGHT as usize] {
    let mut output: Vec<Vec<TileType>> = vec![vec![TileType::Wall; WIDTH as usize]];

    for l in input.split('\n') {
        let mut curr = vec![TileType::Wall];
        for c in l.chars() {
            match c {
                '#' => curr.push(TileType::Wall),
                '.' => curr.push(TileType::Ground),
                _ => (),
            }
        }
        curr.push(TileType::Wall);
        if curr.len() == WIDTH.into() {
            output.push(curr);
        }
    }

    output.push(vec![TileType::Wall; WIDTH as usize]);

    let len = output.len();

    // Convert the vec into fixed size array
    output
        .into_iter()
        .map(|row| {
            let len = row.len();
            row.try_into().unwrap_or_else(|_| {
                panic!("Tile map have incorrect width, expected {WIDTH}, but recieved {len}",)
            })
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| {
            panic!("Tile map have incorrect height, expected {HEIGHT}, but recieved {len}")
        })
}

/// Marker Component for a tile
#[derive(Component)]
pub struct Tile;

/// Marker component for a sub tile
#[derive(Component)]
pub struct SubTile;

/// Render all the tile map tile. This system only run when the tile map is changes.
pub fn render_tile_map(
    mut commands: Commands,
    tile_map: Res<TileMap>,
    tiles: Query<Entity, With<Tile>>,
) {
    if !tile_map.is_changed() {
        return;
    }

    info!("Tile map changes rendering tile map");

    for tile in tiles.iter() {
        commands.entity(tile).despawn_recursive();
    }

    let mut ground_tile: Vec<(AtlasSprite, GridTransform, Transform, Tile)> =
        Vec::with_capacity((WIDTH * HEIGHT) as usize);

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
pub fn setup_tile_map(mut commands: Commands) {
    let tile_map = gen_tile_map(
        "
        ...................
        ...................
        ............#......
        .........#..###....
        ...##.#.##....#....
        ...#########.......
        ....###..#.........
        .....##..#.........
        .........##........
        ...................
        ...................
        ",
    );
    commands.insert_resource(TileMap(tile_map));
}
