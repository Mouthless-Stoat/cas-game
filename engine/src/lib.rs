#![feature(macro_metavar_expr)]

//! Library for the game.
//! Contain implementation for various engine functions and components.

pub mod prelude;

pub mod animation;
pub mod atlas;
pub mod grid;
pub mod map;
pub mod player;

/// Size of each tile.
pub static TILE_SIZE: u16 = 8;

/// Visible area width.
pub static WIDTH: u16 = 21;
/// Visible area height
pub static HEIGHT: u16 = 13;

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
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub struct Compass<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,

    pub north_east: T,
    pub south_east: T,
    pub south_west: T,
    pub north_west: T,
}
