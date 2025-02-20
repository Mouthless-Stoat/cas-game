use bevy::prelude::*;

use crate::animation::TransformAnimation;
use crate::atlast::Texture;
use crate::{atlast::AtlastSprite, GridTransform};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: AtlastSprite,
    pub transform: GridTransform,
    pub marker: Player,
    pub transform_animation: TransformAnimation,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            sprite: AtlastSprite(Texture::PlayerD),
            transform: GridTransform::default(),
            marker: Player,
            transform_animation: TransformAnimation::default(),
        }
    }
}
