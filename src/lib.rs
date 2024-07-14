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
#![doc = crate_utils::doc_test_boilerplate!()]
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
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
//!         sprite.with(sprite_color(WHITE.into(), RED.into()))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticIn,
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
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
//! let mut sprite_color = sprite.state(WHITE.into()); // We want the intial color to be white
//! sprite_commands.animation().insert(sequence((
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticOut,
//!         // Switch the constructor to the relative variant
//!         sprite_color.with(sprite_color_to(RED.into()))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticIn,
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
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
//! let mut sprite_color = sprite.state(WHITE.into());
//! sprite_commands.animation()
//!     .repeat(Repeat::Infinitely) // Add repeat
//!     .insert(sequence((
//!         tween(
//!             Duration::from_secs(1),
//!             EaseFunction::QuadraticOut,
//!             sprite_color.with(sprite_color_to(RED.into()))
//!         ),
//!         tween(
//!             Duration::from_secs(1),
//!             EaseFunction::QuadraticIn,
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
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
//!             target_sprite_color.with(sprite_color_to(RED.into()))
//!         ),
//!         tween(
//!             duration,
//!             EaseFunction::QuadraticIn,
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
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
//!             sprite.with(sprite_color(WHITE.into(), RED.into()))
//!         ),
//!         tween(
//!             Duration::from_secs(1),
//!             EaseFunction::QuadraticIn,
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
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
//!         sprite.with(sprite_color(WHITE.into(), RED.into()))
//!     ),
//!     tween(
//!         Duration::from_secs(1),
//!         EaseFunction::QuadraticIn,
//!         sprite.with(sprite_color(RED.into(), WHITE.into()))
//!     ),
//! )));
//! ```
//!
//! ## [`AnimationTarget`]
//! [`AnimationTarget`] can be used for automatic target searching.
//! ```no_run
#![doc = crate_utils::doc_test_boilerplate!()]
//! # use bevy::color::palettes::css::{WHITE, RED};
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
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]

use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use bevy::{app::PluginGroupBuilder, prelude::*};

mod crate_utils;

#[cfg(feature = "bevy_lookup_curve")]
pub use bevy_lookup_curve;
pub use bevy_time_runner;

pub mod curve;
// pub mod interpolate;
pub mod items;
pub mod set;
pub mod targets;
pub mod tween_event;
pub mod utils;

pub mod builder;

/// Commonly used items
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::curve::EaseFunction;
    // pub use crate::interpolate::{self, BoxedInterpolator, Interpolator};

    pub use crate::bevy_time_runner::{Repeat, RepeatStyle, TimeDirection};

    pub use crate::builder::{AnimationBuilderExt, TargetSetExt as _};

    pub use crate::targets::IntoTarget;
    pub use crate::tween_event::{TweenEvent, TweenEventData};

    // #[cfg(feature = "bevy_asset")]
    // pub use crate::tween::AssetDynTween;
    // #[cfg(feature = "bevy_asset")]
    // pub use crate::tween::AssetTween;

    // pub use crate::tween::ComponentDynTween;
    // pub use crate::tween::ComponentTween;

    // pub use crate::tween::ResourceDynTween;
    // pub use crate::tween::ResourceTween;

    // pub use crate::BevyTweenRegisterSystems;
    pub use crate::DefaultTweenPlugins;
}

// #[cfg(feature = "bevy_asset")]
// pub use tween::asset_dyn_tween_system;
// #[cfg(feature = "bevy_asset")]
// pub use tween::asset_tween_system;
// #[cfg(feature = "bevy_asset")]
// pub use tween::component_dyn_tween_system;
// pub use tween::component_tween_system;

// pub use tween::resource_dyn_tween_system;
// pub use tween::resource_tween_system;

// pub use tween_event::tween_event_system;
// pub use tween_event::tween_event_taking_system;

/// Default plugins for using crate.
///
/// Plugins:
/// - [`TweenCorePlugin`]
/// - [`interpolate::DefaultInterpolatorsPlugin`]
/// - [`interpolate::DefaultDynInterpolatorsPlugin`]
/// - [`interpolation::EaseFunctionPlugin`]
/// - [`tween_event::DefaultTweenEventPlugins`]
pub struct DefaultTweenPlugins;

