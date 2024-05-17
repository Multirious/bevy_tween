//! Module containing implementations for tween
//!
//! # [`Tween`]
//!
//! **Plugins**:
//! - [`DefaultTweenEventsPlugin`]
//!
//! **Components**:
//! - [`Tween<T, I>`]
//! - [`SkipTween`]
//! - [`SkipTweener`]
//! - [`TweenerMarker`]
//! - [`TweenProgress`]
//! - [`TweenInterpolationValue`]
//! - [`TweenEventData`]
//!
//! **Events**:
//! - [`TweenEvent`]
//!
//! **Systems**
//! - [`component_tween_system`]
//! - [`resource_tween_system`]
//! - [`asset_tween_system`]
//! - [`tween_event_system`]
//! - [`tween_event_taking_system`]
//!
//! Built-in targets:
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
//! This crate contains some systems with generic for tweening components, assets,
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
//!         pub start: f32,
//!         pub end: f32,
//!     }
//!
//!     impl Interpolator for FooA {
//!         type Item = super::Foo;
//!
//!         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!             item.a = self.start.lerp(self.end, value);
//!         }
//!     }
//!
//!     pub struct FooB {
//!         pub start: f32,
//!         pub end: f32,
//!     }
//!
//!     impl Interpolator for FooB {
//!         type Item = super::Foo;
//!
//!         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!             item.b = self.start.lerp(self.end, value);
//!         }
//!     }
//!
//!     pub struct FooC {
//!         pub start: f32,
//!         pub end: f32,
//!     }
//!
//!     impl Interpolator for FooC {
//!         type Item = super::Foo;
//!
//!         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//!             item.c = self.start.lerp(self.end, value);
//!         }
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
//! #     pub struct FooA {
//! #         pub start: f32,
//! #         pub end: f32,
//! #     }
//! #     impl Interpolator for FooA {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//! #             item.a = self.start.lerp(self.end, value);
//! #         }
//! #     }
//! #     pub struct FooB {
//! #         pub start: f32,
//! #         pub end: f32,
//! #     }
//! #     impl Interpolator for FooB {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//! #             item.b = self.start.lerp(self.end, value);
//! #         }
//! #     }
//! #     pub struct FooC {
//! #         pub start: f32,
//! #         pub end: f32,
//! #     }
//! #     impl Interpolator for FooC {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//! #             item.c = self.start.lerp(self.end, value);
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
//! #     pub struct FooA {
//! #         pub start: f32,
//! #         pub end: f32,
//! #     }
//! #     impl Interpolator for FooA {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//! #             item.a = self.start.lerp(self.end, value);
//! #         }
//! #     }
//! #     pub struct FooB {
//! #         pub start: f32,
//! #         pub end: f32,
//! #     }
//! #     impl Interpolator for FooB {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//! #             item.b = self.start.lerp(self.end, value);
//! #         }
//! #     }
//! #     pub struct FooC {
//! #         pub start: f32,
//! #         pub end: f32,
//! #     }
//! #     impl Interpolator for FooC {
//! #         type Item = super::Foo;
//! #         fn interpolate(&self, item: &mut Self::Item, value: f32) {
//! #             item.c = self.start.lerp(self.end, value);
//! #         }
//! #     }
//! # }
//! fn main() {
//!     use my_interpolate::*;
//!
//!     // One system to rule them all
//!     // Note that we're only using the `Foo` type, not `FooA`, `FooB`,
//!     // and `FooC`!
//!     App::new().add_tween_systems(bevy_tween::component_tween_system::<
//!         BoxedInterpolator<Foo>,
//!     >());
//!     // BoxedInterpolator definition:
//!     // type BoxedInterpolator<Item> = Box<dyn Interpolator<Item>>;
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

use crate::combinator::TargetState;
use crate::interpolate::Interpolator;
use crate::{utils, BevyTweenRegisterSystems};

