//! # Getting started
//!
//! [`DefaultTweenPlugins`] provide most the stuff you will need.
//! Add the plugin to your app:
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_tween::prelude::*;
//!
//! fn main() {
//!     App::default()
//!         .add_plugins((DefaultPlugins, DefaultTweenPlugins))
//!         .run();
//! }
//! ```
//!
//! # Usages
//!
//! ## Creating animation
//!
//! Let say we have this sprite:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! let mut sprite_commands = commands.spawn(SpriteBundle {
//!     sprite: Sprite {
//!         custom_size: Some(Vec2::new(50., 50.)),
//!         ..Default::default()
//!     },
//!     ..Default::default()
//! });
//! ```
//!
//! Now we want to animate it, maybe with one simple tween animation.
//!
//! ### `insert_tween_here` method
//! We can do this:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{interpolate::sprite_color, combinator::tween};
//!
//! let sprite = sprite_commands.id().into_target();
//! sprite_commands.animation().insert_tween_here(
//!     Duration::from_secs(1),
//!     EaseFunction::QuadraticOut,
//!     sprite.with(sprite_color(Color::WHITE, Color::RED))
//!     // Since this argument accepts a bundle, you can add additional tween to this like so:
//!     // (
//!     //     sprite.with(sprite_color(Color::WHITE, Color::RED)),
//!     //     sprite.with(something_something(...)),
//!     // )
//! );
//! ```
//! This insert every animation components directly into your entity.
//! Use this if you wish to keep your entity
//! structure simple (no children) and doesn't need complicated animation.
//!
//! ### Combinator framework
//!
//! Now what if we want to chain animations?
//!
//! This crate provide a way to create animation using combinator framework.
//! So we can just:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{
//!     interpolate::sprite_color,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! sprite_commands.animation().insert(sequence((
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticOut,
//!         sprite.with(sprite_color(Color::WHITE, Color::RED))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticIn,
//!         sprite.with(sprite_color(Color::RED, Color::WHITE))
//!     ),
//! )));
//! ```
//! This adds one [`TimeRunner`] to your entity and 2 [`Tween`] entities as the child.
//!
//! ### State
//!
//! There's a little bit of boilerplate we can improve.
//! Currently we've specified `Color::RED` 2 times because we want our tween to
//! continue from previous value.
//!
//! We can use state for this:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{
//!     interpolate::sprite_color_to,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! let mut sprite_color = sprite.state(Color::WHITE); // We want the intial color to be white
//! sprite_commands.animation().insert(sequence((
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticOut,
//!         // Switch the constructor to the relative variant
//!         sprite_color.with(sprite_color_to(Color::RED))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticIn,
//!         sprite_color.with(sprite_color_to(Color::WHITE))
//!     ),
//! )));
//! ```
//! Looks good!
//!
//! ### Repeating
//!
//! If we want to repeat our animation to so we can do:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{
//!     interpolate::sprite_color_to,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! let mut sprite_color = sprite.state(Color::WHITE);
//! sprite_commands.animation()
//!     .repeat(Repeat::Infinitely) // Add repeat
//!     .insert(sequence((
//!         tween(
//!             Duration::from_secs(1),
//!             EaseFunction::QuadraticOut,
//!             sprite_color.with(sprite_color_to(Color::RED))
//!         ),
//!         tween(
//!             Duration::from_secs(1),
//!             EaseFunction::QuadraticIn,
//!             sprite_color.with(sprite_color_to(Color::WHITE))
//!         ),
//!     )));
//! ```
//!
//! ### Custom combinator
//!
//! What if you want to abstract animation?
//! - To manage large animation code
//! - To reuse animation code
//! - Custom combinators
//!
//! Combinator framework got you covered!:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{
//!     interpolate::sprite_color_to,
//!     combinator::{AnimationCommands, TargetState, tween, sequence},
//!     tween::TargetComponent,
//! };
//!
//! // Create new combinator
//! fn my_animation(
//!     // You can use `TargetComponent` if you doesn't use state.
//!     target_sprite_color: &mut TargetState<TargetComponent, Color>,
//!     duration: Duration
//! ) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
//!     sequence((
//!         tween(
//!             duration,
//!             EaseFunction::QuadraticOut,
//!             target_sprite_color.with(sprite_color_to(Color::RED))
//!         ),
//!         tween(
//!             duration,
//!             EaseFunction::QuadraticIn,
//!             target_sprite_color.with(sprite_color_to(Color::WHITE))
//!         ),
//!     ))
//! }
//!
//! let sprite = sprite_commands.id().into_target();
//! let mut sprite_color = sprite.state(Color::WHITE);
//! sprite_commands.animation()
//!     .repeat(Repeat::Infinitely)
//!     .insert(my_animation(&mut sprite_color, Duration::from_secs(1)));
//! ```
//! There are more combinators you can use. Check them out in the [`combinator`] module.
//!
//! ## Animator as a child
//! You can spawn animator as a child if you want
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{
//!     interpolate::sprite_color,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! sprite_commands.with_children(|c| {
//!     c.animation().insert(sequence((
//!         tween(
//!             Duration::from_secs(1),
//!             EaseFunction::QuadraticOut,
//!             sprite.with(sprite_color(Color::WHITE, Color::RED))
//!         ),
//!         tween(
//!             Duration::from_secs(1),
//!             EaseFunction::QuadraticIn,
//!             sprite.with(sprite_color(Color::RED, Color::WHITE))
//!         ),
//!     )));
//! });
//! ```
//!
//! ## Orphaned Animator
//! An animator does not required to be a child or inside your target entity.
//! You can spawn them anywhere in the world if needed.
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{
//!     interpolate::sprite_color,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! // use `.animation()` on commands directly to spawn a new entity
//! commands.animation().insert(sequence((
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticOut,
//!         sprite.with(sprite_color(Color::WHITE, Color::RED))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticIn,
//!         sprite.with(sprite_color(Color::RED, Color::WHITE))
//!     ),
//! )));
//! ```
//!
//! ## [`AnimationTarget`]
//! [`AnimationTarget`] can be used for automatic target searching.
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(SpriteBundle {
//! #     sprite: Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! #     ..Default::default()
//! # });
//! use bevy_tween::{
//!     interpolate::sprite_color,
//!     combinator::tween,
//!     tween::AnimationTarget,
//! };
//!
//! let sprite = AnimationTarget.into_target(); // which returns TargetComponent::Marker
//! sprite_commands.insert(AnimationTarget);
//! sprite_commands.animation().insert_tween_here(
//!     Duration::from_secs(1),
//!     EaseFunction::QuadraticOut,
//!     sprite.with(sprite_color(Color::WHITE, Color::RED))
//! );
//! ```
//!
//! ## Custom interpolator
//!
//! See these documentations for more details:
//! - ["Your own interpolator"](crate::interpolate#your-own-interpolator).
//! - ["Registering systems"](crate::tween#registering-systems).
//!
//! This example shows how to create your own inteprolator.
//!
//! <details>
//! <summary>
//!
//! `interpolator` example
//!
//! </summary>
//!
//! ```no_run
#![doc = include_str!("../examples/interpolator.rs")]
//! ```
//!
//! </details>
//!
//! ## Entity structure
//!
//! This example shows what's actually going on under the hood within this crate's API.
//!
//! <details>
//! <summary>
//!
//! `entity_structure` example
//!
//! </summary>
//!
//! Run `cargo run --example entity_structure` to see this in action.
//! ```no_run
#![doc = include_str!("../examples/entity_structure.rs")]
//! ```
//!
//! </details>
//!
//! [`TimeRunner`]: bevy_time_runner::TimeRunner
//! [`Tween`]: tween::Tween
//! [`AnimationTarget`]: tween::AnimationTarget
#![allow(clippy::needless_doctest_main)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]

use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use bevy::{app::PluginGroupBuilder, prelude::*};

mod utils;

#[cfg(feature = "bevy_lookup_curve")]
pub use bevy_lookup_curve;
pub use bevy_time_runner;

pub mod interpolate;
pub mod interpolation;
pub mod tween;

pub mod combinator;

/// Commonly used items
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::interpolate::{self, BoxedInterpolator, Interpolator};
    pub use crate::interpolation::EaseFunction;

    pub use crate::bevy_time_runner::{Repeat, RepeatStyle, TimeDirection};

    pub use crate::combinator::{AnimationBuilderExt, TransformTargetStateExt};

    pub use crate::tween::{IntoTarget, TweenEvent, TweenEventData};

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

#[cfg(feature = "bevy_asset")]
pub use tween::asset_dyn_tween_system;
#[cfg(feature = "bevy_asset")]
pub use tween::asset_tween_system;
#[cfg(feature = "bevy_asset")]
pub use tween::component_dyn_tween_system;
pub use tween::component_tween_system;

pub use tween::resource_dyn_tween_system;
pub use tween::resource_tween_system;

pub use tween::tween_event_system;
pub use tween::tween_event_taking_system;

/// Default plugins for using crate.
///
/// Plugins:
/// - [`TweenCorePlugin`]
/// - [`interpolate::DefaultInterpolatorsPlugin`]
/// - [`interpolate::DefaultDynInterpolatorsPlugin`]
/// - [`interpolation::EaseFunctionPlugin`]
pub struct DefaultTweenPlugins;

impl PluginGroup for DefaultTweenPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        #[allow(clippy::let_and_return)]
        let group = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(TweenCorePlugin::default())
            .add(interpolate::DefaultInterpolatorsPlugin)
            .add(interpolate::DefaultDynInterpolatorsPlugin)
            .add(interpolation::EaseFunctionPlugin)
            .add(tween::DefaultTweenEventsPlugin);
        #[cfg(feature = "bevy_lookup_curve")]
        let group = group.add(interpolation::bevy_lookup_curve::BevyLookupCurveInterpolationPlugin);
        group
    }
}

/// This resource will be used while initializing tween plugin and systems.
/// [`BevyTweenRegisterSystems`] for example.
#[derive(Resource, Clone)]
pub struct TweenAppResource {
    /// Configured schedule for tween systems.
    pub schedule: InternedScheduleLabel,
}

impl Default for TweenAppResource {
    fn default() -> Self {
        TweenAppResource {
            schedule: PostUpdate.intern(),
        }
    }
}

/// Configure [`TweenSystemSet`] and register types.
///
/// [`TweenSystemSet`] configuration:
/// - In schedule configured by [`TweenAppResource`]:
///   1. [`UpdateInterpolationValue`],
///   2. [`ApplyTween`],
///
///   [`UpdateInterpolationValue`]: [`TweenSystemSet::UpdateInterpolationValue`]
///   [`ApplyTween`]: [`TweenSystemSet::ApplyTween`]
#[derive(Default)]
pub struct TweenCorePlugin {
    /// See [`TweenAppResource`]
    pub app_resource: TweenAppResource,
}

impl Plugin for TweenCorePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_time_runner::TimeRunnerPlugin>() {
            app.add_plugins(bevy_time_runner::TimeRunnerPlugin {
                schedule: self.app_resource.schedule,
            });
        }
        app.configure_sets(
            self.app_resource.schedule,
            (
                TweenSystemSet::UpdateInterpolationValue,
                TweenSystemSet::ApplyTween,
            )
                .chain()
                .after(bevy_time_runner::TimeRunnerSet::Progress),
        )
        .insert_resource(self.app_resource.clone())
        .register_type::<tween::AnimationTarget>()
        .register_type::<tween::TweenInterpolationValue>();
    }

    fn cleanup(&self, app: &mut App) {
        app.world.remove_resource::<TweenAppResource>();
    }
}

