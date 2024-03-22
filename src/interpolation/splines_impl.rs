use bevy::prelude::*;
use splines::Spline;
use super::{Interpolation, sample_interpolations_system};

#[derive(Component)]
pub struct EaseSpline(pub Spline<f32, f32>);

impl Interpolation for EaseSpline {
    fn sample(&self, v: f32) -> f32 {
        self.0.clamped_sample(v).expect("spline not empty")
    }
}

pub struct EaseSplinePlugin;

impl Plugin for EaseSplinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            sample_interpolations_system::<EaseSpline>
                .in_set(crate::TweenSystemSet::UpdateInterpolationValue)
        );
    }
}
