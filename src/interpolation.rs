//! Module containing ease functions and related systems.
//!
//! # [`Interpolation`]
//!
//! **Built-in interpolations**:
//! - [`EaseKind`]
//! - [`EaseClosure`]
//!
//! **Systems**:
//! - [`sample_interpolations_system`]

use bevy::prelude::*;

use crate::{tween::TweenInterpolationValue, TweenSystemSet};
use bevy_time_runner::TimeSpanProgress;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy_lookup_curve")]
pub mod bevy_lookup_curve;

/// A trait for implementing interpolation algorithms.
///
/// Currently only used for registering [`sample_interpolations_system`].
pub trait Interpolation {
    /// Sample a value from this algorithm.
    /// Input should be between 0–1 and returns value that should be
    /// between 0–1
    fn sample(&self, v: f32) -> f32;
}

/// Plugin for [`EaseKind`]
pub struct EaseKindPlugin;

impl Plugin for EaseKindPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        app.add_systems(
            app_resource.schedule,
            sample_interpolations_system::<EaseKind>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
        )
        .register_type::<EaseKind>();
    }
}

/// Curve functions over the [unit interval], commonly used for easing transitions.
/// 
/// # Note
/// This enum is copied directly from [`EaseFunction`] and will be deprecated in future version.
///
/// [unit interval]: `Interval::UNIT`
#[derive(Debug, Copy, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum EaseKind {
    /// `f(t) = t`
    Linear,

    /// `f(t) = t²`
    QuadraticIn,
    /// `f(t) = -(t * (t - 2.0))`
    QuadraticOut,
    /// Behaves as `EaseFunction::QuadraticIn` for t < 0.5 and as `EaseFunction::QuadraticOut` for t >= 0.5
    QuadraticInOut,

    /// `f(t) = t³`
    CubicIn,
    /// `f(t) = (t - 1.0)³ + 1.0`
    CubicOut,
    /// Behaves as `EaseFunction::CubicIn` for t < 0.5 and as `EaseFunction::CubicOut` for t >= 0.5
    CubicInOut,

    /// `f(t) = t⁴`
    QuarticIn,
    /// `f(t) = (t - 1.0)³ * (1.0 - t) + 1.0`
    QuarticOut,
    /// Behaves as `EaseFunction::QuarticIn` for t < 0.5 and as `EaseFunction::QuarticOut` for t >= 0.5
    QuarticInOut,

    /// `f(t) = t⁵`
    QuinticIn,
    /// `f(t) = (t - 1.0)⁵ + 1.0`
    QuinticOut,
    /// Behaves as `EaseFunction::QuinticIn` for t < 0.5 and as `EaseFunction::QuinticOut` for t >= 0.5
    QuinticInOut,

    /// `f(t) = 1.0 - cos(t * π / 2.0)`
    SineIn,
    /// `f(t) = sin(t * π / 2.0)`
    SineOut,
    /// Behaves as `EaseFunction::SineIn` for t < 0.5 and as `EaseFunction::SineOut` for t >= 0.5
    SineInOut,

    /// `f(t) = 1.0 - sqrt(1.0 - t²)`
    CircularIn,
    /// `f(t) = sqrt((2.0 - t) * t)`
    CircularOut,
    /// Behaves as `EaseFunction::CircularIn` for t < 0.5 and as `EaseFunction::CircularOut` for t >= 0.5
    CircularInOut,

    /// `f(t) = 2.0^(10.0 * (t - 1.0))`
    ExponentialIn,
    /// `f(t) = 1.0 - 2.0^(-10.0 * t)`
    ExponentialOut,
    /// Behaves as `EaseFunction::ExponentialIn` for t < 0.5 and as `EaseFunction::ExponentialOut` for t >= 0.5
    ExponentialInOut,

    /// `f(t) = -2.0^(10.0 * t - 10.0) * sin((t * 10.0 - 10.75) * 2.0 * π / 3.0)`
    ElasticIn,
    /// `f(t) = 2.0^(-10.0 * t) * sin((t * 10.0 - 0.75) * 2.0 * π / 3.0) + 1.0`
    ElasticOut,
    /// Behaves as `EaseFunction::ElasticIn` for t < 0.5 and as `EaseFunction::ElasticOut` for t >= 0.5
    ElasticInOut,

    /// `f(t) = 2.70158 * t³ - 1.70158 * t²`
    BackIn,
    /// `f(t) = 1.0 +  2.70158 * (t - 1.0)³ - 1.70158 * (t - 1.0)²`
    BackOut,
    /// Behaves as `EaseFunction::BackIn` for t < 0.5 and as `EaseFunction::BackOut` for t >= 0.5
    BackInOut,

    /// bouncy at the start!
    BounceIn,
    /// bouncy at the end!
    BounceOut,
    /// Behaves as `EaseFunction::BounceIn` for t < 0.5 and as `EaseFunction::BounceOut` for t >= 0.5
    BounceInOut,

    /// `n` steps connecting the start and the end
    Steps(usize),

    /// `f(omega,t) = 1 - (1 - t)²(2sin(omega * t) / omega + cos(omega * t))`, parametrized by `omega`
    Elastic(f32),
}

