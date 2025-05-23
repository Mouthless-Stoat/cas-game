//! Some custom rendering system

use bevy::prelude::*;

use crate::prelude::*;

/// Unload any atlast sprite that is outside of the camera "viewport"
pub fn unload_outside(
    camera_trans: Single<&GridTransform, With<Camera>>,
    mut sprite: Query<(&mut Visibility, &GridTransform)>,
) {
    for (mut vis, trans) in &mut sprite {
        let diff = (trans.translation - camera_trans.translation)
            .abs()
            .as_vec2();

        *vis = if diff.x > (WIDTH / 2).into() || diff.y > (HEIGHT / 2).into() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
