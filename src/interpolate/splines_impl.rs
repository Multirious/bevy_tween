use super::Interpolator;
use crate::tween;
use bevy::prelude::*;
use splines::Spline;

pub struct SplinesInterpolatorsPlugin;
impl Plugin for SplinesInterpolatorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            tween::component_tween_system::<TranslationSpline>,
        );
    }
}

// there's a version mismatch
// splines's glam is v0.24
// bevy's glam is v0.25

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct W<T>(pub T);

impl<T> From<T> for W<T> {
    fn from(value: T) -> Self {
        W(value)
    }
}

impl splines::interpolate::Interpolate<f32> for W<Vec3> {
    fn step(t: f32, threshold: f32, a: Self, b: Self) -> Self {
        if t < threshold {
            a
        } else {
            b
        }
    }

    fn cosine(t: f32, a: Self, b: Self) -> Self {
        let cos_nt = (1. - (t * std::f32::consts::PI).cos()) * 0.5;
        <Self as splines::interpolate::Interpolate<f32>>::lerp(cos_nt, a, b)
    }

    fn lerp(t: f32, a: Self, b: Self) -> Self {
        W(a.0 * (1. - t) + b.0 * t)
    }

    fn cubic_hermite(
        t: f32,
        x: (f32, Self),
        a: (f32, Self),
        b: (f32, Self),
        y: (f32, Self),
    ) -> Self {
        // sampler stuff
        let two_t = t * 2.;
        let three_t = t * 3.;
        let t2 = t * t;
        let t3 = t2 * t;
        let two_t3 = t2 * two_t;
        let two_t2 = t * two_t;
        let three_t2 = t * three_t;

        // tangents
        let m0 = (b.1 .0 - x.1 .0) / (b.0 - x.0) * (b.0 - a.0);
        let m1 = (y.1 .0 - a.1 .0) / (y.0 - a.0) * (b.0 - a.0);

        W(a.1 .0 * (two_t3 - three_t2 + 1.)
            + m0 * (t3 - two_t2 + t)
            + b.1 .0 * (three_t2 - two_t3)
            + m1 * (t3 - t2))
    }

    fn quadratic_bezier(t: f32, a: Self, u: Self, b: Self) -> Self {
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;

        W(u.0 + (a.0 - u.0) * one_t2 + (b.0 - u.0) * t * t)
    }

    fn cubic_bezier(t: f32, a: Self, u: Self, v: Self, b: Self) -> Self {
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;
        let t2 = t * t;

        W(a.0 * one_t3
            + (u.0 * one_t2 * t + v.0 * one_t * t2) * 3.
            + b.0 * t2 * t)
    }

    fn cubic_bezier_mirrored(
        t: f32,
        a: Self,
        u: Self,
        v: Self,
        b: Self,
    ) -> Self {
        <Self as splines::interpolate::Interpolate<f32>>::cubic_bezier(
            t,
            a,
            u,
            W(b.0 + b.0 - v.0),
            b,
        )
    }
}

#[derive(Default, Clone)]
pub struct TranslationSpline(pub Spline<f32, W<Vec3>>);

impl Interpolator for TranslationSpline {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        let translation =
            self.0.clamped_sample(value).expect("spline not empty");
        item.translation = translation.0;
    }
}