mod systems;
#[allow(deprecated)]
#[cfg(feature = "bevy_asset")]
pub use systems::{
    apply_asset_tween_system, asset_dyn_tween_system, asset_tween_system,
    asset_tween_system_full,
};
#[allow(deprecated)]
pub use systems::{
    apply_component_tween_system, component_dyn_tween_system,
    component_tween_system, component_tween_system_full,
};
#[allow(deprecated)]
pub use systems::{
    apply_resource_tween_system, resource_dyn_tween_system,
    resource_tween_system, resource_tween_system_full,
};
pub use systems::{tween_event_system, tween_event_taking_system};

/// Skip a tween from tweening.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipTween;

#[allow(deprecated)]
mod man {
    use super::*;
    /// Skip a tweener from functioning.
    #[derive(
        Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect,
    )]
    #[reflect(Component)]
    #[deprecated(
        since = "0.5.0",
        note = "The timing inside this crate is moved to `bevy_time_runner`. Use `bevy_time_runner::SkipTimeRunner` instead."
    )]
    pub struct SkipTweener;
}
#[allow(deprecated)]
pub use man::SkipTweener;

#[deprecated(
    since = "0.5.0",
    note = "Use `bevy_time_runner::TimeSpanProgress` instead"
)]
pub use bevy_time_runner::TimeSpanProgress as TweenProgress;

/// Automatically managed by an [`Interpolation`] such as [`EaseFunction`] and
/// [`EaseClosure`] when a tween has the component [`TweenProgress`].
/// See [`sample_interpolations_system`]
///
/// [`sample_interpolations_system`]: crate::interpolation::sample_interpolations_system
/// [`Interpolation`]: crate::interpolation::Interpolation
/// [`EaseFunction`]: crate::interpolation::EaseFunction
/// [`EaseClosure`]: crate::interpolation::EaseClosure
#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)] // might want to use sparseset but i'm not sure yet
pub struct TweenInterpolationValue(pub f32);

/// Containing `target` and `interpolator`
#[derive(
    Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash, Reflect,
)]
#[reflect(Component)]
pub struct Tween<T, I> {
    #[allow(missing_docs)]
    pub target: T,
    #[allow(missing_docs)]
    pub interpolator: I,
}
impl<T, I> Tween<T, I>
where
    I: Interpolator,
{
    /// Create a new [`Tween`] with a target and an interpolator.
    pub fn new_target<G>(target: G, interpolator: I) -> Self
    where
        G: Into<T>,
    {
        Tween {
            interpolator,
            target: target.into(),
        }
    }
}

impl<T, I> Tween<T, I>
where
    T: Default,
    I: Interpolator,
{
    /// Create a new [`Tween`] with the default target and an interpolator.
    pub fn new(interpolator: I) -> Self {
        Tween::new_target(T::default(), interpolator)
    }

    /// Convert the current generic interpolator into a dynamic one.
    pub fn with_interpolator_boxed(
        self,
    ) -> Tween<T, Box<dyn Interpolator<Item = I::Item>>> {
        let Tween {
            target,
            interpolator,
        } = self;
        Tween {
            target,
            interpolator: Box::new(interpolator),
        }
    }
}

impl<T, Item> Tween<T, Box<dyn Interpolator<Item = Item>>>
where
    Item: 'static,
{
    /// Create a new [`Tween`] with a target and an interpolator that will be boxed internally.
    pub fn new_target_boxed<G, I>(target: G, interpolator: I) -> Self
    where
        G: Into<T>,
        I: Interpolator<Item = Item>,
    {
        Self::new_target(target, Box::new(interpolator))
    }
}

impl<T, Item> Tween<T, Box<dyn Interpolator<Item = Item>>>
where
    T: Default,
    Item: 'static,
{
    /// Create a new [`Tween`] with the default target and an interpolator that will be boxed internally.
    pub fn new_boxed<I>(interpolator: I) -> Self
    where
        I: Interpolator<Item = Item>,
    {
        Self::new(Box::new(interpolator))
    }
}