impl EaseKind {
    /// Sample a value from this ease function.
    pub fn sample(&self, t: f32) -> f32 {
        match self {
            EaseKind::Linear => easing_functions::linear(t),
            EaseKind::QuadraticIn => easing_functions::quadratic_in(t),
            EaseKind::QuadraticOut => easing_functions::quadratic_out(t),
            EaseKind::QuadraticInOut => easing_functions::quadratic_in_out(t),
            EaseKind::CubicIn => easing_functions::cubic_in(t),
            EaseKind::CubicOut => easing_functions::cubic_out(t),
            EaseKind::CubicInOut => easing_functions::cubic_in_out(t),
            EaseKind::QuarticIn => easing_functions::quartic_in(t),
            EaseKind::QuarticOut => easing_functions::quartic_out(t),
            EaseKind::QuarticInOut => easing_functions::quartic_in_out(t),
            EaseKind::QuinticIn => easing_functions::quintic_in(t),
            EaseKind::QuinticOut => easing_functions::quintic_out(t),
            EaseKind::QuinticInOut => easing_functions::quintic_in_out(t),
            EaseKind::SineIn => easing_functions::sine_in(t),
            EaseKind::SineOut => easing_functions::sine_out(t),
            EaseKind::SineInOut => easing_functions::sine_in_out(t),
            EaseKind::CircularIn => easing_functions::circular_in(t),
            EaseKind::CircularOut => easing_functions::circular_out(t),
            EaseKind::CircularInOut => easing_functions::circular_in_out(t),
            EaseKind::ExponentialIn => easing_functions::exponential_in(t),
            EaseKind::ExponentialOut => easing_functions::exponential_out(t),
            EaseKind::ExponentialInOut => easing_functions::exponential_in_out(t),
            EaseKind::ElasticIn => easing_functions::elastic_in(t),
            EaseKind::ElasticOut => easing_functions::elastic_out(t),
            EaseKind::ElasticInOut => easing_functions::elastic_in_out(t),
            EaseKind::BackIn => easing_functions::back_in(t),
            EaseKind::BackOut => easing_functions::back_out(t),
            EaseKind::BackInOut => easing_functions::back_in_out(t),
            EaseKind::BounceIn => easing_functions::bounce_in(t),
            EaseKind::BounceOut => easing_functions::bounce_out(t),
            EaseKind::BounceInOut => easing_functions::bounce_in_out(t),
            EaseKind::Steps(num_steps) => easing_functions::steps(*num_steps, t),
            EaseKind::Elastic(omega) => easing_functions::elastic(*omega, t),
        }
    }
}

impl Interpolation for EaseKind {
    fn sample(&self, v: f32) -> f32 {
        self.sample(v)
    }
}

impl From<EaseFunction> for EaseKind {
    fn from(x: EaseFunction) -> Self {
        match x {
            EaseFunction::Linear => EaseKind::Linear,
            EaseFunction::QuadraticIn => EaseKind::QuadraticIn,
            EaseFunction::QuadraticOut => EaseKind::QuadraticOut,
            EaseFunction::QuadraticInOut => EaseKind::QuadraticInOut,
            EaseFunction::CubicIn => EaseKind::CubicIn,
            EaseFunction::CubicOut => EaseKind::CubicOut,
            EaseFunction::CubicInOut => EaseKind::CubicInOut,
            EaseFunction::QuarticIn => EaseKind::QuarticIn,
            EaseFunction::QuarticOut => EaseKind::QuarticOut,
            EaseFunction::QuarticInOut => EaseKind::QuarticInOut,
            EaseFunction::QuinticIn => EaseKind::QuinticIn,
            EaseFunction::QuinticOut => EaseKind::QuinticOut,
            EaseFunction::QuinticInOut => EaseKind::QuinticInOut,
            EaseFunction::SineIn => EaseKind::SineIn,
            EaseFunction::SineOut => EaseKind::SineOut,
            EaseFunction::SineInOut => EaseKind::SineInOut,
            EaseFunction::CircularIn => EaseKind::CircularIn,
            EaseFunction::CircularOut => EaseKind::CircularOut,
            EaseFunction::CircularInOut => EaseKind::CircularInOut,
            EaseFunction::ExponentialIn => EaseKind::ExponentialIn,
            EaseFunction::ExponentialOut => EaseKind::ExponentialOut,
            EaseFunction::ExponentialInOut => EaseKind::ExponentialInOut,
            EaseFunction::ElasticIn => EaseKind::ElasticIn,
            EaseFunction::ElasticOut => EaseKind::ElasticOut,
            EaseFunction::ElasticInOut => EaseKind::ElasticInOut,
            EaseFunction::BackIn => EaseKind::BackIn,
            EaseFunction::BackOut => EaseKind::BackOut,
            EaseFunction::BackInOut => EaseKind::BackInOut,
            EaseFunction::BounceIn => EaseKind::BounceIn,
            EaseFunction::BounceOut => EaseKind::BounceOut,
            EaseFunction::BounceInOut => EaseKind::BounceInOut,
            EaseFunction::Steps(x) => EaseKind::Steps(x),
            EaseFunction::Elastic(x) => EaseKind::Elastic(x),
        }
    }
}

