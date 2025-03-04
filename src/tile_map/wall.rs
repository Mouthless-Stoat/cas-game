use bevy::prelude::*;

use crate::prelude::*;

use super::{NeighbourTile, SubTile};

#[derive(Bundle)]
pub struct WallPiece {
    sprite: AtlasSprite,
    transform: Transform,
    marker: SubTile,
}

impl WallPiece {
    #[rustfmt::skip]
    pub fn new(top: bool, left: bool, neighbour: NeighbourTile) -> WallPiece {
        let vert_wall = if top { neighbour.north } else { neighbour.south };
        let horz_wall = if left { neighbour.west } else { neighbour.east };

        let corner = match (top, left) {
            (true, true) => neighbour.north_west,
            (true, false) => neighbour.north_east,
            (false, true) => neighbour.south_west,
            (false, false) => neighbour.south_east,
        };

        let x = if left { -2.0 } else { 2.0 };
        let y = if top { 2.5 } else { -1.5 };

        WallPiece {
            sprite: AtlasSprite {
                texture: Texture::Wall {
                    top,
                    horz_wall,
                    vert_wall,
                    corner,
                },
                flip_y: left,
                flip_x: false,
            },
            transform: Transform::from_xyz(x, y, -10.0),
            marker: SubTile,
        }
    }
}
