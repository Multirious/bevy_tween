//! Module containing ease functions and related systems.
//!
//! # [`Interpolation`]
//!
//! **Built-in interpolations**:
//! - [`EaseFunction`]
//! - [`EaseClosure`]
//!
//! **Systems**:
//! - [`sample_interpolations_system`]

use bevy::prelude::*;

use crate::TweenSystemSet;
use bevy_time_runner::TimeSpanProgress;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy_lookup_curve")]
pub mod bevy_lookup_curve;
mod ease_function;

/// Automatically managed by an [`Interpolation`] such as [`EaseFunction`] and
/// [`EaseClosure`] when a tween has the component [`TimeSpanProgress`](bevy_time_runner::TimeSpanProgress).
/// See [`sample_interpolations_system`]
///
/// [`sample_interpolations_system`]: crate::interpolation::sample_interpolations_system
/// [`Interpolation`]: crate::interpolation::Interpolation
/// [`EaseFunction`]: crate::interpolation::EaseFunction
/// [`EaseClosure`]: crate::interpolation::EaseClosure
#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)] // might want to use sparseset but i'm not sure yet
pub struct CurveValue<V = f32>(pub V);

/// Plugin for [`EaseFunction`]
pub struct EaseFunctionPlugin;

impl Plugin for EaseFunctionPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            (
                ease_function_a_to_b_system::<f32>(|&a, &b, &v| a.lerp(b, v)),
                ease_function_a_to_b_system::<Vec2>(|&a, &b, &v| a.lerp(b, v)),
                ease_function_a_to_b_system::<Vec3>(|&a, &b, &v| a.lerp(b, v)),
                ease_function_a_to_b_system::<Quat>(|&a, &b, &v| a.slerp(b, v)),
                ease_function_a_to_b_system::<Color>(|&a, b, &v| a.mix(b, v)),
            )
                .in_set(TweenSystemSet::UpdateCurveValue),
        )
        .register_type::<EaseFunction>();
    }
}

// i'm very proud of this text art i made.
// it's some what better than images because we can see them in terminal editors too
// helix, vim, etc.

