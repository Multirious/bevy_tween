/// Interpolation types support for [`bevy_lookup_curve`](::bevy_lookup_curve)
use super::*;
use ::bevy_lookup_curve::{Knot, KnotInterpolation, LookupCache, LookupCurve};

/// Use [`bevy_lookup_curve`](::bevy_lookup_curve) for interpolation.
pub struct BevyLookupCurveInterpolationPlugin;

impl Plugin for BevyLookupCurveInterpolationPlugin {
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be inserted to world");
        app.add_systems(
            app_resource.schedule,
            (
                sample_interpolations_system::<Curve>
                    .in_set(TweenSystemSet::UpdateInterpolationValue),
                sample_interpolations_mut_system::<CurveCached>
                    .in_set(TweenSystemSet::UpdateInterpolationValue),
            ),
        );
    }
}

/// Use [`LookupCurve`](bevy_lookup_curve::LookupCurve) for interpolation with cache.
#[derive(Default, Component)]
pub struct CurveCached(pub LookupCurve, pub LookupCache);

impl CurveCached {
    /// Create new [`CurveCached`] with new cache inside
    pub fn new(curve: LookupCurve) -> CurveCached {
        CurveCached(curve, LookupCache::new())
    }

    /// Create new [`CurveCached`] with linear curve
    pub fn new_linear() -> CurveCached {
        CurveCached(
            LookupCurve::new(vec![
                Knot {
                    position: Vec2::ZERO,
                    interpolation: KnotInterpolation::Linear,
                    ..Default::default()
                },
                Knot {
                    position: Vec2::ONE,
                    interpolation: KnotInterpolation::Linear,
                    ..Default::default()
                },
            ]),
            LookupCache::new(),
        )
    }
}

impl InterpolationMut for CurveCached {
    fn sample_mut(&mut self, v: f32) -> f32 {
        self.0.lookup_cached(v, &mut self.1)
    }
}

/// Use [`LookupCurve`](bevy_lookup_curve::LookupCurve) for interpolation.
#[derive(Default, Component)]
pub struct Curve(pub bevy_lookup_curve::LookupCurve);

impl Curve {
    /// Create new [`Curve`]
    pub fn new(curve: bevy_lookup_curve::LookupCurve) -> Curve {
        Curve(curve)
    }

    /// Create new [`Curve`] with linear curve
    pub fn new_linear() -> Curve {
        Curve(LookupCurve::new(vec![
            Knot {
                position: Vec2::ZERO,
                interpolation: KnotInterpolation::Linear,
                ..Default::default()
            },
            Knot {
                position: Vec2::ONE,
                interpolation: KnotInterpolation::Linear,
                ..Default::default()
            },
        ]))
    }
}

impl Interpolation for Curve {
    fn sample(&self, v: f32) -> f32 {
        self.0.lookup(v)
    }
}