/// Plugin for [`EaseClosure`]. In case you want to use custom an ease
/// function. Since most people likely wouldn't use this type, this plugin is
/// not with [`DefaultTweenPlugins`] to reduce unused system.
///
/// [`DefaultTweenPlugins`]: crate::DefaultTweenPlugins
pub struct EaseClosurePlugin;
impl Plugin for EaseClosurePlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        app.add_systems(
            app_resource.schedule,
            sample_interpolations_system::<EaseClosure>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
        );
    }
}

/// Use a custom easing function via a closure.
///
/// See [`EaseKind`].
#[derive(Component)]
pub struct EaseClosure(pub Box<dyn Fn(f32) -> f32 + Send + Sync + 'static>);

impl EaseClosure {
    /// Create new [`EaseClosure`]
    pub fn new<F: Fn(f32) -> f32 + Send + Sync + 'static>(f: F) -> EaseClosure {
        EaseClosure(Box::new(f))
    }
}

impl Default for EaseClosure {
    fn default() -> Self {
        EaseClosure::new(easing_functions::linear)
    }
}

impl Interpolation for EaseClosure {
    fn sample(&self, v: f32) -> f32 {
        self.0(v)
    }
}

/// This system will automatically sample in each entities with a
/// [`TimeSpanProgress`] component then insert [`TweenInterpolationValue`].
/// Remove [`TweenInterpolationValue`] if [`TimeSpanProgress`] is removed.
#[allow(clippy::type_complexity)]
pub fn sample_interpolations_system<I>(
    mut commands: Commands,
    query: Query<
        (Entity, &I, &TimeSpanProgress),
        Or<(Changed<I>, Changed<TimeSpanProgress>)>,
    >,
    mut removed: RemovedComponents<TimeSpanProgress>,
) where
    I: Interpolation + Component,
{
    query.iter().for_each(|(entity, interpolator, progress)| {
        if progress.now_percentage.is_nan() {
            return;
        }
        let value = interpolator.sample(progress.now_percentage.clamp(0., 1.));

        commands
            .entity(entity)
            .insert(TweenInterpolationValue(value));
    });
    removed.read().for_each(|entity| {
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<TweenInterpolationValue>();
        }
    });
}

