/// Enum for texture in the atlas
#[derive(Clone, Copy)]
#[allow(missing_docs)]
#[repr(usize)]
pub enum Texture {
    Player = 0,
    Blank,
    Dwarf,
    Snake,
    Goblin,
    Ground,
    Brick,
    Soil,
    Grass,
    Flower,
    Grass2,
    Flower2,

    DoorN,
    DoorE,
    DoorS,
    DoorW,

    Wall {
        /// Is this the top wall piece
        top: bool,
        /// Is this wall piece connected on the side or horizontally
        horz_wall: bool,
        /// Is this wall piece connected on the top or vertically
        vert_wall: bool,
        /// If this wall piece have a walled corner
        corner: bool,
    },
}

impl From<Texture> for usize {
    fn from(val: Texture) -> Self {
        match val {
            Texture::Wall {
                top,
                horz_wall,
                vert_wall,
                corner,
            } => {
                let top_offset = if top { 0 } else { 5 };

                if corner && horz_wall && vert_wall {
                    return top_offset + 4;
                }

                let horz_offset = if horz_wall { 0 } else { 2 };
                let vert_offset = if vert_wall { 0 } else { 1 };

                top_offset + horz_offset + vert_offset
            }
            // this is literal black magic, I do not know what this mean, any question please
            // consult the rustonomicon
            _ => unsafe { *(&raw const val).cast::<Self>() },
        }
    }
}
