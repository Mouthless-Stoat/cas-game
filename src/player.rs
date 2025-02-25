//! Contain implementations for the player.

use bevy::prelude::*;

use crate::prelude::*;

/// Marker component for the player
#[derive(Component)]
#[require(AtlasSprite(player_sprite), GridTransform, TransformAnimation)]
pub struct Player;

fn player_sprite() -> AtlasSprite {
    AtlasSprite::new(Texture::Player)
}
