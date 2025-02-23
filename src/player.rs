use bevy::prelude::*;

use crate::{
    animation::TransformAnimation,
    atlast::{AtlastSpriteBundle, Texture},
    GridTransform,
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: AtlastSpriteBundle,
    pub transform: GridTransform,
    pub marker: Player,
    pub transform_animation: TransformAnimation,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            sprite: AtlastSpriteBundle::new(Texture::Player),
            transform: GridTransform::default(),
            marker: Player,
            transform_animation: TransformAnimation::default(),
        }
    }
}