/// Robert Penner's easing functions in an enum
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum EaseFunction {
    #[default]
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗  
    ///  │                                    ▄▀▘  
    ///  │                                 ▄▞▀  ┆  
    ///  │                              ▗▄▀     ┆  
    ///  │                            ▄▀▘       ┆  
    ///  │                         ▄▞▀          ┆  
    ///  │                      ▗▄▀             ┆  
    ///  │                    ▄▀▘               ┆  
    ///  │                 ▄▞▀                  ┆  
    ///  │              ▗▄▀                     ┆  
    ///  │            ▄▀▘                       ┆  
    ///  │         ▄▞▀                          ┆  
    ///  │      ▗▄▀                             ┆  
    ///  │    ▄▀▘                               ┆  
    ///  │ ▄▞▀                                  ┆  
    ///  ▄▀─────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    Linear,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗▀  
    ///  │                                    ▄▘┆  
    ///  │                                  ▗▞  ┆  
    ///  │                                 ▄▘   ┆  
    ///  │                               ▗▞     ┆  
    ///  │                              ▞▘      ┆  
    ///  │                            ▄▀        ┆  
    ///  │                          ▄▀          ┆  
    ///  │                        ▄▀            ┆  
    ///  │                      ▄▀              ┆  
    ///  │                   ▄▞▀                ┆  
    ///  │                ▗▄▀                   ┆  
    ///  │             ▄▞▀▘                     ┆  
    ///  │        ▄▄▄▀▀                         ┆  
    ///  ▄▄▄▄▄▞▀▀▀──────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuadraticIn,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄  
    ///  │                           ▗▄▄▞▀▀▀    ┆  
    ///  │                        ▄▞▀▘          ┆  
    ///  │                    ▗▄▀▀              ┆  
    ///  │                  ▄▀▘                 ┆  
    ///  │               ▗▞▀                    ┆  
    ///  │             ▗▞▘                      ┆  
    ///  │           ▗▞▘                        ┆  
    ///  │         ▗▞▘                          ┆  
    ///  │        ▞▘                            ┆  
    ///  │      ▄▀                              ┆  
    ///  │    ▗▞                                ┆  
    ///  │   ▄▘                                 ┆  
    ///  │ ▗▞                                   ┆  
    ///  │▗▘                                    ┆  
    ///  ▞▘─────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuadraticOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗▄▄▄  
    ///  │                               ▄▄▀▀▘  ┆  
    ///  │                            ▗▞▀       ┆  
    ///  │                          ▄▀▘         ┆  
    ///  │                        ▄▀            ┆  
    ///  │                      ▗▞              ┆  
    ///  │                     ▞▘               ┆  
    ///  │                   ▗▞                 ┆  
    ///  │                  ▄▘                  ┆  
    ///  │                 ▞                    ┆  
    ///  │               ▄▀                     ┆  
    ///  │             ▗▞                       ┆  
    ///  │           ▗▞▘                        ┆  
    ///  │         ▄▀▘                          ┆  
    ///  │     ▗▄▞▀                             ┆  
    ///  ▄▄▄▄▀▀▘────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuadraticInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▞  
    ///  │                                     ▞┆  
    ///  │                                    ▞ ┆  
    ///  │                                   ▞  ┆  
    ///  │                                  ▞   ┆  
    ///  │                                ▗▞    ┆  
    ///  │                               ▗▘     ┆  
    ///  │                              ▞▘      ┆  
    ///  │                            ▗▀        ┆  
    ///  │                          ▗▞▘         ┆  
    ///  │                        ▗▞▘           ┆  
    ///  │                      ▄▞▘             ┆  
    ///  │                   ▄▞▀                ┆  
    ///  │              ▄▄▞▀▀                   ┆  
    ///  ▄▄▄▄▄▄▄▄▄▄▞▀▀▀▀────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    CubicIn,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄▄▄▄▄▄  
    ///  │                      ▄▄▞▀▀▀▀         ┆  
    ///  │                  ▄▞▀▀                ┆  
    ///  │               ▄▞▀                    ┆  
    ///  │             ▄▀                       ┆  
    ///  │           ▄▀                         ┆  
    ///  │         ▗▀                           ┆  
    ///  │        ▞▘                            ┆  
    ///  │      ▗▀                              ┆  
    ///  │     ▄▘                               ┆  
    ///  │    ▞                                 ┆  
    ///  │   ▞                                  ┆  
    ///  │  ▞                                   ┆  
    ///  │ ▞                                    ┆  
    ///  │▞                                     ┆  
    ///  ▞──────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    CubicOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄▄  
    ///  │                            ▄▄▀▀▀     ┆  
    ///  │                          ▄▀          ┆  
    ///  │                        ▄▀            ┆  
    ///  │                      ▗▞              ┆  
    ///  │                     ▗▘               ┆  
    ///  │                    ▗▘                ┆  
    ///  │                   ▗▘                 ┆  
    ///  │                  ▗▘                  ┆  
    ///  │                 ▗▘                   ┆  
    ///  │                ▗▘                    ┆  
    ///  │               ▄▘                     ┆  
    ///  │             ▗▞                       ┆  
    ///  │           ▗▞▘                        ┆  
    ///  │        ▗▄▞▘                          ┆  
    ///  ▄▄▄▄▄▄▞▀▀▘─────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    CubicInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▐  
    ///  │                                     ▗▘  
    ///  │                                     ▞┆  
    ///  │                                    ▞ ┆  
    ///  │                                   ▞  ┆  
    ///  │                                  ▞   ┆  
    ///  │                                 ▞    ┆  
    ///  │                                ▞     ┆  
    ///  │                              ▗▞      ┆  
    ///  │                             ▗▘       ┆  
    ///  │                           ▗▞▘        ┆  
    ///  │                         ▗▞▘          ┆  
    ///  │                      ▗▄▀▘            ┆  
    ///  │                  ▄▄▞▀▘               ┆  
    ///  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▞▀▀▀▀────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuarticIn,
    /// ```txt
    ///  ▲                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  
    ///  │                  ▄▄▞▀▀▀▀             ┆  
    ///  │              ▗▄▀▀                    ┆  
    ///  │            ▄▀▘                       ┆  
    ///  │          ▄▀                          ┆  
    ///  │        ▗▀                            ┆  
    ///  │       ▄▘                             ┆  
    ///  │      ▞                               ┆  
    ///  │     ▞                                ┆  
    ///  │    ▞                                 ┆  
    ///  │   ▞                                  ┆  
    ///  │  ▞                                   ┆  
    ///  │ ▞                                    ┆  
    ///  │▗▘                                    ┆  
    ///  │▌                                     ┆  
    ///  ▞──────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuarticOut,
    /// ```txt
    ///  ▲                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗▄▄▄▄▄▄▄▄  
    ///  │                          ▗▄▀▀▘       ┆  
    ///  │                        ▗▞▘           ┆  
    ///  │                       ▞▘             ┆  
    ///  │                      ▞               ┆  
    ///  │                     ▞                ┆  
    ///  │                    ▞                 ┆  
    ///  │                   ▗▘                 ┆  
    ///  │                  ▗▘                  ┆  
    ///  │                  ▞                   ┆  
    ///  │                 ▞                    ┆  
    ///  │                ▞                     ┆  
    ///  │               ▞                      ┆  
    ///  │             ▄▀                       ┆  
    ///  │          ▗▄▀                         ┆  
    ///  ▄▄▄▄▄▄▄▄▄▀▀▘───────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuarticInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗  
    ///  │                                      ▞  
    ///  │                                     ▐┆  
    ///  │                                    ▗▘┆  
    ///  │                                    ▞ ┆  
    ///  │                                   ▞  ┆  
    ///  │                                  ▐   ┆  
    ///  │                                 ▗▘   ┆  
    ///  │                                ▗▘    ┆  
    ///  │                               ▞▘     ┆  
    ///  │                             ▗▞       ┆  
    ///  │                           ▗▞▘        ┆  
    ///  │                         ▗▞▘          ┆  
    ///  │                     ▄▄▞▀▘            ┆  
    ///  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀▀▀▀─────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuinticIn,
    /// ```txt
    ///  ▲                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  
    ///  │               ▄▄▞▀▀▀▘                ┆  
    ///  │            ▄▀▀                       ┆  
    ///  │          ▄▀                          ┆  
    ///  │        ▄▀                            ┆  
    ///  │       ▞                              ┆  
    ///  │     ▗▀                               ┆  
    ///  │    ▗▘                                ┆  
    ///  │    ▌                                 ┆  
    ///  │   ▞                                  ┆  
    ///  │  ▞                                   ┆  
    ///  │ ▗▘                                   ┆  
    ///  │ ▌                                    ┆  
    ///  │▞                                     ┆  
    ///  ▗▘                                     ┆  
    ///  ▞──────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuinticOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄▄▄▄▄▄  
    ///  │                         ▗▞▀▀┄┄┄┄┄┄┄┄┄┄  
    ///  │                       ▗▞▘            ┆  
    ///  │                      ▗▘              ┆  
    ///  │                     ▗▘               ┆  
    ///  │                    ▗▘                ┆  
    ///  │                    ▞                 ┆  
    ///  │                   ▐                  ┆  
    ///  │                   ▌                  ┆  
    ///  │                  ▞                   ┆  
    ///  │                 ▗▘                   ┆  
    ///  │                ▗▘                    ┆  
    ///  │               ▗▘                     ┆  
    ///  │              ▄▘                      ┆  
    ///  │            ▄▀                        ┆  
    ///  ▄▄▄▄▄▄▄▄▄▄▞▀▀──────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    QuinticInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▀  
    ///  │                                   ▗▞ ┆  
    ///  │                                 ▗▞▘  ┆  
    ///  │                                ▄▘    ┆  
    ///  │                              ▄▀      ┆  
    ///  │                            ▗▀        ┆  
    ///  │                          ▗▞▘         ┆  
    ///  │                        ▗▞▘           ┆  
    ///  │                      ▗▞▘             ┆  
    ///  │                    ▄▀▘               ┆  
    ///  │                 ▗▄▀                  ┆  
    ///  │               ▄▞▘                    ┆  
    ///  │           ▗▄▞▀                       ┆  
    ///  │       ▗▄▞▀▘                          ┆  
    ///  ▄▄▄▄▄▀▀▀▘──────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    SineIn,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗▄▄▄▄  
    ///  │                             ▄▄▀▀▀▘   ┆  
    ///  │                         ▄▄▀▀         ┆  
    ///  │                      ▄▞▀             ┆  
    ///  │                   ▗▄▀                ┆  
    ///  │                 ▗▞▘                  ┆  
    ///  │               ▄▀▘                    ┆  
    ///  │             ▄▀                       ┆  
    ///  │           ▄▀                         ┆  
    ///  │         ▗▀                           ┆  
    ///  │       ▗▞▘                            ┆  
    ///  │     ▗▞▘                              ┆  
    ///  │    ▄▘                                ┆  
    ///  │  ▄▀                                  ┆  
    ///  │▗▞                                    ┆  
    ///  ▞▘─────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    SineOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄  
    ///  │                                ▄▞▀▀▘ ┆  
    ///  │                             ▄▞▀      ┆  
    ///  │                           ▄▀         ┆  
    ///  │                         ▄▀           ┆  
    ///  │                       ▄▀             ┆  
    ///  │                     ▗▀               ┆  
    ///  │                   ▗▞▘                ┆  
    ///  │                  ▄▘                  ┆  
    ///  │                ▗▀                    ┆  
    ///  │              ▗▞▘                     ┆  
    ///  │            ▗▞▘                       ┆  
    ///  │          ▗▞▘                         ┆  
    ///  │        ▄▞▘                           ┆  
    ///  │     ▄▞▀                              ┆  
    ///  ▄▄▄▞▀▀─────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    SineInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄  
    ///  │                                      ▐  
    ///  │                                     ▗▘  
    ///  │                                    ▗▘┆  
    ///  │                                   ▗▘ ┆  
    ///  │                                  ▗▘  ┆  
    ///  │                                ▗▞▘   ┆  
    ///  │                              ▗▞▘     ┆  
    ///  │                            ▗▞▘       ┆  
    ///  │                         ▗▄▀▘         ┆  
    ///  │                      ▗▄▀▘            ┆  
    ///  │                  ▄▄▞▀▘               ┆  
    ///  │           ▗▄▄▄▀▀▀                    ┆  
    ///  ▄▄▄▄▄▄▄▞▀▀▀▀▘──────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    CircularIn,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄▄▄  
    ///  │                       ▗▄▄▄▀▀▀▀▀▘     ┆  
    ///  │                  ▄▄▞▀▀▘              ┆  
    ///  │              ▗▄▀▀                    ┆  
    ///  │           ▗▄▀▘                       ┆  
    ///  │         ▄▀▘                          ┆  
    ///  │       ▄▀                             ┆  
    ///  │     ▄▀                               ┆  
    ///  │   ▗▀                                 ┆  
    ///  │  ▗▘                                  ┆  
    ///  │ ▗▘                                   ┆  
    ///  │▗▘                                    ┆  
    ///  │▌                                     ┆  
    ///  ▐                                      ┆  
    ///  ▐                                      ┆  
    ///  ▞──────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    CircularOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄  
    ///  │                            ▄▄▞▀▀▀▘   ┆  
    ///  │                         ▄▞▀          ┆  
    ///  │                       ▄▀             ┆  
    ///  │                     ▄▀               ┆  
    ///  │                    ▞                 ┆  
    ///  │                   ▗▘                 ┆  
    ///  │                   ▐                  ┆  
    ///  │                   ▌                  ┆  
    ///  │                  ▗▘                  ┆  
    ///  │                  ▞                   ┆  
    ///  │                ▗▞                    ┆  
    ///  │              ▗▞▘                     ┆  
    ///  │            ▄▞▘                       ┆  
    ///  │        ▄▄▞▀                          ┆  
    ///  ▄▄▄▄▄▞▀▀▀──────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    CircularInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗  
    ///  │                                      ▐  
    ///  │                                      ▌  
    ///  │                                     ▐┆  
    ///  │                                     ▌┆  
    ///  │                                    ▞ ┆  
    ///  │                                   ▗▘ ┆  
    ///  │                                  ▗▘  ┆  
    ///  │                                 ▗▘   ┆  
    ///  │                                ▗▘    ┆  
    ///  │                               ▄▘     ┆  
    ///  │                             ▗▞       ┆  
    ///  │                           ▄▞▘        ┆  
    ///  │                      ▄▄▄▀▀           ┆  
    ///  ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▞▀▀▀▀▀▀────────────────┬─▶
    /// 0                                       1  
    /// ```
    ExponentialIn,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  
    ///  │             ▗▄▄▞▀▀▀▀▀▀▘              ┆  
    ///  │          ▄▞▀▘                        ┆  
    ///  │        ▄▀                            ┆  
    ///  │      ▗▀                              ┆  
    ///  │     ▗▘                               ┆  
    ///  │    ▗▘                                ┆  
    ///  │   ▗▘                                 ┆  
    ///  │  ▗▘                                  ┆  
    ///  │  ▞                                   ┆  
    ///  │ ▐                                    ┆  
    ///  │ ▌                                    ┆  
    ///  │▐                                     ┆  
    ///  │▌                                     ┆  
    ///  ▗▘                                     ┆  
    ///  ▞──────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    ExponentialOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▄▄▄▄▄▄▄▄▄▄  
    ///  │                        ▗▄▀▀▀▘        ┆  
    ///  │                      ▗▞▘             ┆  
    ///  │                     ▗▘               ┆  
    ///  │                     ▌                ┆  
    ///  │                    ▐                 ┆  
    ///  │                    ▌                 ┆  
    ///  │                   ▐                  ┆  
    ///  │                   ▌                  ┆  
    ///  │                  ▐                   ┆  
    ///  │                  ▌                   ┆  
    ///  │                 ▞                    ┆  
    ///  │                ▗▘                    ┆  
    ///  │               ▄▘                     ┆  
    ///  │            ▗▄▀                       ┆  
    ///  ▄▄▄▄▄▄▄▄▄▄▞▀▀▘─────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    ExponentialInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄  
    ///  │                                      ┆  
    ///  │                                      ┆  
    ///  │                                      ┆  
    ///  │                                      ┆  
    ///  │                                    ▐▖┆  
    ///  │                                    ▐▐┆  
    ///  │                                    ▐▐┆  
    ///  │                                 ▗  ▐▐┆  
    ///  │                                 ▐▌ ▐▐┆  
    ///  │                              ▗  ▐▐ ▐▐┆  
    ///  │                              ▐▌ ▐▐ ▐▐┆  
    ///  │                           ▐▖ ▞▚ ▞▐ ▞▐┆  
    ///  │                  ▄  ▄  ▛▖ ▌▚ ▌▐ ▌▐ ▌▐┆  
    ///  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▚▞─▚▞─▚▐─▐▗▘▐─▌▐─▌▐─▌▐┬─▶
    /// 0                        ▘ ▝▞ ▐▗▘▐ ▌▐ ▌▝▖  
    ///                                ▛  ▌▌ ▌▌ ▌  
    ///                                   █  ▌▌ ▌  
    ///                                   ▛  ▌▌ ▌  
    ///                                      ▌▌ ▌  
    ///                                      █  ▌  
    ///                                      ▘  ▌  
    ///                                         ▌  
    ///                                         ▌  
    ///                                         ▌  
    ///                                         ▚  
    /// ```
    ElasticIn,
    /// ```txt
    ///    ▗                                       
    ///    ▌▌                                      
    ///    ▌▌                                      
    ///    ▌▌                                      
    ///    ▌▌ ▐▖                                   
    ///    ▌▌ ▌▌                                   
    ///    ▌▌ ▌▌ ▐▖                                
    ///  ▲ ▌▌ ▌▌ ▞▌ ▗                              
    ///  │ ▌▚ ▌▚ ▌▚ ▞▌ ▗▖                          
    /// 1┤┄▌▐┄▌▐┄▌▐┄▌▐┄▌▐┄▞▚┄▞▚┄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄  
    ///  ▐▗▘▐ ▌▐ ▌▐ ▌▝▄▘ ▛  ▀  ▀                ┆  
    ///  ▐▐ ▐▐ ▐▐ ▝▟  ▘                         ┆  
    ///  ▐▐ ▐▐ ▐▐  ▘                            ┆  
    ///  ▐▐ ▐▐  █                               ┆  
    ///  ▐▐ ▐▐  ▘                               ┆  
    ///  ▐▐ ▐▐                                  ┆  
    ///  ▐▐  ▛                                  ┆  
    ///  ▐▐                                     ┆  
    ///  ▐▐                                     ┆  
    ///  ▐▐                                     ┆  
    ///  ▐▞                                     ┆  
    ///  ▝                                      ┆  
    ///  │                                      ┆  
    ///  │                                      ┆  
    ///  ───────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    ElasticOut,
    /// ```txt
    ///                       ▖                    
    ///                       █                    
    ///                       █                    
    ///  ▲                    █▐▖                  
    ///  │                    █▐▚▙▗▖▗ ▖            
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▙▌█▐▌█▐▌▛▝▀▀▀▀▀▀▀▀▀▀  
    ///  │                   █▌█▐▌▛▝            ┆  
    ///  │                   █▌█▝               ┆  
    ///  │                   █▌▛                ┆  
    ///  │                   █▌                 ┆  
    ///  │                   █▘                 ┆  
    ///  │                   ▌                  ┆  
    ///  │                   ▌                  ┆  
    ///  │                   ▌                  ┆  
    ///  │                   ▌                  ┆  
    ///  │                  ▖▌                  ┆  
    ///  │                  █▌                  ┆  
    ///  │                  █▌                  ┆  
    ///  │                ▐▖█▌                  ┆  
    ///  │          ▖▗ ▄▗▚▌▙▜▌                  ┆  
    ///  ▀▀▀▀▀▀▀▀▀▀▀▝▞▚▌█▐▌█▐▌──────────────────┬─▶
    /// 0             ▝ ▛▐▌█▐▌                  1  
    ///                 ▝ █▐▌                     
    ///                   ▛▐▌                     
    ///                    ▐▌                     
    ///                    ▐▘                     
    /// ```
    ElasticInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▐  
    ///  │                                      ▌  
    ///  │                                     ▗▘  
    ///  │                                     ▞┆  
    ///  │                                    ▗▘┆  
    ///  │                                    ▞ ┆  
    ///  │                                   ▗▘ ┆  
    ///  │                                   ▌  ┆  
    ///  │                                  ▐   ┆  
    ///  │                                  ▌   ┆  
    ///  │                                 ▞    ┆  
    ///  │                                ▗▘    ┆  
    ///  │                               ▗▘     ┆  
    ///  │                               ▞      ┆  
    ///  ▀▀▀▄▄▖─────────────────────────▞───────┬─▶
    /// 0     ▝▀▄▖                     ▞        1  
    ///          ▝▀▄                  ▞            
    ///             ▀▚▖             ▗▞             
    ///               ▝▀▄▖         ▄▘              
    ///                  ▝▀▄▄▄▄▄▄▞▀                
    /// ```
    BackIn,
    /// ```txt
    ///                ▗▄▀▀▀▀▀▀▚▄                  
    ///               ▞▘         ▀▚▄               
    ///             ▗▀              ▀▄▖            
    ///  ▲         ▗▘                 ▝▚▄          
    ///  │        ▗▘                     ▀▚▄       
    /// 1┤┄┄┄┄┄┄┄▗▘┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▀▀▚▄▄  
    ///  │       ▌                              ┆  
    ///  │      ▞                               ┆  
    ///  │     ▐                                ┆  
    ///  │    ▗▘                                ┆  
    ///  │    ▞                                 ┆  
    ///  │   ▗▘                                 ┆  
    ///  │   ▞                                  ┆  
    ///  │  ▐                                   ┆  
    ///  │  ▌                                   ┆  
    ///  │ ▐                                    ┆  
    ///  │ ▌                                    ┆  
    ///  │▐                                     ┆  
    ///  │▞                                     ┆  
    ///  ▗▘                                     ┆  
    ///  ▞──────────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    BackOut,
    /// ```txt
    ///                              ▗▄▖           
    ///  ▲                         ▗▀▘ ▝▀▚▄        
    ///  │                        ▞▘       ▀▄▖     
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▐┄┄┄┄┄┄┄┄┄┄┄▝▀▄▄  
    ///  │                      ▗▘              ┆  
    ///  │                      ▌               ┆  
    ///  │                     ▐                ┆  
    ///  │                     ▌                ┆  
    ///  │                    ▐                 ┆  
    ///  │                    ▌                 ┆  
    ///  │                   ▐                  ┆  
    ///  │                   ▌                  ┆  
    ///  │                  ▐                   ┆  
    ///  │                  ▌                   ┆  
    ///  │                 ▐                    ┆  
    ///  │                 ▌                    ┆  
    ///  │                ▐                     ┆  
    ///  │               ▗▘                     ┆  
    ///  ▄▄▖─────────────▌──────────────────────┬─▶
    /// 0  ▝▀▄▖         ▞                       1  
    ///       ▝▚▄     ▗▀                           
    ///          ▀▀▄▄▀▘                            
    /// ```
    BackInOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗▄  
    ///  │                                   ▄▀▘┆  
    ///  │                                 ▗▞   ┆  
    ///  │                                ▗▘    ┆  
    ///  │                               ▗▘     ┆  
    ///  │                              ▗▘      ┆  
    ///  │                              ▌       ┆  
    ///  │                             ▞        ┆  
    ///  │                            ▗▘        ┆  
    ///  │                            ▌         ┆  
    ///  │                           ▞          ┆  
    ///  │              ▗▞▀▀▀▄▖     ▗▘          ┆  
    ///  │             ▞▘     ▝▚    ▞           ┆  
    ///  │           ▗▞         ▚  ▗▘           ┆  
    ///  │    ▗▄▀▚▄  ▞           ▚ ▞            ┆  
    ///  ▄▀▀▚▞▘────▀▞─────────────▚▘────────────┬─▶
    /// 0                                       1  
    /// ```
    BounceIn,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄▗┄┄┄┄┄┄┄┄┄┄┄┄┄┄▖┄┄┄┄┄┄▄┄┄▗  
    ///  │             ▞▚            ▞▝▚▄ ▗▄▀ ▀▀▘  
    ///  │            ▗▘ ▚          ▄▘   ▀▘     ┆  
    ///  │            ▞   ▚        ▞            ┆  
    ///  │           ▗▘    ▀▄▖   ▄▀             ┆  
    ///  │           ▞       ▝▀▀▀               ┆  
    ///  │          ▐                           ┆  
    ///  │         ▗▘                           ┆  
    ///  │         ▞                            ┆  
    ///  │        ▐                             ┆  
    ///  │       ▗▘                             ┆  
    ///  │      ▗▘                              ┆  
    ///  │     ▗▘                               ┆  
    ///  │    ▄▘                                ┆  
    ///  │  ▗▞                                  ┆  
    ///  ▄▄▀▘───────────────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    BounceOut,
    /// ```txt
    ///  ▲                                         
    ///  │                                         
    /// 1┤┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▗┄┄┄▄▄  
    ///  │                          ▛▖    ▗▘▀▀▀ ┆  
    ///  │                         ▐ ▝▚▄▄▄▘     ┆  
    ///  │                        ▗▘            ┆  
    ///  │                        ▞             ┆  
    ///  │                       ▞              ┆  
    ///  │                      ▄▘              ┆  
    ///  │                    ▗▞                ┆  
    ///  │                 ▄▀▀▘                 ┆  
    ///  │               ▗▞                     ┆  
    ///  │               ▞                      ┆  
    ///  │              ▞                       ┆  
    ///  │             ▗▘                       ┆  
    ///  │      ▗▄▄▄   ▌                        ┆  
    ///  │     ▗▘   ▀▖▐                         ┆  
    ///  ▄▄▞▀▀▚▘─────▝▘─────────────────────────┬─▶
    /// 0                                       1  
    /// ```
    BounceInOut,
}
impl EaseFunction {
    /// Sample a value from this ease function.
    pub fn sample(&self, v: f32) -> f32 {
        use ease_function::*;
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
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            ease_closure_system.in_set(TweenSystemSet::UpdateCurveValue),
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
        EaseClosure::new(ease_function::linear)
    }
}

