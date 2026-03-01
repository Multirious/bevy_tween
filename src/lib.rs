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
//! let mut sprite_commands = commands.spawn(
//!     Sprite {
//!         custom_size: Some(Vec2::new(50., 50.)),
//!         ..Default::default()
//!     }
//! );
//! ```
//!
//! Now we want to animate it, maybe with one simple tween animation.
//!
//! ### `insert_tween_here` method
//! We can do this:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
//! use bevy_tween::{interpolate::sprite_color, combinator::tween};
//!
//! let sprite = sprite_commands.id().into_target();
//! sprite_commands.animation().insert_tween_here(
//!     Duration::from_secs(1),
//!     EaseKind::QuadraticOut,
//!     sprite.with(sprite_color(WHITE.into(), RED.into()))
//!     // Since this argument accepts a bundle, you can add additional tween to this like so:
//!     // (
//!     //     sprite.with(sprite_color(WHITE.into(), RED.into())),
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
//! # use bevy::color::palettes::css::{WHITE, RED};
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
//! use bevy_tween::{
//!     interpolate::sprite_color,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! sprite_commands.animation().insert(sequence((
//!     tween(
//!         Duration::from_secs(1),
//!         EaseKind::QuadraticOut,
//!         sprite.with(sprite_color(WHITE.into(), RED.into()))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseKind::QuadraticIn,
//!         sprite.with(sprite_color(RED.into(), WHITE.into()))
//!     ),
//! )));
//! ```
//! This adds one [`TimeRunner`] to your entity and 2 [`Tween`] entities as the child.
//!
//! ### State
//!
//! There's a little bit of boilerplate we can improve.
//! Currently we've specified `RED.into()` 2 times because we want our tween to
//! continue from previous value.
//!
//! We can use state for this:
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
//! use bevy_tween::{
//!     interpolate::sprite_color_to,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! let mut sprite_color = sprite.state(WHITE.into()); // We want the intial color to be white
//! sprite_commands.animation().insert(sequence((
//!     tween(
//!         Duration::from_secs(1),
//!         EaseKind::QuadraticOut,
//!         // Switch the constructor to the relative variant
//!         sprite_color.with(sprite_color_to(RED.into()))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseKind::QuadraticIn,
//!         sprite_color.with(sprite_color_to(WHITE.into()))
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
//! # use bevy::color::palettes::css::{WHITE, RED};
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
//! use bevy_tween::{
//!     interpolate::sprite_color_to,
//!     combinator::{tween, sequence}
//! };
//!
//! let sprite = sprite_commands.id().into_target();
//! let mut sprite_color = sprite.state(WHITE.into());
//! sprite_commands.animation()
//!     .repeat(Repeat::Infinitely) // Add repeat
//!     .insert(sequence((
//!         tween(
//!             Duration::from_secs(1),
//!             EaseKind::QuadraticOut,
//!             sprite_color.with(sprite_color_to(RED.into()))
//!         ),
//!         tween(
//!             Duration::from_secs(1),
//!             EaseKind::QuadraticIn,
//!             sprite_color.with(sprite_color_to(WHITE.into()))
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
//! # use bevy::color::palettes::css::{WHITE, RED};
#![doc = utils::doc_test_boilerplate!()]
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
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
//!             EaseKind::QuadraticOut,
//!             target_sprite_color.with(sprite_color_to(RED.into()))
//!         ),
//!         tween(
//!             duration,
//!             EaseKind::QuadraticIn,
//!             target_sprite_color.with(sprite_color_to(WHITE.into()))
//!         ),
//!     ))
//! }
//!
//! let sprite = sprite_commands.id().into_target();
//! let mut sprite_color = sprite.state(WHITE.into());
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
//! # use bevy::color::palettes::css::{WHITE, RED};
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
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
//!             EaseKind::QuadraticOut,
//!             sprite.with(sprite_color(WHITE.into(), RED.into()))
//!         ),
//!         tween(
//!             Duration::from_secs(1),
//!             EaseKind::QuadraticIn,
//!             sprite.with(sprite_color(RED.into(), WHITE.into()))
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
//! # use bevy::color::palettes::css::{WHITE, RED};
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
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
//!         EaseKind::QuadraticOut,
//!         sprite.with(sprite_color(WHITE.into(), RED.into()))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseKind::QuadraticIn,
//!         sprite.with(sprite_color(RED.into(), WHITE.into()))
//!     ),
//! )));
//! ```
//!
//! ## [`AnimationTarget`]
//! [`AnimationTarget`] can be used for automatic target searching.
//! ```no_run
#![doc = utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
//! # let mut sprite_commands = commands.spawn(
//! #     Sprite {
//! #         custom_size: Some(Vec2::new(50., 50.)),
//! #         ..Default::default()
//! #     },
//! # );
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
//!     EaseKind::QuadraticOut,
//!     sprite.with(sprite_color(WHITE.into(), RED.into()))
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
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_cfg))]
#![warn(missing_docs)]

use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use bevy::ecs::system::ScheduleSystem;
use bevy::{app::PluginGroupBuilder, prelude::*};
use std::marker::PhantomData;

mod utils;

#[cfg(feature = "bevy_lookup_curve")]
pub use bevy_lookup_curve;
pub use bevy_time_runner;

pub mod interpolate;
pub mod interpolation;
pub mod tween;
pub mod tween_event;

pub mod combinator;

