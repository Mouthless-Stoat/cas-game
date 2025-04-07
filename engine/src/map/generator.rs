use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::prelude::*;

use super::wall::WallPiece;

/// Generator object use to generate the map
#[derive(Component)]
#[require(GridTransform)]
pub struct Generator(pub u8);

/// Small resource holding the visited position in the map to not overlap room.
#[derive(Resource)]
pub struct Visited(pub Vec<GridTransform>);

/// Generate the map.
///
/// Map generation start with a central generator entity. This entity generate a single room them
/// create copy of itself in every direction with a door with one less depth. This is repeated
/// until the generator depth reach 0.
pub fn generate_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    room_layouts: Res<Assets<RoomLayout>>,
    mut visited: ResMut<Visited>,
    generators: Query<(Entity, &Generator, &GridTransform)>,
) {
    if generators.is_empty() {
        // finally done with generation clear the vistied list
        visited.0.clear();
    }

    for (entity, gen, trans) in &generators {
        if gen.0 == 0 {
            continue;
        }
        visited.0.push(*trans);

        let mut ground_tile: Vec<(AtlasSprite, GridTransform, Transform)> =
            Vec::with_capacity((WIDTH * HEIGHT) as usize);

        let Some(room) = room_layouts.get(&asset_server.load("rooms/test.room")) else {
            return;
        };

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
                    TileType::Door => {
                        commands.spawn((
                            AtlasSprite::new(Texture::DoorN),
                            position,
                            Transform::from_xyz(0.0, 0.0, -10.0),
                        ));
                    }
                }
            }
        }

        commands.spawn_batch(ground_tile);

        commands.entity(entity).despawn();
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
            if visited.0.contains(&t) {
                continue;
            }

            commands.spawn((Generator(gen.0 - 1), t));
        }
    }
}
