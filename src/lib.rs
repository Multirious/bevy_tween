//! All examples in this crate will be using the [`span_tween`] implementation
//! which requires the feature "span_tween" and it is enabled by default.
//!
//! # Getting started
//!
//! [`DefaultTweenPlugins`] provide most the stuff
//! you will need.
//! Add the plugin to your app:
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_tween::*;
//!
//! fn main() {
//!     App::default()
//!         .add_plugins((DefaultPlugins, DefaultTweenPlugins))
//!         .run();
//! }
//! ```
//!
//! # Tween and Tweener
//!
//! Tweener handles the current actual playback timing of any tweens that
//! it's responsible for.
//!
//! Tween is your animation parameters that declares:
//! - "**What**" to interpolate, such as [`TargetComponent`], [`TargetAsset`], and
//!   [`TargetResource`].
//! - "**How**" to interpolate, such as [`interpolate::Translation`] and
//!   [`interpolate::SpriteColor`]. And they're used with something like [`EaseFunction`]
//! - "**When**" to interpolate such as [`TweenTimeSpan`].
//!
//! # Child-Parent Hierarchy
//!
//! This crate let you create paramters for your animation by using child-parent
//! hierarchy. This has the benefit of exposing the whole entity structure and
//! let users modify anything they wanted while also being flexible.
//! The specific entity structure is based on the specific tweener implementation.
//! You may want to see [`span_tween`]
//!
//! # Examples
//!
//! ## Custom interpolator quick example
//!
//! See ["Your own interpolator"](crate::interpolate#your-own-interpolator).
//! See ["Registering systems"](crate::tween#registering-systems).
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_tween::prelude::*;
//!
//! #[derive(Component)]
//! struct Foo {
//!     a: f32,
//!     b: f32,
//! }
//!
//! struct InterpolateFooA {
//!     start: f32,
//!     end: f32,
//! }
//!
//! impl Interpolator for InterpolateFooA {
//!     type Item = Foo;
//!
//!     fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!         item.a = self.start.lerp(self.end, value);
//!     }
//! }
//!
//! struct InterpolateFooB {
//!     start: f32,
//!     end: f32,
//! }
//!
//! impl Interpolator for InterpolateFooB {
//!     type Item = Foo;
//!
//!     fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!         item.b = self.start.lerp(self.end, value);
//!     }
//! }
//!
//! fn main() {
//!     App::new().add_tween_systems((
//!         bevy_tween::component_tween_system::<BoxedInterpolator<Foo>>,
//!         bevy_tween::component_tween_system::<InterpolateFooA>,
//!         bevy_tween::component_tween_system::<InterpolateFooB>,
//!     ));
//! }
//! ```
//!
//! ## Usage
//!
//! Run `cargo run --example span_tween` to see this in action.
//! ```no_run
#![doc = include_str!("../examples/span_tween/span_tween.rs")]
//! ```
//! 
//! [`Tween`]: tween::Tween
//! [`TweenDyn`]: tween::Tween
//! [`Interpolator`]: interpolate::Interpolator
//! [`Interpolation`]: interpolation::Interpolation
//! [`EaseFunction`]: interpolation::EaseFunction
//! [`TargetComponent`]: tween::TargetComponent
//! [`TargetAsset`]: tween::TargetAsset
//! [`TargetResource`]: tween::TargetResource
//! [`TweenTimeSpan`]: span_tween::TweenTimeSpan
//! [`ComponentTween`]: tween::ComponentTween
//! [`ComponentTweenDyn`]: tween::ComponentTweenDyn

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]

use bevy::{app::PluginGroupBuilder, prelude::*};

mod utils;

pub mod interpolate;
pub mod interpolation;
pub mod tween;
pub mod tween_timer;

#[cfg(feature = "span_tween")]
pub mod span_tween;

