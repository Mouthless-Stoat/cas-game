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
        let vert_wall = if top { neighbour.top } else { neighbour.bottom };
        let horz_wall = if left { neighbour.left } else { neighbour.right };

        let corner = match (top, left) {
            (true, true) => neighbour.top_left,
            (true, false) => neighbour.top_right,
            (false, true) => neighbour.bottom_left,
            (false, false) => neighbour.bottom_right,
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