mod easing_functions {
    use bevy::math::prelude::*;
    use ops::FloatPow;
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_3, PI};

    #[inline]
    pub(crate) fn linear(t: f32) -> f32 {
        t
    }

    #[inline]
    pub(crate) fn quadratic_in(t: f32) -> f32 {
        t.squared()
    }
    #[inline]
    pub(crate) fn quadratic_out(t: f32) -> f32 {
        1.0 - (1.0 - t).squared()
    }
    #[inline]
    pub(crate) fn quadratic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t.squared()
        } else {
            1.0 - (-2.0 * t + 2.0).squared() / 2.0
        }
    }

    #[inline]
    pub(crate) fn cubic_in(t: f32) -> f32 {
        t.cubed()
    }
    #[inline]
    pub(crate) fn cubic_out(t: f32) -> f32 {
        1.0 - (1.0 - t).cubed()
    }
    #[inline]
    pub(crate) fn cubic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t.cubed()
        } else {
            1.0 - (-2.0 * t + 2.0).cubed() / 2.0
        }
    }

    #[inline]
    pub(crate) fn quartic_in(t: f32) -> f32 {
        t * t * t * t
    }
    #[inline]
    pub(crate) fn quartic_out(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
    }
    #[inline]
    pub(crate) fn quartic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) * (-2.0 * t + 2.0) / 2.0
        }
    }

    #[inline]
    pub(crate) fn quintic_in(t: f32) -> f32 {
        t * t * t * t * t
    }
    #[inline]
    pub(crate) fn quintic_out(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t)
    }
    #[inline]
    pub(crate) fn quintic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                * (-2.0 * t + 2.0)
                / 2.0
        }
    }

    #[inline]
    pub(crate) fn sine_in(t: f32) -> f32 {
        1.0 - ops::cos(t * FRAC_PI_2)
    }
    #[inline]
    pub(crate) fn sine_out(t: f32) -> f32 {
        ops::sin(t * FRAC_PI_2)
    }
    #[inline]
    pub(crate) fn sine_in_out(t: f32) -> f32 {
        -(ops::cos(PI * t) - 1.0) / 2.0
    }

    #[inline]
    pub(crate) fn circular_in(t: f32) -> f32 {
        1.0 - (1.0 - t.squared()).sqrt()
    }
    #[inline]
    pub(crate) fn circular_out(t: f32) -> f32 {
        (1.0 - (t - 1.0).squared()).sqrt()
    }
    #[inline]
    pub(crate) fn circular_in_out(t: f32) -> f32 {
        if t < 0.5 {
            (1.0 - (1.0 - (2.0 * t).squared()).sqrt()) / 2.0
        } else {
            ((1.0 - (-2.0 * t + 2.0).squared()).sqrt() + 1.0) / 2.0
        }
    }

    #[inline]
    pub(crate) fn exponential_in(t: f32) -> f32 {
        ops::powf(2.0, 10.0 * t - 10.0)
    }
    #[inline]
    pub(crate) fn exponential_out(t: f32) -> f32 {
        1.0 - ops::powf(2.0, -10.0 * t)
    }
    #[inline]
    pub(crate) fn exponential_in_out(t: f32) -> f32 {
        if t < 0.5 {
            ops::powf(2.0, 20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - ops::powf(2.0, -20.0 * t + 10.0)) / 2.0
        }
    }

    #[inline]
    pub(crate) fn back_in(t: f32) -> f32 {
        let c = 1.70158;

        (c + 1.0) * t.cubed() - c * t.squared()
    }
    #[inline]
    pub(crate) fn back_out(t: f32) -> f32 {
        let c = 1.70158;

        1.0 + (c + 1.0) * (t - 1.0).cubed() + c * (t - 1.0).squared()
    }
    #[inline]
    pub(crate) fn back_in_out(t: f32) -> f32 {
        let c1 = 1.70158;
        let c2 = c1 + 1.525;

        if t < 0.5 {
            (2.0 * t).squared() * ((c2 + 1.0) * 2.0 * t - c2) / 2.0
        } else {
            ((2.0 * t - 2.0).squared() * ((c2 + 1.0) * (2.0 * t - 2.0) + c2) + 2.0) / 2.0
        }
    }

    #[inline]
    pub(crate) fn elastic_in(t: f32) -> f32 {
        -ops::powf(2.0, 10.0 * t - 10.0) * ops::sin((t * 10.0 - 10.75) * 2.0 * FRAC_PI_3)
    }
    #[inline]
    pub(crate) fn elastic_out(t: f32) -> f32 {
        ops::powf(2.0, -10.0 * t) * ops::sin((t * 10.0 - 0.75) * 2.0 * FRAC_PI_3) + 1.0
    }
    #[inline]
    pub(crate) fn elastic_in_out(t: f32) -> f32 {
        let c = (2.0 * PI) / 4.5;

        if t < 0.5 {
            -ops::powf(2.0, 20.0 * t - 10.0) * ops::sin((t * 20.0 - 11.125) * c) / 2.0
        } else {
            ops::powf(2.0, -20.0 * t + 10.0) * ops::sin((t * 20.0 - 11.125) * c) / 2.0 + 1.0
        }
    }

    #[inline]
    pub(crate) fn bounce_in(t: f32) -> f32 {
        1.0 - bounce_out(1.0 - t)
    }
    #[inline]
    pub(crate) fn bounce_out(t: f32) -> f32 {
        if t < 4.0 / 11.0 {
            (121.0 * t.squared()) / 16.0
        } else if t < 8.0 / 11.0 {
            (363.0 / 40.0 * t.squared()) - (99.0 / 10.0 * t) + 17.0 / 5.0
        } else if t < 9.0 / 10.0 {
            (4356.0 / 361.0 * t.squared()) - (35442.0 / 1805.0 * t) + 16061.0 / 1805.0
        } else {
            (54.0 / 5.0 * t.squared()) - (513.0 / 25.0 * t) + 268.0 / 25.0
        }
    }
    #[inline]
    pub(crate) fn bounce_in_out(t: f32) -> f32 {
        if t < 0.5 {
            (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
        }
    }

    #[inline]
    pub(crate) fn steps(num_steps: usize, t: f32) -> f32 {
        (t * num_steps as f32).round() / num_steps.max(1) as f32
    }

    #[inline]
    pub(crate) fn elastic(omega: f32, t: f32) -> f32 {
        1.0 - (1.0 - t).squared() * (2.0 * ops::sin(omega * t) / omega + ops::cos(omega * t))
    }
}
