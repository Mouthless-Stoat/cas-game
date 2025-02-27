//! Contain implmentation of the engine atlas system
//!
//! The engine first load a global [`Atlas`] resource that contain an atlas texture and a
//! preconfigure [`TextureAtlasLayout`]. This atlas can also be index using a [`Texture`]
//!
//! The engine also provide the [`AtlasSprite`] component that interact with the [`Atlas`] resource
//! to render sprite. The component is simply an auxilary component that is use to update the
//! [`Sprite`] on the same entity. This update happen on the [`atlas_to_sprite`] system.

use bevy::prelude::*;

/// Enum for texture in the atlas
#[derive(Clone, Copy)]
#[allow(missing_docs)]
#[repr(usize)]
pub enum Texture {
    Player = 0,
    Blank,
    Dwarf,
    Snake,
    Zombie,
    Goblin,
    Ground,
    Brick,
    Soil,
    Grass,
    Flower,
    Grass2,
    Flower2,

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

                if corner {
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

/// Representing a atlas witha texture and a layout.
pub struct Atlas {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl Atlas {
    /// Create a new atlas resource using a given texture and a layout
    #[must_use]
    pub fn new(texture: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Atlas { texture, layout }
    }

    /// Get the sprite texture data from the atlas.
    /// Return a [`Handle<Image>`] pointing to the atlas texture and a [`TextureAtlas`] configure
    /// to the correct position within the texture.
    #[must_use]
    pub fn get_sprite_data(&self, texture: Texture) -> (Handle<Image>, TextureAtlas) {
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index: texture.into(),
            },
        )
    }
}

#[repr(usize)]
enum GlobalAtlasIndex {
    Main = 0,
    Wall,
}

/// Global resource for atlas.
/// This hold all [`Handle<Image>`] pointing to the texture for a atlas and all
/// [`Handle<TextureAtlaLayout>`] configure to the atlast config.
#[derive(Resource, Default)]
pub struct GlobalAtlas {
    atlas: Vec<Atlas>,
}

impl GlobalAtlas {
    /// Create a new global atlas resource.
    #[must_use]
    pub fn new() -> Self {
        GlobalAtlas { atlas: vec![] }
    }

    /// Add new atlas into the list
    pub fn add_atlas(&mut self, atlas: Atlas) {
        self.atlas.push(atlas);
    }

    fn sprite_from_atlas(
        &self,
        index: GlobalAtlasIndex,
        texture: Texture,
    ) -> (Handle<Image>, TextureAtlas) {
        self.atlas[index as usize].get_sprite_data(texture)
    }
}

/// Atlast sprite component to interface with the [`Atlas`] resource.
#[derive(Component)]
#[require(Sprite)]
pub struct AtlasSprite {
    /// Texture to index the atlas with.
    pub texture: Texture,
    /// Flip the sprite across the x axis
    pub flip_x: bool,
    /// Flip the sprite across the y axis
    pub flip_y: bool,
}

impl AtlasSprite {
    /// Create a new [`AtlasSprite`] using a given [`Texture`].
    #[must_use]
    pub fn new(texture: Texture) -> AtlasSprite {
        AtlasSprite {
            texture,
            flip_x: false,
            flip_y: false,
        }
    }
}

/// Convert atlas data to sprite.
pub fn atlas_to_sprite(atlas: Res<GlobalAtlas>, mut query: Query<(&mut Sprite, &AtlasSprite)>) {
    for (mut sprite, atlas_sprite) in &mut query {
        if let Some(a) = &mut sprite.texture_atlas {
            a.index = (atlas_sprite.texture).into();
        } else {
            let (i, t) = {
                let t = atlas_sprite.texture;
                match t {
                    Texture::Wall { .. } => atlas.sprite_from_atlas(GlobalAtlasIndex::Wall, t),
                    _ => atlas.sprite_from_atlas(GlobalAtlasIndex::Main, t),
                }
            };

            sprite.image = i;
            sprite.texture_atlas = Some(t);
        }

        sprite.flip_x = atlas_sprite.flip_x;
        sprite.flip_y = atlas_sprite.flip_y;
    }
}
