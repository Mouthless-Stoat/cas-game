use bevy::prelude::*;

use super::{AtlasSprite, GlobalAtlas, GlobalAtlasIndex, Texture};

/// Convert atlas data to sprite.
pub fn atlas_to_sprite(atlas: Res<GlobalAtlas>, mut query: Query<(&mut Sprite, &AtlasSprite)>) {
    for (mut sprite, atlas_sprite) in &mut query {
        if let Some(a) = &mut sprite.texture_atlas {
            a.index = (atlas_sprite.texture).into();
        } else {
            let (i, t) = {
                let t = atlas_sprite.texture;
                // Dispatch the texture into the correct atlast
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
