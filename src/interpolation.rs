//! Module containing ease functions and related systems.
//!
//! # [`Interpolation`]
//! - [`EaseFunction`]
//! - [`EaseClosure`]

use bevy::prelude::*;

use crate::{
    tween::{TweenInterpolationValue, TweenProgressed},
    TweenSystemSet,
};

mod ease_functions;

/// A trait for implementing interpolation algorithms.
/// Use with [`sample_interpolations_system`]
pub trait Interpolation {
    /// Sample a value from this algorithm.
    /// Input should be between 0 to 1 and returns value that should be
    /// between 0 to 1
    fn sample(&self, v: f32) -> f32;
}

/// Plugin for [`EaseFunction`]
pub struct EaseFunctionPlugin;
impl Plugin for EaseFunctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            sample_interpolations_system::<EaseFunction>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
        );
    }
}

/// Easing functions put into an enum.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub enum EaseFunction {
    #[default]
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SineIn,
    SineOut,
    SineInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}
impl EaseFunction {
    /// Sample a value from this ease function.
    pub fn sample(&self, v: f32) -> f32 {
        use ease_functions::*;
        use EaseFunction::*;
        match self {
            Linear => linear(v),
            QuadraticIn => quadratic_in(v),
            QuadraticOut => quadratic_out(v),
            QuadraticInOut => quadratic_in_out(v),
            CubicIn => cubic_in(v),
            CubicOut => cubic_out(v),
            CubicInOut => cubic_in_out(v),
            QuarticIn => quartic_in(v),
            QuarticOut => quartic_out(v),
            QuarticInOut => quartic_in_out(v),
            QuinticIn => quintic_in(v),
            QuinticOut => quintic_out(v),
            QuinticInOut => quintic_in_out(v),
            SineIn => sine_in(v),
            SineOut => sine_out(v),
            SineInOut => sine_in_out(v),
            CircularIn => circular_in(v),
            CircularOut => circular_out(v),
            CircularInOut => circular_in_out(v),
            ExponentialIn => exponential_in(v),
            ExponentialOut => exponential_out(v),
            ExponentialInOut => exponential_in_out(v),
            ElasticIn => elastic_in(v),
            ElasticOut => elastic_out(v),
            ElasticInOut => elastic_in_out(v),
            BackIn => back_in(v),
            BackOut => back_out(v),
            BackInOut => back_in_out(v),
            BounceIn => bounce_in(v),
            BounceOut => bounce_out(v),
            BounceInOut => bounce_in_out(v),
        }
    }
}

impl Interpolation for EaseFunction {
    fn sample(&self, v: f32) -> f32 {
        self.sample(v)
    }
}

/// Plugin for [`EaseClosure`]. In case you want to use custom an ease
/// function. Since most people likely wouldn't use this type, this plugin is
/// not with [`DefaultTweenPlugins`] to reduce unused system.
///
/// [`DefaultTweenPlugins`]: crate::DefaultTweenPlugins
pub struct EaseClosurePlugin;
impl Plugin for EaseClosurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            sample_interpolations_system::<EaseClosure>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
        );
    }
}

/// Use a custom easing function via a closure.
///
/// See [`EaseFunction`].
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
        EaseClosure::new(ease_functions::linear)
    }
}

impl Interpolation for EaseClosure {
    fn sample(&self, v: f32) -> f32 {
        self.0(v)
    }
}

/// This system will automatically sample in each entities with a
/// [`TweenProgressed`] component then insert [`TweenInterpolationValue`].
/// Remove [`TweenInterpolationValue`] if [`TweenProgressed`] is removed.
#[allow(clippy::type_complexity)]
pub fn sample_interpolations_system<I>(
    mut commands: Commands,
    query: Query<
        (Entity, &I, &TweenProgressed),
        Or<(Changed<I>, Changed<TweenProgressed>)>,
    >,
    mut removed: RemovedComponents<TweenProgressed>,
) where
    I: Interpolation + Component,
{
    query.iter().for_each(|(entity, interpolator, progressed)| {
        let value = interpolator.sample(progressed.0);
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
