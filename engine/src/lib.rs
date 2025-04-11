#![feature(macro_metavar_expr)]

//! Library for the game.
//! Contain implementation for various engine functions and components.

use bevy::ecs::system::{Resource, SystemId};
use bevy::utils::HashMap;

pub mod prelude;

pub mod animation;
pub mod atlas;
pub mod grid;
pub mod map;
pub mod player;
pub mod render;

/// Size of each tile.
pub static TILE_SIZE: u16 = 8;

/// Visible area width.
pub static WIDTH: u16 = 21;
/// Visible area height
pub static HEIGHT: u16 = 13;

#[derive(Resource)]
pub struct OneShotSystems(pub HashMap<String, SystemId>);

/// Enum for direction.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Direction {
    /// Zero or no direction.
    Zero,
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    /// Return `true` is the direction is [`Direction::Zero`].
    #[must_use]
    pub fn is_zero(self) -> bool {
        matches!(self, Direction::Zero)
    }
}

/// Compass type
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct OctCompass<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,

    pub north_east: T,
    pub south_east: T,
    pub south_west: T,
    pub north_west: T,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct QuadCompass<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,
}

impl<T> From<OctCompass<T>> for QuadCompass<T> {
    fn from(
        OctCompass {
            north,
            east,
            south,
            west,
            ..
        }: OctCompass<T>,
    ) -> Self {
        QuadCompass {
            north,
            east,
            south,
            west,
        }
    }
}

impl<T> From<QuadCompass<T>> for OctCompass<T>
where
    T: Default,
{
    fn from(
        QuadCompass {
            north,
            east,
            south,
            west,
        }: QuadCompass<T>,
    ) -> Self {
        OctCompass {
            north,
            east,
            south,
            west,
            north_east: T::default(),
            south_east: T::default(),
            south_west: T::default(),
            north_west: T::default(),
        }
    }
}

/// Enum containing the directions of the compass.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum CompassDir {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
