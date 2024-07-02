//! Module containing implementations for tween
//!
//! # Tween
//!
//! **Components**:
//! - [`Tween<T, I>`]
//! - [`SkipTween`]
//!
//! **Systems**
//! - [`component_tween_system`]
//! - [`resource_tween_system`]
//! - [`asset_tween_system`]
//!
//! **Targets**:
//! - [`TargetComponent`]
//! - [`TargetResource`]
//! - [`TargetAsset`]
//!
//! See available interpolators in [`interpolate`].
//!
//! ## Registering systems
//!
//! In order for your custom interpolators to work. You have to register systems
//! to actually have something happening.
//! The [`DefaultTweenPlugins`] will already register some systems for you already to get started.
//! Check [`DefaultInterpolatorsPlugin`] or [`DefaultDynInterpolatorsPlugin`].
//!
//! This crate contains generic systems tweening components, assets,
//! and resources, allowing you to quickly register your custom interpolators.
//!
//! Systems:
//! - [`component_tween_system()`], component tweening system
//! - [`resource_tween_system()`], resource tweening system
//! - [`asset_tween_system()`], asset tweening system
//!
//! Let's say you have some custom components with multiple interpolators.
//!
//! ```no_run
//! # mod a { // had to put this module here for some reason. tf?
//! use bevy::prelude::*;
//! use bevy_tween::prelude::*;
//!
//! #[derive(Component)]
//! pub struct Foo {
//!     a: f32,
//!     b: f32,
//!     c: f32,
//! }
//!
//! mod my_interpolate {
//!     use bevy::prelude::*;
//!     use bevy_tween::prelude::*;
//!
//!     pub struct FooA {
//!         /* ... */
//!     }
//!
//!     impl Interpolator for FooA {
//!         # type Item = super::Foo;
//!         # fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//!         #     todo!()
//!         # }
//!         /* ... */
//!     }
//!
//!     pub struct FooB {
//!         /* ... */
//!     }
//!
//!     impl Interpolator for FooB {
//!         # type Item = super::Foo;
//!         # fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//!         #     todo!()
//!         # }
//!         /* ... */
//!     }
//!
//!     pub struct FooC {
//!         /* ... */
//!     }
//!
//!     impl Interpolator for FooC {
//!         # type Item = super::Foo;
//!         # fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//!         #     todo!()
//!         # }
//!         /* ... */
//!     }
//! }
//! # }
//! ```
//!
//! There's 2 kind of system you might want to register.
//!
//! ### Registering system for generic interpolator
//!
//! Generic interpolator means we're not using any dynamic dispatch.
//! We've to register this system for **every individual interpolator**.
//!
//! ```no_run
//! # mod a { // had to put this module here for some reason. tf?
//! # use bevy::prelude::*;
//! # use bevy_tween::prelude::*;
//! # #[derive(Component)]
//! # pub struct Foo {
//! #     a: f32,
//! #     b: f32,
//! #     c: f32,
//! # }
//! # mod my_interpolate {
//! #     use bevy::prelude::*;
//! #     use bevy_tween::prelude::*;
//! #     pub struct FooA {}
//! #     impl Interpolator for FooA {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooB {}
//! #     impl Interpolator for FooB {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooC {}
//! #     impl Interpolator for FooC {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! # }
//! fn main() {
//!     use bevy_tween::component_tween_system;
//!     use my_interpolate::*;
//!
//!     App::new().add_tween_systems((
//!         component_tween_system::<FooA>(),
//!         component_tween_system::<FooB>(),
//!         component_tween_system::<FooC>(),
//!     ));
//! }
//! # }
//! ```
//!
//! ### Registering system for dynamic interpolator
//!
//! Dynamic interpolator means we're using dynamic dispatch or trait object.
//! We don't have to register system for every interpolator, we only have to
//! register this system just for **every individual component/asset/resource**.
// ///! <div class="warning">
// ///! <a href="fn.component_dyn_tween_system.html"><code>component_dyn_tween_system</code></a> is type of dynamic
// ///! interpolator for <code>Box&lt;dyn Interpolator&gt;</code>.
// ///! </div>
//!
//! ```no_run
//! # mod a { // had to put this module here for some reason. tf?
//! # use bevy::prelude::*;
//! # use bevy_tween::prelude::*;
//! # #[derive(Component)]
//! # pub struct Foo {
//! #     a: f32,
//! #     b: f32,
//! #     c: f32,
//! # }
//! # mod my_interpolate {
//! #     use bevy::prelude::*;
//! #     use bevy_tween::prelude::*;
//! #     pub struct FooA {}
//! #     impl Interpolator for FooA {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooB {}
//! #     impl Interpolator for FooB {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! #     pub struct FooC {}
//! #     impl Interpolator for FooC {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, _item: &mut Self::Item, _value: f32) {
//! #             todo!()
//! #         }
//! #     }
//! # }
//! fn main() {
//!     use my_interpolate::*;
//!     use bevy_tween::component_dyn_tween_system;
//!
//!     // One system to rule them all
//!     // Note that we're only using the `Foo` type, not `FooA`, `FooB`,
//!     // and `FooC`!
//!     App::new().add_tween_systems(component_dyn_tween_system::<Foo>());
//!     // `component_dyn_tween_system` is just an alias for
//!     // `component_tween_system::<Box<dyn Interpolator<Item = ...>>>`
//! }
//! # }
//! ```
//!
//! [`BevyTweenRegisterSystems`]: crate::BevyTweenRegisterSystems
//! [`interpolate`]: crate::interpolate
//! [`DefaultTweenPlugins`]: crate::DefaultTweenPlugins
//! [`DefaultInterpolatorsPlugin`]: crate::interpolate::DefaultInterpolatorsPlugin
//! [`DefaultDynInterpolatorsPlugin`]: crate::interpolate::DefaultDynInterpolatorsPlugin

use bevy::prelude::*;

mod plugin;
mod system;

pub use plugin::*;
pub use system::*;

pub trait Set: Send + Sync + 'static {
    type Item;
    type Value;
    fn set(&self, item: &mut Self::Item, value: &Self::Value);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Setter<S>(pub S)
where
    S: Set;

impl<S> Setter<S>
where
    S: Set,
{
    fn new(set: S) -> Setter<S> {
        Setter(set)
    }
}

/// Skip a tween from tweening.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipTween;
