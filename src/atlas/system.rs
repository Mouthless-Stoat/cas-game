use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use bevy::prelude::*;

use crate::TILE_SIZE;

use super::{Atlas, AtlasSprite, GlobalAtlas, GlobalAtlasIndex, Texture};

/// Create the global atlas resource
pub fn create_global_atlas(mut commands: Commands, asset_server: Res<AssetServer>) {
    // read the width and height of the sheet
    let (width, height) = {
        // consult the png spec: here http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

        // load the file
        let mut f = File::open(
            [
                env!("CARGO_MANIFEST_DIR"),
                "assets",
                "textures",
                "atlas.png",
            ]
            .iter()
            .collect::<PathBuf>(),
        )
        .unwrap();
        f.seek(SeekFrom::Start(16)).unwrap(); // skip the first 16 bytes

        let mut buf = [0; 8];
        f.read_exact(&mut buf).unwrap(); // read the next 8 bytes

        // assembly 4 byte into u32 for width and heigh
        (
            u32::from_be_bytes(buf[0..4].try_into().unwrap()),
            u32::from_be_bytes(buf[4..8].try_into().unwrap()),
        )
    };

    let mut global_atlas = GlobalAtlas::new();

    let main_atlas = Atlas::new(
        asset_server.load("textures/atlas.png"),
        asset_server.add(TextureAtlasLayout::from_grid(
            UVec2::ONE * u32::from(TILE_SIZE),
            width / u32::from(TILE_SIZE + 2),
            height / u32::from(TILE_SIZE + 2),
            Some(UVec2::ONE * 2),
            Some(UVec2::ONE),
        )),
    );

    let wall_atlas = {
        let texture = asset_server.load("textures/wall_atlas.png");

        let mut atlas_layout = TextureAtlasLayout::new_empty(UVec2::new(12, 30));

        for i in 0..5 {
            let top_corner = UVec2::new(i * 6 + 1, 1);
            atlas_layout.add_texture(URect::from_corners(
                top_corner,
                top_corner + UVec2::new(4, 3),
            ));
        }

        for i in 0..5 {
            let top_corner = UVec2::new(i * 6 + 1, 6);
            atlas_layout.add_texture(URect::from_corners(
                top_corner,
                top_corner + UVec2::new(4, 5),
            ));
        }

        let layout = asset_server.add(atlas_layout);

        Atlas::new(texture, layout)
    };

    global_atlas.add_atlas(main_atlas);
    global_atlas.add_atlas(wall_atlas);

    commands.insert_resource(global_atlas);
}

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

        sprite.flip_x = atlas_sprite.flip_y;
        sprite.flip_y = atlas_sprite.flip_x;
    }
}
