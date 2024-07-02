//! Systems and plugins for tweening
//!
//! # Registering custom tween system
//! Check out [`DefaultTweenSystemPlugins`].
//!
//! When you have any components/assets/resources you want to tween,
//! you create a type implementing [`Setter`] and register them to the related plugin:
//! - [`ComponentTweenPlugin`] ([`component`])
//! - [`ResourceTweenPlugin`] ([`resource`])
//! - [`AssetTweenPlugin`] ([`asset`])
//! - [`HandleComponentTweenPlugin`] ([`handle_component`])
//!
//! If these aren't enough for your uses, you can always create a custom system.
//!
//! # How it works
//!
//! In this crate, tweening is a behavior resulting from orchestrating specific systems
//! together in a pipeline to create a flexible and extendable animation system.
//!
//! A single entity should have the components "Timing", "Curve", "Target", and "Setter" for input.
//! Then the systems will take these components and execute the tweening.
//! This crate will usually call this entity a "Tween entity".
//!
//! The system pipeline:
//! 1. Timing. Use [`TimeSpan`] to specify at which point the tween entity starts and stops tweening.
//!    [`bevy_time_runner`]'s systems will provide [`TimeSpanProgress`] component to signal the progress.
//! 2. Curve. Any curve systems then take [`TimeSpanProgress`] to interpolate their curve in the entity
//!    then provide [`CurveValue`] component.
//! 3. Tween. Any tween systems will then take [`CurveValue`] and use them with "Target" and "Setter"
//!    inside the entity to does the tweening.
//!
//! # Bevy items
//!
//! **Plugins**:
//! - [`DefaultTweenSystemPlugins`]
//! - [`ComponentTweenPlugin`] ([`component`])
//! - [`ResourceTweenPlugin`] ([`resource`])
//! - [`AssetTweenPlugin`] ([`asset`])
//! - [`HandleComponentTweenPlugin`] ([`handle_component`])
//!
//! **Components**:
//! - [`SkipTween`]
//!
//! **Systems**:
//! - [`component_tween_system`]
//! - [`resource_tween_system`]
//! - [`asset_tween_system`]
//! - [`handle_component_tween_system`]
//!
//! [`Setter`]: crate::items::Setter
//! [`TimeSpan`]: bevy_time_runner::TimeSpan
//! [`TimeSpanProgress`]: bevy_time_runner::TimeSpanProgress
//! [`CurveValue`]: crate::CurveValue

use bevy::prelude::*;

mod plugin;
mod system;

pub use plugin::*;
pub use system::*;

/// Skip a tween from tweening.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipTween;