impl PluginGroup for DefaultTweenPlugins {
    #[allow(clippy::let_and_return)]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(TweenCorePlugin::default())
            .add(register_types);

        let group = group
            .add(set::component::<items::Translation>())
            .add(set::component::<items::Rotation>())
            .add(set::component::<items::Scale>())
            .add(set::component::<items::AngleZ>());

        #[cfg(feature = "bevy_sprite")]
        let group = group.add(set::component::<items::SpriteColor>());

        #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
        let group = group
            .add(set::asset::<items::ColorMaterial>())
            .add(set::handle_component::<items::ColorMaterial>());

        #[cfg(feature = "bevy_ui")]
        let group = group
            .add(set::component::<items::BackgroundColor>())
            .add(set::component::<items::BorderColor>());

        let group = group
            .add(curve::EaseFunctionAToBPlugin::new(
                |a: &f32, b: &f32, v: f32| a.lerp(*b, v),
            ))
            .add(curve::EaseFunctionAToBPlugin::new(
                |a: &Vec2, b: &Vec2, v: f32| a.lerp(*b, v),
            ))
            .add(curve::EaseFunctionAToBPlugin::new(
                |a: &Vec3, b: &Vec3, v: f32| a.lerp(*b, v),
            ))
            .add(curve::EaseFunctionAToBPlugin::new(
                |a: &Quat, b: &Quat, v: f32| a.slerp(*b, v),
            ))
            .add(curve::EaseFunctionAToBPlugin::new(
                |a: &Color, b: &Color, v: f32| a.mix(b, v),
            ));
        // #[cfg(feature = "bevy_lookup_curve")]
        // let group = group
        //     .add(curve::bevy_lookup_curve::BevyLookupCurveInterpolationPlugin);
        #[cfg(not(feature = "bevy_eventlistener"))]
        let group = group
            .add(tween_event::TweenEventPlugin::<()>::default())
            .add(tween_event::TweenEventPlugin::<&'static str>::default());
        #[cfg(feature = "bevy_eventlistener")]
        let group = group
            .add(
                tween_event::TweenEventPlugin::<()>::default()
                    .with_event_listener(),
            )
            .add(
                tween_event::TweenEventPlugin::<&'static str>::default()
                    .with_event_listener(),
            );
        group
    }
}

fn register_types(a: &mut App) {
    a.register_type::<items::Translation>()
        .register_type::<items::Rotation>()
        .register_type::<items::Scale>()
        .register_type::<items::AngleZ>();
    #[cfg(feature = "bevy_sprite")]
    a.register_type::<items::SpriteColor>();
    #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
    a.register_type::<items::ColorMaterial>();
    #[cfg(feature = "bevy_ui")]
    a.register_type::<items::BackgroundColor>()
        .register_type::<items::BorderColor>();

    a.register_type::<curve::AToB<f32, curve::EaseFunction>>()
        .register_type::<curve::AToB<Vec2, curve::EaseFunction>>()
        .register_type::<curve::AToB<Vec3, curve::EaseFunction>>()
        .register_type::<curve::AToB<Vec4, curve::EaseFunction>>()
        .register_type::<curve::AToB<Quat, curve::EaseFunction>>()
        .register_type::<curve::AToB<Color, curve::EaseFunction>>();

    a.register_type::<tween_event::TweenEventData>()
        .register_type::<tween_event::TweenEventData<&'static str>>();

    a.register_type::<targets::TargetComponent>()
        .register_type::<targets::TargetResource>()
        .register_type::<targets::TargetAsset<ColorMaterial>>();
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
                TweenSystemSet::UpdateSetterValue,
                TweenSystemSet::ResolveTarget,
                TweenSystemSet::Apply,
            )
                .chain()
                .after(bevy_time_runner::TimeRunnerSet::Progress),
        )
        .insert_resource(self.app_resource.clone());
    }

    fn cleanup(&self, app: &mut App) {
        app.world_mut().remove_resource::<TweenAppResource>();
    }
}

/// Enum of SystemSet in this crate.
/// See [`TweenCorePlugin`] for default system configuration.
#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    /// This set is for systems that responsible for updating any
    /// [`tween::CurveValue`] such as
    /// [`interpolation::sample_interpolations_system`].
    UpdateSetterValue,
    ResolveTarget,
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
    Apply,
}

/// Skip a tween from tweening.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipTween;
