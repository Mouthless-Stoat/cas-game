//! Include anomation implementation for the engine.
//! - [`TransformAnimation`]: Aniamtion releated to an object transform.

use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::*;

/// Animation for object transform. Easing between 2 transform value.
#[derive(Component, Clone, Copy)]
#[require(GridTransform)]
pub struct TransformAnimation {
    /// Old postiion to ease from.
    pub old_transform: GridTransform,

    /// The duration of the animation.
    pub duration: Duration,
    progress: Duration,

    function: EaseFunction,
}

impl TransformAnimation {
    /// Create a new [`TransformAnimation`] component with a given `function`.
    #[allow(unused)]
    fn with_function(function: EaseFunction) -> Self {
        TransformAnimation {
            function,
            ..Default::default()
        }
    }
}

impl Default for TransformAnimation {
    fn default() -> Self {
        TransformAnimation {
            old_transform: GridTransform::default(),
            duration: Duration::default(),
            progress: Duration::default(),
            function: EaseFunction::Linear,
        }
    }
}

/// System to handle [`TransformAnimation`] and animate them.
pub fn transform_animation(
    fixed_time: Res<Time<Fixed>>,
    mut data: Query<(&mut Transform, &GridTransform, &mut TransformAnimation)>,
) {
    for (mut transform, grid, mut transform_animation) in &mut data {
        // skip if there no animation
        if transform_animation.duration.is_zero() {
            transform.translation = grid.as_vec3_with_z(transform.translation.z);
            continue;
        }

        // Increase the progress
        transform_animation.progress += fixed_time.delta();

        let TransformAnimation {
            old_transform,
            duration,
            progress,
            function,
        } = *transform_animation;

        let z = transform.translation.z;

        // create a easing curve
        let curve = EasingCurve::new(
            old_transform.as_vec3_with_z(z),
            grid.as_vec3_with_z(z),
            function,
        )
        .reparametrize_linear(interval(0.0, 1.0).unwrap()) // map to between 0 and 1
        .unwrap();

        let t = (progress.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);

        transform.translation = curve.sample(t).unwrap();

        if progress >= duration {
            transform_animation.duration = Duration::ZERO;
            transform_animation.progress = Duration::ZERO;
        }
    }
}