/// Commonly used items
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::interpolate::{self, BoxedInterpolator, Interpolator};
    pub use crate::interpolation::EaseKind;

    pub use crate::bevy_time_runner::{
        Repeat, RepeatStyle, TimeContext, TimeDirection,
    };

    pub use crate::combinator::{AnimationBuilderExt, TransformTargetStateExt};

    pub use crate::tween::IntoTarget;
    pub use crate::tween_event::{TweenEvent, TweenEventData};

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

    pub use crate::DefaultTweenPluginsOnDefaultTime;
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

pub use tween_event::tween_event_system;

/// The default tween plugins on default time context
pub type DefaultTweenPluginsOnDefaultTime = DefaultTweenPlugins<()>;

/// Default plugins for using crate.
pub struct DefaultTweenPlugins<TimeCtx = ()>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    /// Register all systems from this plugin to the specified schedule.
    pub schedule: InternedScheduleLabel,
    /// Enable debug information and warnings.
    ///
    /// This currently is passed to [`bevy_time_runner::TimeRunnerPlugin::enable_debug`] field.
    pub enable_debug: bool,
    /// A marker for the plugins time context
    marker: PhantomData<TimeCtx>,
}

impl<TimeCtx> PluginGroup for DefaultTweenPlugins<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut tween_core_plugin =
            TweenCorePlugin::<TimeCtx>::in_schedule(self.schedule);
        tween_core_plugin.enable_debug = self.enable_debug;
        let group = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(tween_core_plugin)
            .add_group(tween_event::DefaultTweenEventPlugins::<TimeCtx>::in_schedule(
                self.schedule,
            ))
            .add(
                interpolation::EaseKindPlugin::<TimeCtx>::in_schedule(
                    self.schedule,
                )
            )
            .add(
                interpolate::DefaultInterpolatorsPlugin::<TimeCtx>::in_schedule(
                    self.schedule,
                )
            )
            .add(
                interpolate::DefaultDynInterpolatorsPlugin::<TimeCtx>::in_schedule(
                    self.schedule,
                )
            )
            .add(
                SystemSetsRegistraitonPlugin {
                    schedule: self.schedule,
                }
            );

        #[cfg(feature = "bevy_lookup_curve")]
        let group = group.add(interpolation::bevy_lookup_curve::BevyLookupCurveInterpolationPlugin::<TimeCtx>::in_schedule(self.schedule));

        group
    }
}

impl<TimeCtx> DefaultTweenPlugins<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    /// Register all systems from this plugin to the specified schedule.
    pub fn in_schedule(schedule: InternedScheduleLabel) -> Self {
        Self {
            marker: PhantomData,
            schedule,
            enable_debug: true,
        }
    }
}

impl Default for DefaultTweenPlugins<()> {
    fn default() -> Self {
        Self {
            schedule: PostUpdate.intern(),
            enable_debug: true,
            marker: PhantomData,
        }
    }
}

/// This resource will be used while initializing tween plugin and systems.
/// [`BevyTweenRegisterSystems`] for example.
#[derive(Resource, Clone)]
#[deprecated(
    // TODO: since = "...",
    note = "This resource became less practical after generic_time_context (#78) PR"
)]
#[doc(hidden)]
pub struct TweenAppResource {
    /// Configured schedule for tween systems.
    pub schedule: InternedScheduleLabel,
}

#[allow(deprecated)]
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
pub struct TweenCorePlugin<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    /// The schedule to register the core time-runner plugin in
    schedule: InternedScheduleLabel,
    /// Enable debug information and warnings.
    ///
    /// This currently is passed to [`bevy_time_runner::TimeRunnerPlugin::enable_debug`] field.
    pub enable_debug: bool,
    /// A marker for the plugins time context
    marker: PhantomData<TimeCtx>,
}

impl<TimeCtx> Plugin for TweenCorePlugin<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_time_runner::TimeRunnerPlugin>() {
            let mut time_runner_plugin =
                bevy_time_runner::TimeRunnerPlugin::<TimeCtx>::in_schedule(
                    self.schedule,
                );
            time_runner_plugin.enable_debug = self.enable_debug;
            app.add_plugins(time_runner_plugin);
        }

        app.register_type::<tween::AnimationTarget>()
            .register_type::<tween::TweenInterpolationValue>();
    }
}

impl<TimeCtx> TweenCorePlugin<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    /// Constructor for schedule
    pub fn in_schedule(schedule: InternedScheduleLabel) -> Self {
        Self {
            marker: PhantomData,
            schedule,
            ..default()
        }
    }
}
impl<TimeCtx> Default for TweenCorePlugin<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            schedule: PostUpdate.intern(),
            enable_debug: true,
            marker: PhantomData,
        }
    }
}

/// A plugin for registering the system sets in specific schedule
struct SystemSetsRegistraitonPlugin {
    /// The schedule to register the sets in
    schedule: InternedScheduleLabel,
}

impl Plugin for SystemSetsRegistraitonPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            self.schedule,
            (
                TweenSystemSet::UpdateInterpolationValue,
                TweenSystemSet::ApplyTween,
            )
                .chain()
                .after(bevy_time_runner::TimeRunnerSet::Progress),
        );
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
    ApplyTween,
}

/// Helper trait to add systems by this crate to your app
/// for different schedules
pub trait BevyTweenRegisterSystems {
    /// Register tween systems
    fn add_tween_systems<M>(
        &mut self,
        schedule: InternedScheduleLabel,
        tween_systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
}

impl BevyTweenRegisterSystems for App {
    /// Register tween systems in schedule
    /// in set [`TweenSystemSet::ApplyTween`]
    fn add_tween_systems<M>(
        &mut self,
        schedule: InternedScheduleLabel,
        tween_systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(
            schedule,
            tween_systems.in_set(TweenSystemSet::ApplyTween),
        )
    }
}
