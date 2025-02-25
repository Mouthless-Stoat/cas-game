#![feature(macro_metavar_expr)]

//! Library for the game.
//! Contain implementation for various engine functions and components.

pub mod prelude;

pub mod animation;
pub mod atlas;
pub mod grid;
pub mod player;

/// Size of each tile.
pub static TILE_SIZE: u16 = 8;

/// Visible area width.
pub static WIDTH: u16 = 16;
/// Visible area height
pub static HEIGHT: u16 = 10;

/// Enum for direction.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Direction {
    /// Zero or no direction.
    Zero,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Return `true` is the direction is [`Direction::Zero`].
    #[must_use]
    pub fn is_zero(self) -> bool {
        matches!(self, Direction::Zero)
    }
}
