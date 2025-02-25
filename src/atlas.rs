//! Contain implmentation of the engine atlas system
//!
//! The engine first load a global [`Atlas`] resource that contain an atlas texture and a
//! preconfigure [`TextureAtlasLayout`]. This atlas can also be index using a [`Texture`]
//!
//! The engine also provide the [`AtlasSprite`] component that interact with the [`Atlas`] resource
//! to render sprite. The component is simply an auxilary component that is use to update the
//! [`Sprite`] on the same entity. This update happen on the [`atlas_to_sprite`] system.

use bevy::prelude::*;

macro_rules! texture_enum {
    ($(#[$attr:meta])*---$($n:ident),*) => {

        $(#[$attr])*
        #[derive(Clone, Copy)]
        pub enum Texture {
            $(
                #[allow(missing_docs)]
                $n
            ),*
        }

        impl From<Texture> for usize {
            fn from(val: Texture) -> Self {
                match val {
                    $(Texture::$n => ${index()}),*
                }
            }
        }
    };
}

// Order here does matter as it refer to the index in the atlas.
texture_enum! {
    /// Enum for texture within the atlas.
    ---
    Player, Blank, Dwarf,
    Snake, Zombie, Goblin,
    Ground, Brick, Soil, Grass, Flower, Grass2, FlowerPatch
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
}

impl AtlasSprite {
    /// Create a new [`AtlasSprite`] using a given [`Texture`].
    #[must_use]
    pub fn new(texture: Texture) -> AtlasSprite {
        AtlasSprite { texture }
    }
}

/// Convert atlas data to sprite.
pub fn atlas_to_sprite(atlas: Res<Atlas>, mut query: Query<(&mut Sprite, &AtlasSprite)>) {
    for (mut s, AtlasSprite { texture: t }) in &mut query {
        if let Some(a) = &mut s.texture_atlas {
            a.index = (*t).into();
        } else {
            let (i, t) = atlas.as_sprite_data(*t);
            s.image = i;
            s.texture_atlas = Some(t);
        }
    }
}
