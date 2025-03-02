//! Contain implementation for the grid system.
//!
//! The engine uses the [`GridTransform`] component as an suxilry to the [`Transform`] component.
//! This component are used to update the [`Transform`] using the [`update_transform`].system.

use bevy::prelude::*;

use crate::{animation::TransformAnimation, Direction, TILE_SIZE};
use crate::{HEIGHT, WIDTH};

/// Grid transform component to define a grid position. Interface with the [`Transform`] component.
#[allow(missing_docs)]
#[derive(Component, Default, Clone, Copy)]
#[require(Transform)]
pub struct GridTransform {
    pub translation: IVec2,
    pub rotation: Quat,
}

impl GridTransform {
    /// Translate in a direction by some amount in place.
    pub fn translate_mut(&mut self, dir: Direction, amount: i32) {
        match dir {
            Direction::Zero => (),
            Direction::Up => self.translation.y -= amount,
            Direction::Left => self.translation.x -= amount,
            Direction::Down => self.translation.y += amount,
            Direction::Right => self.translation.x += amount,
        }
    }

    /// Translate in a direction by some amount and return the new position. This does not change the current
    /// translation of the transform.
    ///
    /// For the in place version see [`translate_mut`][GridTransform::translate_mut]
    #[must_use = "Consider using `translate_mut` if you want to modify it in place"]
    pub fn translate(&self, dir: Direction, amount: i32) -> GridTransform {
        let mut translation = self.translation;
        match dir {
            Direction::Zero => (),
            Direction::Up => translation -= IVec2::Y * amount,
            Direction::Left => translation -= IVec2::X * amount,
            Direction::Down => translation += IVec2::Y * amount,
            Direction::Right => translation += IVec2::X * amount,
        }

        GridTransform {
            translation,
            ..*self
        }
    }

    /// Create a new [`GridTransform`] using a given `(x, y)`
    pub fn from_xy(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        GridTransform {
            translation: IVec2::new(x.into(), y.into()),
            ..GridTransform::default()
        }
    }

    /// Convert this transform to a [`Vec3`] with the `z` index at `0.0`
    #[must_use]
    pub fn as_vec3(&self) -> Vec3 {
        let f32_tile = f32::from(TILE_SIZE);
        let i32_width = i32::from(WIDTH);
        let i32_height = i32::from(HEIGHT);

        let x = (self.translation.x - i32_width / 2) as f32 * f32_tile;
        let y = (i32_height / 2 - self.translation.y) as f32 * f32_tile;

        Vec3::new(x, y, 0.0)
    }

    /// Convert this transform to a [`Vec3`] with the given `z` index.
    #[must_use]
    pub fn as_vec3_with_z(&self, z: f32) -> Vec3 {
        self.as_vec3().with_z(z)
    }
}

/// System to update [`GridTransform`] to [`Transform`].
pub fn update_transform(
    mut transform: Query<(&mut Transform, &GridTransform), Without<TransformAnimation>>,
) {
    for (mut transform, grid) in &mut transform {
        transform.translation = grid.as_vec3_with_z(transform.translation.z);
    }
}
