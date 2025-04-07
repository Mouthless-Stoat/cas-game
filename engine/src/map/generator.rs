use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::prelude::*;

use super::wall::WallPiece;

#[derive(Component)]
#[require(GridTransform)]
pub struct Generator(pub u8);

#[derive(Resource)]
pub struct Visited(pub Vec<GridTransform>);

pub fn generate_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    room_layouts: Res<Assets<RoomLayout>>,
    mut visted: ResMut<Visited>,
    generators: Query<(Entity, &Generator, &GridTransform)>,
) {
    if generators.is_empty() {
        return;
    }

    for (entity, gen, trans) in &generators {
        if gen.0 == 0 {
            continue;
        }
        visted.0.push(*trans);

        let mut ground_tile: Vec<(AtlasSprite, GridTransform, Transform, Tile)> =
            Vec::with_capacity((WIDTH * HEIGHT) as usize);

        let Some(room) = room_layouts.get(&asset_server.load("rooms/test.room")) else {
            return;
        };

        for (row, y_og) in room.layout.iter().zip(0i32..) {
            for (tile, x_og) in row.iter().zip(0i32..) {
                let x = x_og + trans.translation.x;
                let y = y_og + trans.translation.y;

                let position = GridTransform::from_xy(x, y);

                if matches!(tile, TileType::Ground) {
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
                        Tile,
                    ));
                }

                if matches!(tile, TileType::Wall) {
                    let neighbour = room.get_neighbour_wall(UVec2::new(
                        u32::try_from(x_og).unwrap(),
                        u32::try_from(y_og).unwrap(),
                    ));

                    commands
                        .spawn((Tile, position, Visibility::Inherited))
                        .with_children(|t| {
                            t.spawn(WallPiece::new(true, true, neighbour));
                            t.spawn(WallPiece::new(true, false, neighbour));

                            t.spawn(WallPiece::new(false, true, neighbour));
                            t.spawn(WallPiece::new(false, false, neighbour));
                        });
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
            if visted.0.contains(&t) {
                continue;
            }

            commands.spawn((Generator(gen.0 - 1), t));
        }
    }
}
