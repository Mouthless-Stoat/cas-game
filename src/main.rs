use bevy::prelude::*;

use rand::{seq::SliceRandom, thread_rng};

use bevy_test::{
    animation::{animation_transform, TransformAnimation},
    atlast::{atlast_to_sprite, Atlast, AtlastSpriteBundle, Texture},
    grid::{update_transform, GridTransform},
    *,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, tileset))
        .add_systems(Update, (update_transform, animation_transform))
        .add_systems(PostUpdate, (atlast_to_sprite, input))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("sheet.png");
    commands.insert_resource(Atlast::new(
        texture,
        asset_server.add(TextureAtlasLayout::from_grid(
            UVec2::ONE * TILE_SIZE as u32,
            4,
            4,
            Some(UVec2::ONE),
            None,
        )),
    ));

    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: (TILE_SIZE * (HEIGHT + 2)) as f32,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
    commands.spawn(PlayerBundle::default());
}

#[derive(Component)]
struct Tile;

fn tileset(mut commands: Commands) {
    let mut tiles: Vec<(AtlastSpriteBundle, GridTransform, Transform, Tile)> =
        Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            tiles.push((
                AtlastSpriteBundle::new(
                    *[
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Blank,
                        Texture::Brick,
                        Texture::Flower,
                        Texture::FlowerPatch,
                        Texture::Grass,
                        Texture::Soil,
                        Texture::Soil,
                    ]
                    .choose(&mut thread_rng())
                    .unwrap(),
                ),
                GridTransform::from_xy(
                    x as i32 - WIDTH as i32 / 2,
                    y as i32 - HEIGHT as i32 / 2 + 1,
                ),
                Transform::from_xyz(0.0, 0.0, -10.0),
                Tile,
            ))
        }
    }

    commands.spawn_batch(tiles)
}

fn input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut GridTransform, With<Player>>,
    mut animation: Single<&mut TransformAnimation, With<Player>>,
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
            break;
        }
    }

    transform.translation = transform.translation.clamp(
        -IVec2::new(WIDTH as i32 / 2, HEIGHT as i32 / 2 - 1),
        IVec2::new(WIDTH as i32 / 2 - 1, HEIGHT as i32 / 2),
    );
}
