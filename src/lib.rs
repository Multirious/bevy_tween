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
//!         .add_plugins((
//!             DefaultPlugins,
//!             DefaultTweenPlugins,
//!         ))
//!         .run();
//! }
//! ```
//!
//! ## Tween and Tween player
//!
//! Tween player handles the current actual playback timing of any tweens that
//! it's responsible for.
//!
//! Tween is your animation parameters that declares:
//! - "**What**" to interpolate, such as [`TargetComponent`], [`TargetAsset`], and
//!   [`TargetResource`]. These implemented the [`TweenTarget`] trait.
//! - "**How**" to interpolate, such as [`interpolate::Translation`] and
//!   [`interpolate::SpriteColor`]. These implemented the [`Interpolator`]
//!   trait.
//! - "**When**" to interpolate such as [`TweenTimeSpan`].
//!
//! ## Entity structure
//!
//! If we have this entity:
//!   ```
//!   # use bevy::prelude::*;
//!   # use bevy_tween::prelude::*;
//!   # let world = World::new();
//!   # let mut commands_queue = bevy::ecs::system::CommandQueue::default();
//!   # let mut commands = Commands::new(&mut commands_queue, &world);
//!   let my_entity = commands.spawn(SpriteBundle::default()).id();
//!   ```
//!  
//!   We can create a tween player with tween in 2 ways:
//! - Tween in the same entity as a tween player.<br/>
//!   This is the case where you might want to make a simple animation where
//!   there's not many parameteres. Because an entity can only have one unique
//!   component, it limits on what animation you can achieve with this.
//!   ```
//!   # use bevy::prelude::*;
//!   # use bevy_tween::prelude::*;
//!   # let world = World::new();
//!   # let mut commands_queue = bevy::ecs::system::CommandQueue::default();
//!   # let mut commands = Commands::new(&mut commands_queue, &world);
//!   # let my_entity = commands.spawn(SpriteBundle::default()).id();
//!   // Spawning some tween
//!   commands.spawn((
//!       // The tween player:
//!       SpanTweenPlayerBundle::new(Duration::from_secs(1)),
//!       // The tween:
//!       // Tween this from the start to the second 1.
//!       SpanTweenBundle::new(..Duration::from_secs(1)),
//!       // Tween this with ease quadratic out.
//!       EaseFunction::QuadraticOut,
//!       // Tween a component.
//!       ComponentTween::new_target(
//!           // Tween the component of this entity
//!           my_entity,
//!           // Tween transform's translation of the entity
//!           interpolate::Translation {
//!               start: Vec3::new(0., 0., 0.),
//!               end: Vec3::new(0., 100., 0.),
//!           }
//!       )
//!   ));
//!   ```
//! - Tween(s) as a child of a tween player.<br/>
//!   This is the case where you want to make a more complex animation. By having
//!   tweens as tween player's children, you can have any number of tweens and types
//!   you wanted .
//!   ```
//!   # use bevy::prelude::*;
//!   # use bevy_tween::prelude::*;
//!   # let world = World::new();
//!   # let mut commands_queue = bevy::ecs::system::CommandQueue::default();
//!   # let mut commands = Commands::new(&mut commands_queue, &world);
//!   # let my_entity = commands.spawn(SpriteBundle::default()).id();
//!   // Spawning some tween
//!   commands.spawn(
//!       // The tween player:
//!       SpanTweenPlayerBundle::new(Duration::from_secs(1)),
//!   ).with_children(|c| {
//!       // The tween:
//!       c.spawn((
//!           SpanTweenBundle::new(..Duration::from_secs(1)),
//!           EaseFunction::QuadraticOut,
//!           ComponentTween::new_target(
//!               my_entity,
//!               interpolate::Translation {
//!                   start: Vec3::new(0., 0., 0.),
//!                   end: Vec3::new(0., 100., 0.),
//!               }
//!           )
//!       ));
//!      // spawn some more tween if needed.
//!      // c.spawn( ... );
//!   });
//!   ```
//! - Also the above 2 combined will works just fine btw.
//!
//! ## Examples
//!
//! This example show the basic way you would do to create some entity with
//! tweening on them.
//!
//! Run `cargo run --example simple_tween` to see this in action.
//! ```no_run
#![doc = include_str!("../examples/simple_tween.rs")]
//! ```
//!
//! [`Tween`]: tween::Tween
//! [`TweenDyn`]: tween::Tween
//! [`Interpolator`]: interpolate::Interpolator
//! [`TargetComponent`]: tween::TargetComponent
//! [`TargetAsset`]: tween::TargetAsset
//! [`TargetResource`]: tween::TargetResource
//! [`TweenTimeSpan`]: span_tween::TweenTimeSpan
//! [`TweenTarget`]: tween::TweenTarget

