use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::prelude::*;

/// Generator object use to generate the map
#[derive(Component)]
#[require(GridTransform)]
pub struct Generator(pub u8);

/// List of visited location during map generation.
#[derive(Resource)]
pub struct Visited(pub HashMap<(i32, i32), QuadCompass<bool>>);

/// Place hoder room object.
#[derive(Component)]
pub struct Room(QuadCompass<bool>);

/// Proc the map genrator to start generating the next depth.
///
/// The generation start with a central generator that spawn children in every direction with a
/// door.
pub fn proc_generator(
    mut commands: Commands,
    mut visited: ResMut<Visited>,
    generators: Query<(Entity, &Generator, &GridTransform)>,
    one_shot_system: Res<OneShotSystems>,
) {
    if generators.is_empty() {
        commands.run_system(one_shot_system.0["fill_room"]);
        return;
    }
    // process every generator
    for (entity, gen, trans) in &generators {
        let key = (
            trans.translation.x / WIDTH as i32,
            trans.translation.y / HEIGHT as i32,
        );

        // despawn this generator
        commands.entity(entity).despawn();

        if gen.0 == 0 || visited.0.contains_key(&key) {
            continue;
        }

        // spawn new child to continue generating
        let mut vec = vec![];

        let mut doors = QuadCompass {
            north: thread_rng().gen_bool(0.5),
            east: thread_rng().gen_bool(0.5),
            south: thread_rng().gen_bool(0.5),
            west: thread_rng().gen_bool(0.5),
        };

        if !doors.north && !doors.east && !doors.south && !doors.west {
            doors = QuadCompass {
                north: true,
                east: true,
                south: true,
                west: true,
            }
        }

        if doors.north {
            vec.push(GridTransform::from_xy(
                trans.translation.x,
                trans.translation.y - HEIGHT as i32,
            ));
        }
        if doors.east {
            vec.push(GridTransform::from_xy(
                trans.translation.x + WIDTH as i32,
                trans.translation.y,
            ));
        }
        if doors.south {
            vec.push(GridTransform::from_xy(
                trans.translation.x,
                trans.translation.y + HEIGHT as i32,
            ));
        }
        if doors.west {
            vec.push(GridTransform::from_xy(
                trans.translation.x - WIDTH as i32,
                trans.translation.y,
            ));
        }

        visited.0.insert(key, doors);

        for t in vec {
            commands.spawn((Generator(gen.0 - 1), t));
        }

        commands.spawn((Room(doors), *trans));
    }
}

/// Replace [`Room`] object with actual room
pub fn fill_room(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut visited: ResMut<Visited>,
    room_layouts: Res<Assets<RoomLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    room_list: Res<RoomList>,
    mut rooms: Query<(Entity, &mut Room, &GridTransform)>,
) {
    let Some(loaded_folder) = loaded_folders.get(&room_list.0) else {
        return;
    };
    for (entity, mut room, trans) in &mut rooms {
        commands.entity(entity).despawn();
        let mut ground_tile: Vec<(AtlasSprite, GridTransform, Transform)> =
            Vec::with_capacity((WIDTH * HEIGHT) as usize);

        let curr = (
            trans.translation.x / WIDTH as i32,
            trans.translation.y / HEIGHT as i32,
        );

        if !visited.0.is_empty() {
            room.0.north =
                matches!(visited.0.get(&(curr.0, curr.1 - 1)), Some(d) if d.south || room.0.north);
            room.0.east =
                matches!(visited.0.get(&(curr.0 + 1, curr.1)), Some(d) if d.west || room.0.east);
            room.0.south =
                matches!(visited.0.get(&(curr.0, curr.1 + 1)), Some(d) if d.north || room.0.south);
            room.0.west =
                matches!(visited.0.get(&(curr.0 - 1, curr.1)), Some(d) if d.east || room.0.west);

            visited.0.insert(curr, room.0);
        }

        let Some(layout) = RoomList::pick_room(room.0, loaded_folder, &room_layouts) else {
            return;
        };

        map.rooms.insert(
            (
                trans.translation.x / WIDTH as i32,
                trans.translation.y / HEIGHT as i32,
            ),
            layout,
        );

        for (row, y_og) in layout.layout.iter().zip(0i32..) {
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
                        let neighbour = layout.get_neighbour_wall(UVec2::new(
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
    }
}