/// Useful for the implementor to specify what this *target* will return the
/// tweenable [`Self::Item`] which should match [`Interpolator::Item`].
/// See [`TargetComponent`], [`TargetResource`], and [`TargetAsset`].
#[deprecated(since = "0.3.0", note = "It's not really that useful")]
pub trait TweenTarget {
    /// Type to be interpolated
    type Item;
}

/// Convenient alias for [`Tween`] that [`TargetComponent`] with generic [`Interpolator`].
pub type ComponentTween<I> = Tween<TargetComponent, I>;

/// Convenient alias for [`Tween`] that [`TargetComponent`] with boxed dyanmic [`Interpolator`].
pub type ComponentDynTween<C> =
    Tween<TargetComponent, Box<dyn Interpolator<Item = C>>>;

/// Tell the tween what component of what entity to tween.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TargetComponent {
    /// Target the entity that contains this tween's tweener.
    TweenerEntity,
    /// Target the parent of this tween's tweener.
    TweenerParent,
    /// Target this entity.
    Entity(Entity),
    /// Target these entities.
    Entities(Vec<Entity>),
}

impl TargetComponent {
    /// Target the entity that contains this tween's tweener.
    pub fn tweener_entity() -> TargetComponent {
        TargetComponent::TweenerEntity
    }

    /// Target the parent of this tween's tweener.
    pub fn tweener_parent() -> TargetComponent {
        TargetComponent::TweenerParent
    }

    /// Target this entity.
    pub fn entity(entity: Entity) -> TargetComponent {
        TargetComponent::Entity(entity)
    }

    /// Target these entities.
    pub fn entities<I>(entities: I) -> TargetComponent
    where
        I: IntoIterator<Item = Entity>,
    {
        TargetComponent::from_iter(entities)
    }

    /// Create a new [`TargetState`] with the initial value out of this target.
    pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
        TargetState::new(self.clone(), value)
    }

    /// Create a new tween with the supplied interpolator out of this target.
    pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
        Tween {
            target: self.clone(),
            interpolator,
        }
    }

    /// Create a new tween with the supplied closure out of this target.
    pub fn with_closure<F, C>(
        &self,
        closure: F,
    ) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    where
        F: Fn(&mut C, f32) + Send + Sync + 'static,
        C: Component,
    {
        let closure = crate::interpolate::closure(closure);
        let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
        Tween {
            target: self.clone(),
            interpolator,
        }
    }
}

impl Default for TargetComponent {
    fn default() -> Self {
        TargetComponent::tweener_entity()
    }
}

impl From<Entity> for TargetComponent {
    fn from(value: Entity) -> Self {
        TargetComponent::entity(value)
    }
}

impl FromIterator<Entity> for TargetComponent {
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        TargetComponent::Entities(iter.into_iter().collect())
    }
}

impl<const N: usize> From<[Entity; N]> for TargetComponent {
    fn from(value: [Entity; N]) -> Self {
        TargetComponent::entities(value)
    }
}

impl From<Vec<Entity>> for TargetComponent {
    fn from(value: Vec<Entity>) -> Self {
        TargetComponent::entities(value)
    }
}

