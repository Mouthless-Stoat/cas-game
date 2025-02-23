#![feature(macro_metavar_expr)]

mod player;
pub use player::*;

use self::grid::GridTransform;

pub mod animation;
pub mod atlast;
pub mod grid;

pub static TILE_SIZE: u16 = 8;

pub static WIDTH: u16 = 16;
pub static HEIGHT: u16 = 10;

#[derive(Clone, Copy)]
pub enum Direction {
    Zero,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[must_use] pub fn is_zero(self) -> bool {
        matches!(self, Direction::Zero)
    }
}
