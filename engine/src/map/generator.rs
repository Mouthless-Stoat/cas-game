use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::prelude::*;

use super::wall::WallPiece;

/// Generator object use to generate the map
#[derive(Component)]
#[require(GridTransform)]
pub struct Generator(pub u8);

/// Generate the map.
///
/// Map generation start with a central generator entity. This entity generate a single room them
/// create copy of itself in every direction with a door with one less depth. This is repeated
/// until the generator depth reach 0.
#[allow(clippy::too_many_lines)]
pub fn generate_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    room_layouts: Res<Assets<RoomLayout>>,
    room_list: Res<RoomList>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    generators: Query<(Entity, &Generator, &GridTransform)>,
) {
    // process every generator
    for (entity, gen, trans) in &generators {
        if gen.0 == 0 {
            continue;
        }

        let mut ground_tile: Vec<(AtlasSprite, GridTransform, Transform)> =
            Vec::with_capacity((WIDTH * HEIGHT) as usize);
        let Some(room) = loaded_folders
            .get(&room_list.0)
            .and_then(|f| f.handles.first())
            .map(|h| h.id().typed_unchecked::<RoomLayout>())
            .and_then(|l| room_layouts.get(l))
        else {
            return;
        };

        map.rooms.insert(
            (
                trans.translation.x / WIDTH as i32,
                trans.translation.y / HEIGHT as i32,
            ),
            *room,
        );

        for (row, y_og) in room.layout.iter().zip(0i32..) {
            for (tile, x_og) in row.iter().zip(0i32..) {
                let x = x_og + trans.translation.x;
                let y = y_og + trans.translation.y;

                let position = GridTransform::from_xy(x, y);

                match tile {
                    TileType::Ground => {
                        ground_tile.push((
                            AtlasSprite::new(
                                *[
                                    Texture::Blank,
                                    Texture::Blank,
                                    Texture::Blank,
                                    Texture::Blank,
                                    Texture::Blank,
                                    Texture::Blank,
                                    Texture::Blank,
                                    Texture::Soil,
                                    Texture::Flower,
                                    Texture::Grass,
                                ]
                                .choose(&mut thread_rng())
                                .unwrap(),
                            ),
                            position,
                            Transform::from_xyz(0.0, 0.0, -10.0),
                        ));
                    }

                    TileType::Wall => {
                        let neighbour = room.get_neighbour_wall(UVec2::new(
                            u32::try_from(x_og).unwrap(),
                            u32::try_from(y_og).unwrap(),
                        ));

                        commands
                            .spawn((position, Visibility::Inherited))
                            .with_children(|t| {
                                t.spawn(WallPiece::new(true, true, neighbour));
                                t.spawn(WallPiece::new(true, false, neighbour));

                                t.spawn(WallPiece::new(false, true, neighbour));
                                t.spawn(WallPiece::new(false, false, neighbour));
                            });
                    }
                    TileType::Door(dir) => {
                        let texture = match dir {
                            CompassDir::North => Texture::DoorN,
                            CompassDir::East => Texture::DoorE,
                            CompassDir::South => Texture::DoorS,
                            CompassDir::West => Texture::DoorW,
                            _ => unreachable!(),
                        };
                        commands.spawn((
                            AtlasSprite::new(texture),
                            position,
                            Transform::from_xyz(0.0, 0.0, -10.0),
                        ));
                    }
                }
            }
        }

        commands.spawn_batch(ground_tile);

        // despawn this geneerator
        commands.entity(entity).despawn();

        // spawn new child to continue generating
        let mut vec = vec![];
        if room.doors.north {
            vec.push(GridTransform::from_xy(
                trans.translation.x,
                trans.translation.y + HEIGHT as i32,
            ));
        }
        if room.doors.east {
            vec.push(GridTransform::from_xy(
                trans.translation.x + WIDTH as i32,
                trans.translation.y,
            ));
        }
        if room.doors.south {
            vec.push(GridTransform::from_xy(
                trans.translation.x,
                trans.translation.y - HEIGHT as i32,
            ));
        }
        if room.doors.west {
            vec.push(GridTransform::from_xy(
                trans.translation.x - WIDTH as i32,
                trans.translation.y,
            ));
        }

        for t in vec {
            if map.rooms.contains_key(&(t.translation.x, t.translation.y)) {
                continue;
            }

            commands.spawn((Generator(gen.0 - 1), t));
        }
    }
}
