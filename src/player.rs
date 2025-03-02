//! Contain implementations for the player.

use bevy::prelude::*;

use crate::prelude::*;

/// Marker component for the player
#[derive(Component)]
#[require(
    AtlasSprite(player_sprite),
    GridTransform(player_transform),
    TransformAnimation
)]
pub struct Player;

fn player_sprite() -> AtlasSprite {
    AtlasSprite::new(Texture::Player)
}

fn player_transform() -> GridTransform {
    GridTransform::from_xy(1, 1)
}