/// Enum of SystemSet in this crate.
/// See [`TweenCorePlugin`] for default system configuration.
#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    /// This set is for systems that responsible for updating any
    /// [`tween::TweenInterpolationValue`] such as
    /// [`interpolation::sample_interpolations_system`].
    UpdateInterpolationValue,
    /// This set is for systems that responsible for actually executing any
    /// active tween and setting the value to its respective tweening item such
    /// as these systems:
    /// - [`tween::component_tween_system`]
    /// - [`tween::resource_tween_system`]
    /// - [`tween::asset_tween_system`]
    ///
    /// Events is not necessary related to tweening but their code is still working in the same area.
    /// - [`tween::tween_event_system`]
    /// - [`tween::tween_event_taking_system`]
    ApplyTween,
}

/// Helper trait to add systems by this crate to your app and avoid mistake
/// from forgetting to use the intended schedule and set.
pub trait BevyTweenRegisterSystems {
    /// Register tween systems
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}

impl BevyTweenRegisterSystems for App {
    /// Register tween systems in schedule configured in [`TweenAppResource`]
    /// in set [`TweenSystemSet::ApplyTween`]
    ///
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    fn add_tween_systems<M>(
        &mut self,
        tween_systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let app_resource = self
            .world
            .get_resource::<TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        self.add_systems(
            app_resource.schedule,
            tween_systems.in_set(TweenSystemSet::ApplyTween),
        )
    }
}
