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
}
impl From<Texture> for usize {
    fn from(val: Texture) -> Self {
        val as usize
    }
}

/// Global resource for atlas resource.
/// This hold a [`Handle<Image>`] pointing to the texture for this atlas and a
/// [`Handle<TextureAtlaLayout>`] configure to the atlast config.
#[derive(Resource)]
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
    pub fn as_sprite_data(&self, texture: Texture) -> (Handle<Image>, TextureAtlas) {
        (
            self.texture.clone(),
            TextureAtlas {
                layout: self.layout.clone(),
                index: texture.into(),
            },
        )
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
pub fn atlas_to_sprite(atlas: Res<Atlas>, mut query: Query<(&mut Sprite, &AtlasSprite)>) {
    for (mut sprite, atlas_sprite) in &mut query {
        if let Some(a) = &mut sprite.texture_atlas {
            a.index = (atlas_sprite.texture).into();
        } else {
            let (i, t) = atlas.as_sprite_data(atlas_sprite.texture);
            sprite.image = i;
            sprite.texture_atlas = Some(t);
        }

        sprite.flip_x = atlas_sprite.flip_x;
        sprite.flip_y = atlas_sprite.flip_y;
    }
}
