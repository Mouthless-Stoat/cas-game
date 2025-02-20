use std::time::Duration;

use bevy::prelude::*;

use crate::grid::GridTransform;

#[derive(Component, Clone, Copy)]
pub struct TransformAnimation {
    pub old_transform: GridTransform,

    pub duration: Duration,
    pub progress: Duration,

    pub function: EaseFunction,
}

impl Default for TransformAnimation {
    fn default() -> Self {
        TransformAnimation {
            old_transform: Default::default(),
            duration: Default::default(),
            progress: Default::default(),
            function: EaseFunction::Linear,
        }
    }
}

pub fn animation_transform(
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

        // create a easing jerk
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
