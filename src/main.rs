#![allow(missing_docs)]

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    time::Duration,
};

use bevy::prelude::*;
use cas::prelude::*;

use rand::{seq::SliceRandom, thread_rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup, tileset))
        .add_systems(Update, (update_transform, transform_animation))
        .add_systems(PostUpdate, (atlas_to_sprite, input))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // read the width and height of the sheet
    let (width, height) = {
        // consult the png spec: here http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

        // load the file
        let mut f = File::open(
            [env!("CARGO_MANIFEST_DIR"), "assets", "sheet.png"]
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

    commands.insert_resource(Atlas::new(
        asset_server.load("sheet.png"),
        asset_server.add(TextureAtlasLayout::from_grid(
            UVec2::ONE * u32::from(TILE_SIZE),
            width / u32::from(TILE_SIZE + 2),
            height / u32::from(TILE_SIZE + 2),
            Some(UVec2::ONE * 2),
            Some(UVec2::ONE),
        )),
    ));

    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: f32::from(TILE_SIZE * (HEIGHT + 2)),
            },
            ..OrthographicProjection::default_2d()
        },
    ));
    commands.spawn(Player);
}

#[derive(Component)]
struct Tile;

fn tileset(mut commands: Commands) {
    let mut tiles: Vec<(AtlasSprite, GridTransform, Transform, Tile)> =
        Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            tiles.push((
                AtlasSprite::new(
                    *[
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Brick,
                        Texture::Brick,
                        Texture::Brick,
                        Texture::Flower,
                        Texture::Grass,
                        Texture::Soil,
                        Texture::Soil,
                    ]
                    .choose(&mut thread_rng())
                    .unwrap(),
                ),
                GridTransform::from_xy(
                    i32::from(x) - i32::from(WIDTH) / 2,
                    i32::from(y) - i32::from(HEIGHT) / 2 + 1,
                ),
                Transform::from_xyz(0.0, 0.0, -10.0),
                Tile,
            ));
        }
    }

    commands.spawn_batch(tiles);
}

fn input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut GridTransform, With<Player>>,
    mut animation: Single<&mut TransformAnimation, With<Player>>,
    mut sprite: Single<&mut AtlasSprite, With<Player>>,
) {
    for i in keyboard_input.get_just_pressed() {
        let dir = match i {
            KeyCode::KeyW | KeyCode::ArrowUp => Direction::Up,
            KeyCode::KeyS | KeyCode::ArrowDown => Direction::Down,
            KeyCode::KeyA | KeyCode::ArrowLeft => Direction::Left,
            KeyCode::KeyD | KeyCode::ArrowRight => Direction::Right,
            _ => Direction::Zero,
        };
        if !dir.is_zero() {
            animation.old_transform = **transform;
            transform.translate(dir, 1);
            if matches!(dir, Direction::Left) {
                sprite.flip_x = true;
            } else if matches!(dir, Direction::Right) {
                sprite.flip_x = false;
            }
            animation.duration = Duration::from_millis(100);
            break;
        }
    }

    transform.translation = transform.translation.clamp(
        -IVec2::new(i32::from(WIDTH) / 2, i32::from(HEIGHT) / 2 - 1),
        IVec2::new(i32::from(WIDTH) / 2 - 1, i32::from(HEIGHT) / 2),
    );
}
