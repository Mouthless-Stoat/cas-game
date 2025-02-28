#![allow(missing_docs)]

use std::time::Duration;

use bevy::prelude::*;
use cas::prelude::*;

use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let default_plugin = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "CAS Game".into(),
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins(default_plugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup, tileset, create_global_atlas))
        .add_systems(Update, (update_transform, transform_animation))
        .add_systems(PostUpdate, (atlas_to_sprite, input))
        .run();
}

fn setup(mut commands: Commands) {
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
        if !dir.is_zero() && animation.duration.is_zero() {
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

    transform.translation = transform
        .translation
        .clamp(IVec2::ZERO, IVec2::new(i32::from(WIDTH), i32::from(HEIGHT)));
}