impl From<&Vec<Entity>> for TargetComponent {
    fn from(value: &Vec<Entity>) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl From<&[Entity]> for TargetComponent {
    fn from(value: &[Entity]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl<const N: usize> From<&[Entity; N]> for TargetComponent {
    fn from(value: &[Entity; N]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

/// A tweener must have this marker within the entity to let
/// [`ComponentTween`]s' system correctly search for the tweener that owns them.
#[derive(Debug, Default, PartialEq, Eq, Hash, Component, Reflect)]
pub struct TweenerMarker;

impl<I> ComponentTween<I>
where
    I: Interpolator,
    I::Item: Component,
{
    /// Convenient shortcut for targetting tweener's entity.
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_tween::prelude::*;
    /// # const interpolator: interpolate::Translation = interpolate::Translation { start: Vec3::ZERO, end: Vec3::ZERO };
    /// use bevy_tween::tween::TargetComponent;
    /// assert_eq!(
    ///     ComponentTween::tweener_entity(interpolator),
    ///     ComponentTween::new_target(
    ///         TargetComponent::TweenerEntity,
    ///         interpolator
    ///     )
    /// );
    /// ```
    pub fn tweener_entity(interpolator: I) -> Self {
        ComponentTween::new_target(
            TargetComponent::tweener_entity(),
            interpolator,
        )
    }

    /// Convenient shortcut for targetting tweener's parent.
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_tween::prelude::*;
    /// # const interpolator: interpolate::Translation = interpolate::Translation { start: Vec3::ZERO, end: Vec3::ZERO };
    /// use bevy_tween::tween::TargetComponent;
    /// assert_eq!(
    ///     ComponentTween::tweener_parent(
    ///         interpolator
    ///     ),
    ///     ComponentTween::new_target(
    ///         TargetComponent::TweenerParent,
    ///         interpolator
    ///     )
    /// );
    /// ```
    pub fn tweener_parent(interpolator: I) -> Self {
        ComponentTween::new_target(
            TargetComponent::tweener_parent(),
            interpolator,
        )
    }
}

impl<C> ComponentDynTween<C>
where
    C: Component,
{
    /// Convenient method for targetting tweener's entity.
    pub fn tweener_entity_boxed<I>(interpolator: I) -> Self
    where
        I: Interpolator<Item = C>,
    {
        ComponentTween::new_target(
            TargetComponent::tweener_entity(),
            Box::new(interpolator),
        )
    }

    /// Convenient method for targetting tweener's parent.
    pub fn tweener_parent_boxed<I>(interpolator: I) -> Self
    where
        I: Interpolator<Item = C>,
    {
        ComponentTween::new_target(
            TargetComponent::tweener_parent(),
            Box::new(interpolator),
        )
    }
}

/// Convenient alias for [`Tween`] that [`TargetResource`] with generic [`Interpolator`].
pub type ResourceTween<I> = Tween<TargetResource, I>;

/// Convenient alias for [`Tween`] that [`TargetResource`] with dyanmic [`Interpolator`].
pub type ResourceDynTween<R> =
    Tween<TargetResource, Box<dyn Interpolator<Item = R>>>;

/// Tell the tween what resource to tween.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct TargetResource;

impl TargetResource {
    /// New resource target
    pub fn new() -> TargetResource {
        TargetResource
    }

    /// Create a new [`TargetState`] with the initial value out of this target.
    pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
        TargetState::new(self.clone(), value)
    }

    /// Create a new tween with the supplied interpolator out of this target.
    pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
        Tween {
            target: self.clone(),
            interpolator,
        }
    }

    /// Create a new tween with the supplied closure out of this target.
    pub fn with_closure<F, C>(
        &self,
        closure: F,
    ) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    where
        F: Fn(&mut C, f32) + Send + Sync + 'static,
        C: Component,
    {
        let closure = crate::interpolate::closure(closure);
        let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
        Tween {
            target: self.clone(),
            interpolator,
        }
    }
}

/// Convenient alias for [`Tween`] that [`TargetAsset`] with generic [`Interpolator`].
#[cfg(feature = "bevy_asset")]
pub type AssetTween<I> = Tween<TargetAsset<<I as Interpolator>::Item>, I>;

/// Convenient alias for [`Tween`] that [`TargetAsset`] with dyanmic [`Interpolator`].
#[cfg(feature = "bevy_asset")]
pub type AssetDynTween<A> =
    Tween<TargetAsset<A>, Box<dyn Interpolator<Item = A>>>;

impl<I> AssetTween<I>
where
    I: Interpolator,
    I::Item: Asset,
{
    /// Set the target to the supplied asset.
    pub fn for_asset(mut self, asset: Handle<I::Item>) -> Self {
        self.target = TargetAsset::Asset(asset);
        self
    }

    /// Set the target to the supplied assets.
    pub fn for_assets<Iter>(mut self, assets: Iter) -> Self
    where
        Iter: IntoIterator<Item = Handle<I::Item>>,
    {
        self.target = TargetAsset::from_iter(assets);
        self
    }
}

/// Tell the tween what asset of what type to tween.
#[cfg(feature = "bevy_asset")]
#[derive(Debug, PartialEq, Eq, Hash, Reflect)]
pub enum TargetAsset<A: Asset>
where
    A: Asset,
{
    /// Target this asset
    Asset(Handle<A>),
    /// Target these assets
    Assets(Vec<Handle<A>>),
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> TargetAsset<A> {
    /// Target this asset
    pub fn asset(asset: Handle<A>) -> Self {
        TargetAsset::Asset(asset)
    }

    /// Target these assets
    pub fn assets<I>(assets: I) -> Self
    where
        I: IntoIterator<Item = Handle<A>>,
    {
        TargetAsset::from_iter(assets)
    }

    /// Create a new [`TargetState`] with the initial value out of this target.
    pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
        TargetState::new(self.clone(), value)
    }

    /// Create a new tween with the supplied interpolator out of this target.
    pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
        Tween {
            target: self.clone(),
            interpolator,
        }
    }

    /// Create a new tween with the supplied closure out of this target.
    pub fn with_closure<F, C>(
        &self,
        closure: F,
    ) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    where
        F: Fn(&mut C, f32) + Send + Sync + 'static,
        C: Component,
    {
        let closure = crate::interpolate::closure(closure);
        let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
        Tween {
            target: self.clone(),
            interpolator,
        }
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> Clone for TargetAsset<A> {
    fn clone(&self) -> Self {
        match self {
            TargetAsset::Asset(handle) => TargetAsset::Asset(handle.clone()),
            TargetAsset::Assets(v) => TargetAsset::Assets(v.clone()),
        }
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> Default for TargetAsset<A> {
    fn default() -> Self {
        TargetAsset::Asset(Default::default())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<Handle<A>> for TargetAsset<A> {
    fn from(value: Handle<A>) -> Self {
        TargetAsset::Asset(value)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> FromIterator<Handle<A>> for TargetAsset<A> {
    fn from_iter<T: IntoIterator<Item = Handle<A>>>(iter: T) -> Self {
        TargetAsset::Assets(iter.into_iter().collect())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> From<[Handle<A>; N]> for TargetAsset<A> {
    fn from(value: [Handle<A>; N]) -> Self {
        TargetAsset::assets(value)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<Vec<Handle<A>>> for TargetAsset<A> {
    fn from(value: Vec<Handle<A>>) -> Self {
        TargetAsset::assets(value)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<&Vec<Handle<A>>> for TargetAsset<A> {
    fn from(value: &Vec<Handle<A>>) -> Self {
        TargetAsset::assets(value.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<&[Handle<A>]> for TargetAsset<A> {
    fn from(value: &[Handle<A>]) -> Self {
        TargetAsset::assets(value.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> From<&[Handle<A>; N]> for TargetAsset<A> {
    fn from(value: &[Handle<A>; N]) -> Self {
        TargetAsset::assets(value.iter().cloned())
    }
}

/// Default event and systems:
/// - [`tween_event_system::<()>`], [`TweenEvent<()>`]
/// - [`tween_event_system::<&'static str>`], [`TweenEvent<&'static str>`]
pub struct DefaultTweenEventsPlugin;

impl Plugin for DefaultTweenEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_tween_systems(systems::tween_event_system::<()>)
            .add_event::<TweenEvent>()
            .add_tween_systems(systems::tween_event_system::<&'static str>)
            .add_event::<TweenEvent<&'static str>>();
    }
}

/// Fires [`TweenEvent`] whenever [`TweenProgress`] and [`TweenEventData`] exist in the same entity.
///
/// # Examples
///
/// ```
#[doc = utils::doc_test_boilerplate!()]
/// commands
///     .spawn((SpanTweenerBundle::new(Duration::from_secs(5))))
///     .with_children(|c| {
///         // The event will be fired repetitively every frame
///         // between the second 2 and 3.
///         c.spawn((
///             SpanTweenBundle::new(
///                 Duration::from_secs(2)..Duration::from_secs(3),
///             ),
///             TweenEventData::new(),
///         ));
///
///         // The event will be fired once at the second 1.
///         c.spawn((
///             SpanTweenBundle::new(
///                 Duration::from_secs(1)..=Duration::from_secs(1),
///             ),
///             TweenEventData::new(),
///         ));
///     });
/// ```
/// 
/// ## Using custom data
///
/// You have to regsiter [`tween_event_system`] or [`tween_event_taking_system`]
/// before using custom data with [`TweenEvent<Data>`]. And add your custom event.
/// Check [`DefaultTweenEventsPlugin`] for built-in events.
/// ```
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Clone)]
/// enum MyTweenData {
///     Idle,
///     Fly,
/// }
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultTweenPlugins)
///         .add_tween_systems(bevy_tween::tween_event_system::<MyTweenData>)
///         .add_event::<TweenEvent<MyTweenData>>();
/// }
/// ```
/// ```
#[doc = utils::doc_test_boilerplate!()]
/// # #[derive(Clone)]
/// # enum MyTweenData {
/// #     Idle,
/// #     Fly,
/// # }
/// #
/// commands
///     .spawn((SpanTweenerBundle::new(Duration::from_secs(5))))
///     .with_children(|c| {
///
///         // The `TweenEvent<MyTweenData>` event will be fired once at the second 2.
///         c.spawn((
///             SpanTweenBundle::new(
///                 Duration::from_secs(2)..=Duration::from_secs(2),
///             ),
///             TweenEventData::with_data(MyTweenData::Idle),
///         ));
///
///         // The `TweenEvent<MyTweenData>` event will be fired once at the second 3.
///         c.spawn((
///             SpanTweenBundle::new(
///                 Duration::from_secs(3)..=Duration::from_secs(3),
///             ),
///             TweenEventData::with_data(MyTweenData::Fly),
///         ));
///     });
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct TweenEventData<Data = ()>(pub Option<Data>)
where
    Data: Send + Sync + 'static;

impl<Data: Send + Sync + 'static> TweenEventData<Data> {
    /// Create new [`TweenEventData`] with custom user data.
    pub fn with_data(data: Data) -> Self {
        TweenEventData(Some(data))
    }
}

impl TweenEventData<()> {
    /// Create new [`TweenEventData`] with no custom user data, simply `Some(())`.
    pub fn new() -> Self {
        TweenEventData(Some(()))
    }
}

impl<Data> TweenEventData<Data>
where
    Data: Send + Sync + 'static,
{
    /// Create new [`TweenEventData`] with `None` value.
    pub fn none() -> Self {
        TweenEventData(None)
    }
}

/// Fires whenever [`TweenProgress`] and [`TweenEventData`] exist in the same entity
/// by [`tween_event_system`] or [`tween_event_taking_system`].
#[derive(Debug, Clone, PartialEq, Event, Reflect)]
pub struct TweenEvent<Data = ()> {
    /// Custom user data
    pub data: Data,
    /// Progress percentage of the tween
    pub progress: TweenProgress,
    /// Sampled value of an interpolation.
    pub interpolation_value: Option<f32>,
    /// The entity
    pub entity: Entity,
}
