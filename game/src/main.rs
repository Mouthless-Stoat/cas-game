#![allow(missing_docs)]

use std::time::Duration;

use bevy::prelude::*;
use engine::prelude::*;
use engine::render::unload_outside;

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
        .init_asset::<RoomLayout>()
        .init_asset_loader::<RoomLayoutLoader>()
        .add_systems(Startup, (setup, create_global_atlas, setup_tile_map))
        .add_systems(Update, (generate_map, unload_outside))
        .add_systems(Update, (update_transform, transform_animation))
        .add_systems(PostUpdate, (atlas_to_sprite, input))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        GridTransform::from_xy(WIDTH / 2, HEIGHT / 2),
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: f32::from(TILE_SIZE * (HEIGHT + 2)),
            },
            ..OrthographicProjection::default_2d()
        },
    ));
    commands.spawn(Player);
    commands.spawn(Generator(4));
}

fn input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    world: Res<Map>,
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

            let Ok(headed_position) = transform.translate(dir, 1).translation.try_into() else {
                return;
            };

            let Some(tile_map) = world.curr_room() else {
                return;
            };

            if !tile_map.get_tile(headed_position).is_wall() {
                transform.translate_mut(dir, 1);
            }

            if matches!(dir, Direction::Left) {
                sprite.flip_y = true;
            } else if matches!(dir, Direction::Right) {
                sprite.flip_y = false;
            }

            animation.duration = Duration::from_millis(100);
            break;
        }
    }

    transform.translation = transform.translation.clamp(
        IVec2::ZERO,
        IVec2::new(i32::from(WIDTH - 1), i32::from(HEIGHT - 1)),
    );
}