/// Commonly used items
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::interpolate::{self, BoxedInterpolator, Interpolator};
    pub use crate::interpolation::EaseFunction;

    pub use crate::tween_timer::{Repeat, RepeatStyle};

    #[cfg(feature = "span_tween")]
    #[allow(deprecated)]
    pub use crate::span_tween::{
        span_tween, ChildSpanTweenBuilderExt, SpanTweenBundle,
        SpanTweenerBundle, SpanTweenerEnded, WorldChildSpanTweenBuilderExt,
    };

    #[cfg(feature = "bevy_asset")]
    pub use crate::tween::AssetDynTween;
    #[cfg(feature = "bevy_asset")]
    pub use crate::tween::AssetTween;

    pub use crate::tween::ComponentDynTween;
    pub use crate::tween::ComponentTween;

    pub use crate::tween::ResourceDynTween;
    pub use crate::tween::ResourceTween;

    pub use crate::BevyTweenRegisterSystems;
    pub use crate::DefaultTweenPlugins;
}

#[allow(deprecated)]
#[cfg(feature = "bevy_asset")]
pub use tween::asset_dyn_tween_system;
#[cfg(feature = "bevy_asset")]
pub use tween::asset_tween_system;
#[cfg(feature = "bevy_asset")]
#[allow(deprecated)]
pub use tween::asset_tween_system_full;

#[allow(deprecated)]
pub use tween::component_dyn_tween_system;
pub use tween::component_tween_system;
#[allow(deprecated)]
pub use tween::component_tween_system_full;

#[allow(deprecated)]
pub use tween::resource_dyn_tween_system;
pub use tween::resource_tween_system;
#[allow(deprecated)]
pub use tween::resource_tween_system_full;

/// Default plugins for using crate.
pub struct DefaultTweenPlugins;
impl PluginGroup for DefaultTweenPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let p = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(TweenCorePlugin)
            .add(interpolate::DefaultInterpolatorsPlugin)
            .add(interpolate::DefaultDynInterpolatorsPlugin)
            .add(interpolation::EaseFunctionPlugin);
        #[cfg(feature = "span_tween")]
        let p = p.add(span_tween::SpanTweenPlugin);
        p
    }
}

/// Configure [`TweenSystemSet`] and register types.
///
/// [`TweenSystemSet`] configuration:
/// - In [`PostUpdate`]:
///   1. [`TickTweener`],
///   2. [`Tweener`],
///   3. [`UpdateTweenEaseValue`],
///   4. [`ApplyTween`],
pub struct TweenCorePlugin;
impl Plugin for TweenCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                TweenSystemSet::TickTweener,
                TweenSystemSet::Tweener,
                TweenSystemSet::UpdateInterpolationValue,
                TweenSystemSet::ApplyTween,
            )
                .chain(),
        )
        .register_type::<tween_timer::TweenTimer>()
        .register_type::<tween_timer::AnimationDirection>()
        .register_type::<tween_timer::Repeat>()
        .register_type::<tween_timer::RepeatStyle>()
        .register_type::<tween::TweenState>()
        .register_type::<tween::TweenerMarker>()
        .register_type::<tween::TweenInterpolationValue>();
    }
}

/// Enum of SystemSet in this crate
/// See [`TweenCorePlugin`] for default system configuration.
#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    /// This set is for systems that responsible for ticking any
    /// tweener such as the [`span_tween::tick_span_tweener_system`]
    /// by this crate
    TickTweener,
    /// This set is for systems that responsible for updating any
    /// tweener such as the [`span_tween::span_tweener_system`]
    /// by this crate
    Tweener,
    /// This set is for systems that responsible for updating any
    /// [`tween::TweenInterpolationValue`] such as
    /// [`interpolation::sample_interpolations_system`] by this crate.
    UpdateInterpolationValue,
    /// This set is for systems that responsible for actually executing any
    /// active tween and setting the value to its respective tweening item such
    /// as these systems by this crate:
    /// - [`tween::component_tween_system_full`]
    /// - [`tween::resource_tween_system_full`]
    /// - [`tween::asset_tween_system_full`]
    ApplyTween,
}

/// Helper trait to add systems by this crate to your app and avoid mistake
/// from forgetting to use the intended schedule and set.
pub trait BevyTweenRegisterSystems {
    /// Register tween systems in schedule [`PostUpdate`] with set
    /// [`TweenSystemSet::ApplyTween`]
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}

impl BevyTweenRegisterSystems for App {
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.add_systems(
            PostUpdate,
            tween_systems.in_set(TweenSystemSet::ApplyTween),
        )
    }
}
