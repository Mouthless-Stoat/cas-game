//! Contain implmentation of the engine atlas system
//!
//! The engine first load a global [`Atlas`] resource that contain an atlas texture and a
//! preconfigure [`TextureAtlasLayout`]. This atlas can also be index using a [`Texture`]
//!
//! The engine also provide the [`AtlasSprite`] component that interact with the [`Atlas`] resource
//! to render sprite. The component is simply an auxilary component that is use to update the
//! [`Sprite`] on the same entity. This update happen on the [`atlas_to_sprite`] system.

use bevy::prelude::*;

mod system;
mod texture;

pub use system::*;
pub use texture::*;

/// Representing a atlas with a texture and a layout.
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

/// Use to index into the global atlas
#[repr(usize)]
pub enum GlobalAtlasIndex {
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
