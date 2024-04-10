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

use crate::{tween::TweenInterpolationValue, TweenSystemSet};
use bevy_time_runner::TimeSpanProgress;

mod ease_functions;

/// A trait for implementing interpolation algorithms.
///
/// Currently only used for registering [`sample_interpolations_system`].
pub trait Interpolation {
    /// Sample a value from this algorithm.
    /// Input should be between 0–1 and returns value that should be
    /// between 0–1
    fn sample(&self, v: f32) -> f32;
}

/// A trait for implementing interpolation algorithms that requires mutable access.
///
/// Currently only used for registering [`sample_interpolations_mut_system`].
pub trait InterpolationMut {
    /// Sample a value from this algorithm mutably.
    /// Input should be between 0–1 and returns value that should be
    /// between 0–1
    fn sample_mut(&mut self, v: f32) -> f32;
}

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
            .world
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        app.add_systems(
            app_resource.schedule,
            sample_interpolations_system::<EaseFunction>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
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
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world
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

/// Use [`bevy_lookup_curve`] for interpolation.
pub struct LookupCurveInterpolationPlugin;

impl Plugin for LookupCurveInterpolationPlugin {
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        app.add_systems(
            app_resource.schedule,
            sample_interpolations_mut_system::<LookupCurveCached>
                .in_set(TweenSystemSet::UpdateInterpolationValue),
        );
    }
}

/// Use [`LookupCurve`](bevy_lookup_curve::LookupCurve) for interpolation with cache.
#[derive(Default, Component)]
pub struct LookupCurveCached {
    /// The [`LookupCurve`](bevy_lookup_curve::LookupCurve) that will be used
    /// for interpolation
    pub curve: bevy_lookup_curve::LookupCurve,
    cache: bevy_lookup_curve::LookupCache,
}

impl LookupCurveCached {
    /// Create new [`LookupCurveCached`] with new cache inside
    pub fn new(curve: bevy_lookup_curve::LookupCurve) -> LookupCurveCached {
        LookupCurveCached {
            curve,
            cache: bevy_lookup_curve::LookupCache::new(),
        }
    }
}

impl InterpolationMut for LookupCurveCached {
    fn sample_mut(&mut self, v: f32) -> f32 {
        self.curve.lookup_cached(v, &mut self.cache)
    }
}

/// Use [`LookupCurve`](bevy_lookup_curve::LookupCurve) for interpolation.
#[derive(Default, Component)]
pub struct LookupCurve {
    /// The [`LookupCurve`](bevy_lookup_curve::LookupCurve) that will be used
    /// for interpolation
    pub curve: bevy_lookup_curve::LookupCurve,
}

impl LookupCurve {
    /// Create new [`LookupCurve`]
    pub fn new(curve: bevy_lookup_curve::LookupCurve) -> LookupCurve {
        LookupCurve { curve }
    }
}

impl Interpolation for LookupCurve {
    fn sample(&self, v: f32) -> f32 {
        self.curve.lookup(v)
    }
}

/// This system will automatically sample in each entities with a
/// [`TweenProgress`] component then insert [`TweenInterpolationValue`].
/// Remove [`TweenInterpolationValue`] if [`TweenProgress`] is removed.
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
    remove_removed(&mut commands, &mut removed);
}

/// This system will automatically sample mutably in each entities
/// with a [`TweenProgress`] component then insert [`TweenInterpolationValue`].
/// Remove [`TweenInterpolationValue`] if [`TweenProgress`] is removed.
#[allow(clippy::type_complexity)]
pub fn sample_interpolations_mut_system<I>(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut I, &TweenProgress),
        Or<(Changed<I>, Changed<TweenProgress>)>,
    >,
    mut removed: RemovedComponents<TweenProgress>,
) where
    I: InterpolationMut + Component,
{
    query
        .iter_mut()
        .for_each(|(entity, mut interpolator, progress)| {
            if progress.now_percentage.is_nan() {
                return;
            }
            let value =
                interpolator.sample_mut(progress.now_percentage.clamp(0., 1.));

            commands
                .entity(entity)
                .insert(TweenInterpolationValue(value));
        });
    remove_removed(&mut commands, &mut removed);
}

/// idk how to name this
fn remove_removed(
    commands: &mut Commands,
    removed: &mut RemovedComponents<TweenProgress>,
) {
    removed.read().for_each(|entity| {
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<TweenInterpolationValue>();
        }
    });
}
