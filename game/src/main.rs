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
        .add_systems(Update, (proc_generator, update_camera, unload_outside))
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

// TODO: Use an input event instead of this
fn input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut map: ResMut<Map>,
    mut transform: Single<&mut GridTransform, With<Player>>,
    mut animation: Single<&mut TransformAnimation, With<Player>>,
    mut sprite: Single<&mut AtlasSprite, With<Player>>,
) {
    for i in keyboard_input.get_just_pressed() {
        let move_dir = match i {
            KeyCode::KeyW | KeyCode::ArrowUp => Direction::Up,
            KeyCode::KeyS | KeyCode::ArrowDown => Direction::Down,
            KeyCode::KeyA | KeyCode::ArrowLeft => Direction::Left,
            KeyCode::KeyD | KeyCode::ArrowRight => Direction::Right,
            _ => Direction::Zero,
        };
        if !move_dir.is_zero() && animation.duration.is_zero() {
            animation.old_transform = **transform;

            let headed_position = transform.translate(move_dir, 1).translation;

            let Some(tile_map) = map.curr_room() else {
                return;
            };

            let mut just_door = false;

            match tile_map.get_tile(
                headed_position
                    .rem_euclid(IVec2::new(WIDTH.into(), HEIGHT.into()))
                    .as_uvec2(),
            ) {
                TileType::Ground => transform.translate_mut(move_dir, 1),
                TileType::Door(dir) => {
                    match dir {
                        CompassDir::North => map.curr_room_pos.1 -= 1,
                        CompassDir::East => map.curr_room_pos.0 += 1,
                        CompassDir::South => map.curr_room_pos.1 += 1,
                        CompassDir::West => map.curr_room_pos.0 -= 1,
                        _ => unreachable!(),
                    }
                    transform.translate_mut(move_dir, 3);
                    just_door = true;
                }
                _ => (),
            }

            if matches!(move_dir, Direction::Left) {
                sprite.flip_y = true;
            } else if matches!(move_dir, Direction::Right) {
                sprite.flip_y = false;
            }

            animation.duration = Duration::from_millis(if just_door { 200 } else { 100 });
            break;
        }
    }
}

fn update_camera(mut camera: Single<&mut GridTransform, With<Camera>>, map: Res<Map>) {
    camera.translation = IVec2::new(
        map.curr_room_pos.0 * WIDTH as i32 + WIDTH as i32 / 2,
        map.curr_room_pos.1 * HEIGHT as i32 + HEIGHT as i32 / 2,
    );
}
