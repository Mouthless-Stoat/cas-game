use bevy::prelude::*;

use crate::{animation::TransformAnimation, Direction, TILE_SIZE};

#[derive(Component, Default, Clone, Copy)]
pub struct GridTransform {
    pub translation: IVec2,
    pub rotation: Quat,
}

impl GridTransform {
    pub fn translate(&mut self, dir: Direction, amount: i32) {
        match dir {
            Direction::Zero => (),
            Direction::Up => self.translation.y += amount,
            Direction::Down => self.translation.y -= amount,
            Direction::Right => self.translation.x += amount,
            Direction::Left => self.translation.x -= amount,
        }
    }

    pub fn from_xy(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        GridTransform {
            translation: IVec2::new(x.into(), y.into()),
            ..GridTransform::default()
        }
    }

    #[must_use]
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            (self.translation.x * i32::from(TILE_SIZE)) as f32 + f32::from(TILE_SIZE) / 2.0,
            (self.translation.y * i32::from(TILE_SIZE)) as f32 - f32::from(TILE_SIZE) / 2.0,
            0.0,
        )
    }

    #[must_use]
    pub fn as_vec3_with_z(&self, z: f32) -> Vec3 {
        self.as_vec3().with_z(z)
    }
}

impl From<GridTransform> for Vec3 {
    fn from(val: GridTransform) -> Self {
        Vec3::new(
            (val.translation.x * i32::from(TILE_SIZE)) as f32,
            (val.translation.y * i32::from(TILE_SIZE)) as f32,
            0.0,
        )
    }
}

pub fn update_transform(
    mut transform: Query<(&mut Transform, &GridTransform), Without<TransformAnimation>>,
) {
    for (mut transform, grid) in &mut transform {
        transform.translation = grid.as_vec3_with_z(transform.translation.z);
    }
}
