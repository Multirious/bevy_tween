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

use crate::combinator::TargetState;
use crate::interpolate::Interpolator;

mod systems;
#[cfg(feature = "bevy_asset")]
pub use systems::{
    apply_asset_tween_system, asset_dyn_tween_system, asset_tween_system,
};
pub use systems::{
    apply_component_tween_system, component_dyn_tween_system,
    component_tween_system,
};
pub use systems::{
    apply_resource_tween_system, resource_dyn_tween_system,
    resource_tween_system,
};

/// Skip a tween from tweening.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipTween;

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

/// Convenient alias for [`Tween`] that [`TargetComponent`] with generic [`Interpolator`].
pub type ComponentTween<I> = Tween<TargetComponent, I>;

/// Convenient alias for [`Tween`] that [`TargetComponent`] with boxed dynamic [`Interpolator`].
pub type ComponentDynTween<C> =
    Tween<TargetComponent, Box<dyn Interpolator<Item = C>>>;

/// Tell the tween what component of what entity to tween.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TargetComponent {
    /// The target is not yet selected or resolved.
    None,
    /// Target this entity.
    Entity(Entity),
    /// Target these entities.
    Entities(Vec<Entity>),
}

impl TargetComponent {
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
        TargetComponent::None
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

impl<I> ComponentTween<I>
where
    I: Interpolator,
    I::Item: Component,
{
}

impl<C> ComponentDynTween<C> where C: Component {}

/// Convenient alias for [`Tween`] that [`TargetResource`] with generic [`Interpolator`].
pub type ResourceTween<I> = Tween<TargetResource, I>;

/// Convenient alias for [`Tween`] that [`TargetResource`] with dyanmic [`Interpolator`].
pub type ResourceDynTween<R> =
    Tween<TargetResource, Box<dyn Interpolator<Item = R>>>;

/// Tell the tween what resource to tween.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
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

/// Convenient alias for [`Tween`] that [`TargetAsset`] with dynamic [`Interpolator`].
#[cfg(feature = "bevy_asset")]
pub type AssetDynTween<A> =
    Tween<TargetAsset<A>, Box<dyn Interpolator<Item = A>>>;

/// Tell the tween what asset of what type to tween.
#[cfg(feature = "bevy_asset")]
#[derive(Debug, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TargetAsset<A: Asset>
where
    A: Asset,
{
    /// The target is not yet selected or resolved.
    None,
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
            TargetAsset::None => TargetAsset::None,
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

/// Trait for type to convert into a target type.
pub trait IntoTarget {
    /// The target type
    type Target;

    /// Convert [`Self`] into [`Self::Target`]
    fn into_target(self) -> Self::Target;
}

impl IntoTarget for Entity {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entity(self)
    }
}

impl<const N: usize> IntoTarget for [Entity; N] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self)
    }
}

impl IntoTarget for Vec<Entity> {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self)
    }
}

impl IntoTarget for &[Entity] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

impl<const N: usize> IntoTarget for &[Entity; N] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

impl IntoTarget for &Vec<Entity> {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A> IntoTarget for Handle<A>
where
    A: Asset,
{
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::asset(self)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> IntoTarget for [Handle<A>; N] {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> IntoTarget for Vec<Handle<A>> {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> IntoTarget for &[Handle<A>] {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> IntoTarget for &[Handle<A>; N] {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> IntoTarget for &Vec<Handle<A>> {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self.iter().cloned())
    }
}

#[deprecated(
    since = "0.6.0",
    note = "use `bevy_tween::tween_event::TweenEvent` instead"
)]
#[allow(missing_docs)]
#[doc(hidden)]
pub type TweenEvent<Data> = crate::tween_event::TweenEvent<Data>;

#[deprecated(
    since = "0.6.0",
    note = "use `bevy_tween::tween_event::TweenEventData` instead"
)]
#[allow(missing_docs)]
#[doc(hidden)]
pub type TweenEventData<Data> = crate::tween_event::TweenEventData<Data>;

#[deprecated(
    since = "0.6.0",
    note = "use `bevy_tween::tween_event::DefaultTweenEventPlugins` instead"
)]
#[allow(missing_docs)]
#[doc(hidden)]
pub type DefaultTweenEventsPlugin =
    crate::tween_event::DefaultTweenEventPlugins;

#[deprecated(
    since = "0.6.0",
    note = "use `bevy_tween::tween_event::tween_event_system` instead or `TweenEventPlugin` if you're registering custom tween event"
)]
#[allow(missing_docs)]
#[doc(hidden)]
#[allow(deprecated)]
#[allow(clippy::type_complexity)]
pub fn tween_event_system<Data>(
    q_tween_event_data: Query<
        (
            Entity,
            &TweenEventData<Data>,
            &bevy_time_runner::TimeSpanProgress,
            Option<&crate::curve::CurveValue>,
        ),
        Without<SkipTween>,
    >,
    event_writer: EventWriter<TweenEvent<Data>>,
) where
    Data: Clone + Send + Sync + 'static,
{
    crate::tween_event::tween_event_system(q_tween_event_data, event_writer)
}

#[deprecated(
    since = "0.6.0",
    note = "use `bevy_tween::tween_event::tween_event_taking_system` instead or `TweenEventTakingPlugin` if you're registering custom tween event"
)]
#[allow(missing_docs)]
#[doc(hidden)]
#[allow(deprecated)]
#[allow(clippy::type_complexity)]
pub fn tween_event_taking_system<Data>(
    q_tween_event_data: Query<
        (
            Entity,
            &mut TweenEventData<Data>,
            &bevy_time_runner::TimeSpanProgress,
            Option<&crate::curve::CurveValue>,
        ),
        Without<SkipTween>,
    >,
    event_writer: EventWriter<TweenEvent<Data>>,
) where
    Data: Send + Sync + 'static,
{
    crate::tween_event::tween_event_taking_system(
        q_tween_event_data,
        event_writer,
    )
}