#[derive(Component)]
pub struct AToB<V, C> {
    pub a: V,
    pub b: V,
    pub curve: C,
}

#[allow(clippy::type_complexity)]
pub fn ease_function_a_to_b_system<V>(
    f: impl Send + Sync + 'static + Fn(&V, &V, &f32) -> V,
) -> impl Fn(
    Commands,
    Query<
        (Entity, &AToB<V, EaseFunction>, &TimeSpanProgress),
        Or<(Changed<EaseFunction>, Changed<TimeSpanProgress>)>,
    >,
    RemovedComponents<TimeSpanProgress>,
)
where
    V: Send + Sync + 'static,
{
    move |mut commands, query, mut removed| {
        query.iter().for_each(|(entity, a_to_b, progress)| {
            if progress.now_percentage.is_nan() {
                return;
            }
            let value = a_to_b.curve.sample(progress.now_percentage);

            commands
                .entity(entity)
                .insert(CurveValue(f(&a_to_b.a, &a_to_b.b, &value)));
        });
        removed.read().for_each(|entity| {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.remove::<CurveValue<V>>();
            }
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn ease_closure_system(
    mut commands: Commands,
    query: Query<
        (Entity, &EaseClosure, &TimeSpanProgress),
        Or<(Changed<EaseClosure>, Changed<TimeSpanProgress>)>,
    >,
    mut removed: RemovedComponents<TimeSpanProgress>,
) {
    query.iter().for_each(|(entity, ease_closure, progress)| {
        if progress.now_percentage.is_nan() {
            return;
        }
        let value = ease_closure.0(progress.now_percentage.clamp(0., 1.));

        commands.entity(entity).insert(CurveValue(value));
    });
    removed.read().for_each(|entity| {
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<CurveValue>();
        }
    });
}