// #![warn(missing_docs)]

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

    pub use crate::interpolate::{self, Interpolator};
    pub use crate::interpolation::EaseFunction;
    pub use crate::tween_timer::{Repeat, RepeatStyle, TweenTimerEnded};
    pub use crate::DefaultTweenPlugins;

    #[cfg(all(feature = "bevy_asset", feature = "tween_static"))]
    pub use crate::tween::AssetTween;
    #[cfg(feature = "tween_static")]
    pub use crate::tween::ComponentTween;
    #[cfg(feature = "tween_static")]
    pub use crate::tween::ResourceTween;

    #[cfg(all(feature = "tween_dyn", feature = "bevy_asset"))]
    pub use crate::tween::AssetTweenDyn;
    #[cfg(feature = "tween_dyn")]
    pub use crate::tween::ComponentTweenDyn;
    #[cfg(feature = "tween_dyn")]
    pub use crate::tween::ResourceTweenDyn;

    #[cfg(feature = "span_tween")]
    pub use crate::span_tween::{
        BuildSpanTweens, SpanTweenBundle, SpanTweenPlayerBundle,
    };
}

pub use tween::asset_tween_dyn_system;
pub use tween::asset_tween_system;
pub use tween::component_tween_dyn_system;
pub use tween::component_tween_system;
pub use tween::resource_tween_dyn_system;
pub use tween::resource_tween_system;

/// Default plugins for using crate.
pub struct DefaultTweenPlugins;
impl PluginGroup for DefaultTweenPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let p = PluginGroupBuilder::start::<DefaultTweenPlugins>()
            .add(TweenCorePlugin)
            .add(interpolate::DefaultInterpolatorsPlugin)
            .add(interpolation::EaseFunctionPlugin);
        #[cfg(feature = "span_tween")]
        let p = p.add(span_tween::SpanTweenPlugin);
        p
    }
}

/// Core, necessary types, and configurations you need to get started with
/// this plugin
pub struct TweenCorePlugin;
impl Plugin for TweenCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                TweenSystemSet::TickTweenTimer,
                TweenSystemSet::TweenPlayer,
                TweenSystemSet::UpdateInterpolationValue,
                TweenSystemSet::ApplyTween,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            (tween_timer::tick_tween_timer_system,)
                .in_set(TweenSystemSet::TickTweenTimer),
        )
        .add_event::<tween_timer::TweenTimerEnded>()
        .register_type::<tween_timer::TweenTimer>()
        .register_type::<tween_timer::AnimationDirection>()
        .register_type::<tween_timer::Repeat>()
        .register_type::<tween_timer::RepeatStyle>()
        .register_type::<tween::TweenState>()
        .register_type::<tween::TweenPlayerMarker>()
        .register_type::<tween::TweenInterpolationValue>();
    }
}

/// Enum of SystemSet in this crate
/// After adding the plugin [`TweenCorePlugin`], these set will be configured
/// to run in the [`PostUpdate`] schedule so any modification you've done before
/// this schedule should be correctly applied in the next frame.
///
/// The sets should be configured to run in this order:
///  1. TickTweenTimer
///  2. TweenPlayer
///  3. UpdateTweenEaseValue
///  4. ApplyTween
#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenSystemSet {
    /// This set is for systems that responsible for updating [`TweenTimer`]'s
    /// elasped.
    ///
    /// [`TweenTimer`]: tween_timer::TweenTimer
    TickTweenTimer,
    /// This set is for systems that responsible for updating any specific
    /// tween player implementation such as the [`span_tween::span_tween_player_system`]
    /// by this crate
    TweenPlayer,
    /// This set is for systems that responsible for updating any
    /// [`tween::TweenInterpolationValue`] such as
    /// [`interpolation::sample_interpolations_system`] by this crate.
    UpdateInterpolationValue,
    /// This set is for systems that responsible for actually executing any
    /// active tween and setting the value to its respective tweening item such
    /// as these systems by this crate:
    /// - [`tween::component_tween_system`]
    /// - [`tween::component_tween_dyn_system`]
    /// - [`tween::resource_tween_system`]
    /// - [`tween::resource_tween_dyn_system`]
    /// - [`tween::asset_tween_system`]
    /// - [`tween::asset_tween_dyn_system`]
    ApplyTween,
}
