//! Module containg implementations for tween
//!
//! # [`Tween`]
//!
//! Built-in supported [`TweenTarget`]s are:
//! - [`TargetComponent`]
//! - [`TargetResource`]
//! - [`TargetAsset`]
//!
//! See available interpolator in [`interpolate`].
//!
//! ## Registering systems
//!
//! You have to register some necessary systems for this plugin to work properly
//! with every custom type and interpolators
//! unless there's some system already registered by the [`DefaultTweenPlugins`].
//!
//! This crate already contains some systems for tweening components, assets,
//! and resources.
//! You will usually use aliases of these systems instead to reduce specifying
//! redundant generic.
//!
//! Built-in systems:
//! - [`component_tween_system_full()`], component tweening system
//!   - [`component_tween_system()`], alias system for generic interpolator
//!   - [`component_dyn_tween_system()`], alias system for `Box<dyn Interpolator>`
//! - [`resource_tween_system_full()`], resource tweening system
//!   - [`resource_tween_system()`], alias system for generic interpolator
//!   - [`resource_dyn_tween_system()`], alias system for `Box<dyn Interpolator>`
//! - [`asset_tween_system_full()`], asset tweening system
//!   - [`asset_tween_system()`], alias system for generic interpolator
//!   - [`asset_dyn_tween_system()`], alias system for `Box<dyn Interpolator>`
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
//! There's 2 type of system you might want to register.
//!
//! ### Registering system for generic interpolator
//!
//! Generic interpolator means we're not using any dynamic dispatch.
//! We've to register this system for **every individual interpolator**.
//! (Unless already registered by the [`DefaultTweenPlugins`])
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
//! Dynamic interpolator means we're using dynamic dispatch.
//! We don't have to register system for every interpolator, we only have to
//! register this system just for **every individual component**.
//! (Unless already registered by the [`DefaultTweenPlugins`])
//!
//! To register a dynamic interpolator for your component, you can use
//! [`component_dyn_tween_system`].
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
//!     use bevy_tween::component_dyn_tween_system;
//!     use my_interpolate::*;
//!
//!     // One system to rule them all
//!     // Note that we're only using the `Foo` type, not `FooA`, `FooB`,
//!     // and `FooC`!
//!     App::new().add_tween_systems(component_dyn_tween_system::<Foo>());
//! }
//! # }
//! ```
//!
//! [`BevyTweenRegisterSystems`]: crate::BevyTweenRegisterSystems
//! [`interpolate`]: crate::interpolate
//! [`DefaultTweenPlugins`]: crate::DefaultTweenPlugins

use std::{any::type_name, marker::PhantomData, time::Duration};

use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;

use crate::interpolate::Interpolator;
use crate::tween_timer::AnimationDirection;

/// [`TweenState`] should be automatically managed by a tweener.
/// User just have to add this component to a tween entity and an assigned
/// tweener will take care of it.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect,
)]
#[reflect(Component)]
pub struct TweenState {
    /// `local_elasped` is None meaning that the tween playing that is managing this
    /// tween hasn't/has elasped pass this tween.
    pub local_elasped: Option<Duration>,
    #[allow(missing_docs)]
    pub local_previous_elasped: Option<Duration>,
    /// Direction of currently elasped time
    pub direction: AnimationDirection,
    /// Maximum duration of the this tween.
    pub local_end: Duration,
}

/// Automatically managed by an [`Interpolation`] such as [`EaseFunction`] and
/// [`EaseClosure`] when a tween has the component `TweenState`.
/// See [`sample_interpolations_system`]
///
/// [`sample_interpolations_system`]: crate::interpolation::sample_interpolations_system
/// [`Interpolation`]: crate::interpolation::Interpolation
/// [`EaseFunction`]: crate::interpolation::EaseFunction
/// [`EaseClosure`]: crate::interpolation::EaseClosure
#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)] // might want to use sparseset but i'm not sure yet
pub struct TweenInterpolationValue(pub f32);

/// Containg [`TweenTarget`] and [`Interpolator`]
#[derive(
    Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash, Reflect,
)]
#[reflect(Component)]
pub struct Tween<T, I>
where
    T: TweenTarget,
    I: Interpolator<Item = T::Item>,
{
    #[allow(missing_docs)]
    pub target: T,
    #[allow(missing_docs)]
    pub interpolator: I,
}
impl<T, I> Tween<T, I>
where
    T: TweenTarget,
    I: Interpolator<Item = T::Item>,
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
    T: TweenTarget + Default,
    I: Interpolator<Item = T::Item>,
{
    /// Create a new [`Tween`] with the default target and an interpolator.
    pub fn new(interpolator: I) -> Self {
        Tween::new_target(T::default(), interpolator)
    }
}

impl<T> Tween<T, Box<dyn Interpolator<Item = T::Item>>>
where
    T: TweenTarget,
    T::Item: 'static,
{
    /// Create a new [`Tween`] with a target and an interpolator that will be boxed internally.
    pub fn new_target_boxed<G, I>(target: G, interpolator: I) -> Self
    where
        G: Into<T>,
        I: Interpolator<Item = T::Item>,
    {
        Self::new_target(target, Box::new(interpolator))
    }
}

impl<T> Tween<T, Box<dyn Interpolator<Item = T::Item>>>
where
    T: TweenTarget + Default,
    T::Item: 'static,
{
    /// Create a new [`Tween`] with the default target and an interpolator that will be boxed internally.
    pub fn new_boxed<I>(interpolator: I) -> Self
    where
        I: Interpolator<Item = T::Item>,
    {
        Self::new(Box::new(interpolator))
    }
}

/// Useful for the implementor to specify what this *target* will return the
/// tweenable [`Self::Item`] which should match [`Interpolator::Item`].
/// See [`TargetComponent`], [`TargetResource`], and [`TargetAsset`].
pub trait TweenTarget {
    /// Type to be interpolated
    type Item;
}

/// Convenient alias for [`Tween`] that [`TargetComponent`] with generic [`Interpolator`].
pub type ComponentTween<I> =
    Tween<TargetComponent<<I as Interpolator>::Item>, I>;

/// Convenient alias for [`Tween`] that [`TargetComponent`] with boxed dyanmic [`Interpolator`].
pub type ComponentDynTween<C> =
    Tween<TargetComponent<C>, Box<dyn Interpolator<Item = C>>>;

/// Tell the tween what component of what entity to tween.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TargetComponent<C>
where
    C: Component,
{
    /// Target the entity that contains this tween's tweener.
    TweenerEntity(#[reflect(ignore)] PhantomData<C>),
    /// Target the parent of this tween's tweener.
    TweenerParent(#[reflect(ignore)] PhantomData<C>),
    /// Target this entity.
    Entity(Entity, #[reflect(ignore)] PhantomData<C>),
    /// Target these entities.
    Entities(Vec<Entity>, #[reflect(ignore)] PhantomData<C>),
}

impl<C> TargetComponent<C>
where
    C: Component,
{
    /// Target the entity that contains this tween's tweener.
    pub fn tweener_entity() -> TargetComponent<C> {
        TargetComponent::TweenerEntity(PhantomData)
    }

    /// Target the parent of this tween's tweener.
    pub fn tweener_parent() -> TargetComponent<C> {
        TargetComponent::TweenerParent(PhantomData)
    }

    /// Target this entity.
    pub fn entity(entity: Entity) -> TargetComponent<C> {
        TargetComponent::Entity(entity, PhantomData)
    }

    /// Target these entities.
    pub fn entities<I>(entities: I) -> TargetComponent<C>
    where
        I: IntoIterator<Item = Entity>,
    {
        TargetComponent::from_iter(entities)
    }
}

impl<C> TweenTarget for TargetComponent<C>
where
    C: Component,
{
    type Item = C;
}

impl<C> Default for TargetComponent<C>
where
    C: Component,
{
    fn default() -> Self {
        TargetComponent::tweener_entity()
    }
}

impl<C> From<Entity> for TargetComponent<C>
where
    C: Component,
{
    fn from(value: Entity) -> Self {
        TargetComponent::entity(value)
    }
}

impl<C> FromIterator<Entity> for TargetComponent<C>
where
    C: Component,
{
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        TargetComponent::Entities(iter.into_iter().collect(), PhantomData)
    }
}

impl<C, const N: usize> From<[Entity; N]> for TargetComponent<C>
where
    C: Component,
{
    fn from(value: [Entity; N]) -> Self {
        TargetComponent::entities(value)
    }
}

impl<C> From<Vec<Entity>> for TargetComponent<C>
where
    C: Component,
{
    fn from(value: Vec<Entity>) -> Self {
        TargetComponent::entities(value)
    }
}

impl<C> From<&Vec<Entity>> for TargetComponent<C>
where
    C: Component,
{
    fn from(value: &Vec<Entity>) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl<C> From<&[Entity]> for TargetComponent<C>
where
    C: Component,
{
    fn from(value: &[Entity]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl<C, const N: usize> From<&[Entity; N]> for TargetComponent<C>
where
    C: Component,
{
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
    /// Convenient method for targetting tweener's entity.
    pub fn tweener_entity(interpolator: I) -> Self {
        ComponentTween::new_target(
            TargetComponent::tweener_entity(),
            interpolator,
        )
    }

    /// Convenient method for targetting tweener's parent.
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

/// Tween any [`Tween`] with the [`Interpolator`] that [`TargetComponent`] with
/// value provided by [`TweenInterpolationValue`] component.
#[allow(clippy::type_complexity)]
#[deprecated(
    since = "0.3.0",
    note = "Use `component_tween_system` instead with less required generic"
)]
pub fn component_tween_system_full<C, I>(
    q_tweener: Query<(Option<&Parent>, Has<TweenerMarker>)>,
    q_tween: Query<(
        Entity,
        &Tween<TargetComponent<C>, I>,
        &TweenInterpolationValue,
    )>,
    q_component: Query<&mut I::Item>,
) where
    C: Component,
    I: Interpolator<Item = C> + Send + Sync + 'static,
{
    component_tween_system(q_tweener, q_tween, q_component);
}

/// Tween any [`Tween`] with the [`Interpolator`] that [`TargetComponent`] with
/// value provided by [`TweenInterpolationValue`] component.
#[allow(clippy::type_complexity)]
pub fn component_tween_system<I>(
    q_tweener: Query<(Option<&Parent>, Has<TweenerMarker>)>,
    q_tween: Query<(
        Entity,
        &Tween<TargetComponent<I::Item>, I>,
        &TweenInterpolationValue,
    )>,
    mut q_component: Query<&mut I::Item>,
) where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Component,
{
    q_tween.iter().for_each(|(entity, tween, ease_value)| {
        let target = match &tween.target {
            TargetComponent::TweenerEntity(_) => match q_tweener.get(entity) {
                Ok((_, true)) => entity,
                Ok((Some(this_parent), false)) => {
                    match q_tweener.get(this_parent.get()) {
                        Ok((_, true)) => this_parent.get(),
                        _ => return,
                    }
                }
                _ => return,
            },
            TargetComponent::TweenerParent(_) => match q_tweener.get(entity) {
                Ok((Some(this_parent), true)) => this_parent.get(),
                Ok((Some(this_parent), false)) => {
                    match q_tweener.get(this_parent.get()) {
                        Ok((Some(tweener_parent), true)) => {
                            tweener_parent.get()
                        }
                        _ => return,
                    }
                }
                _ => return,
            },
            TargetComponent::Entity(e, _) => *e,
            TargetComponent::Entities(e, _) => {
                for &target in e {
                    let mut target_component = match q_component.get_mut(target)
                    {
                        Ok(target_component) => target_component,
                        Err(e) => {
                            warn!(
                                "{} query error: {e}",
                                type_name::<ComponentTween<I>>()
                            );
                            continue;
                        }
                    };
                    tween
                        .interpolator
                        .interpolate(&mut target_component, ease_value.0);
                }
                return;
            }
        };

        let mut target_component = match q_component.get_mut(target) {
            Ok(target_component) => target_component,
            Err(e) => {
                warn!("{} query error: {e}", type_name::<ComponentTween<I>>());
                return;
            }
        };
        tween
            .interpolator
            .interpolate(&mut target_component, ease_value.0);
    })
}

/// System alias for [`component_tween_system`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
#[deprecated(
    since = "0.3.0",
    note = "Use `component_tween_system::<BoxedInterpolator<...>>` for consistency"
)]
pub fn component_dyn_tween_system<C>() -> SystemConfigs
where
    C: Component,
{
    component_tween_system::<Box<dyn Interpolator<Item = C>>>.into_configs()
}

/// Convenient alias for [`Tween`] that [`TargetResource`] with generic [`Interpolator`].
pub type ResourceTween<I> = Tween<TargetResource<<I as Interpolator>::Item>, I>;

/// Convenient alias for [`Tween`] that [`TargetResource`] with dyanmic [`Interpolator`].
pub type ResourceTweenDyn<R> =
    Tween<TargetResource<R>, Box<dyn Interpolator<Item = R>>>;

/// Tell the tween what resource to tween.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct TargetResource<R>(#[reflect(ignore)] pub PhantomData<R>)
where
    R: Resource;

impl<R> TargetResource<R>
where
    R: Resource,
{
    /// New resource target
    pub fn new() -> TargetResource<R> {
        TargetResource(PhantomData)
    }
}

impl<R> TweenTarget for TargetResource<R>
where
    R: Resource,
{
    type Item = R;
}

/// Tween any [`Tween`] with the [`Interpolator`] that [`TargetResource`] with
/// value provided by [`TweenInterpolationValue`] component.
#[deprecated(
    since = "0.3.0",
    note = "Use `resource_tween_system` instead with less required generic"
)]
pub fn resource_tween_system_full<R, I>(
    q_tween: Query<(&Tween<TargetResource<R>, I>, &TweenInterpolationValue)>,
    resource: Option<ResMut<I::Item>>,
) where
    R: Resource,
    I: Interpolator<Item = R> + Send + Sync + 'static,
{
    resource_tween_system(q_tween, resource);
}

/// System alias for [`resource_tween_system_full`] that uses generic [`Interpolator`]..
#[allow(clippy::type_complexity)]
pub fn resource_tween_system<I>(
    q_tween: Query<(
        &Tween<TargetResource<I::Item>, I>,
        &TweenInterpolationValue,
    )>,
    resource: Option<ResMut<I::Item>>,
) where
    I: Interpolator,
    I::Item: Resource,
{
    let Some(mut resource) = resource else {
        warn!("Resource does not exists for a resource tween.");
        return;
    };
    q_tween.iter().for_each(|(tween, ease_value)| {
        tween.interpolator.interpolate(&mut resource, ease_value.0);
    })
}

/// System alias for [`resource_tween_system_full`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
#[deprecated(
    since = "0.3.0",
    note = "Use `resource_tween_system::<BoxedInterpolator<...>>` for consistency"
)]
pub fn resource_dyn_tween_system<R>() -> SystemConfigs
where
    R: Resource,
{
    resource_tween_system::<Box<dyn Interpolator<Item = R>>>.into_configs()
}

/// Convenient alias for [`Tween`] that [`TargetAsset`] with generic [`Interpolator`].
#[cfg(feature = "bevy_asset")]
pub type AssetTween<I> = Tween<TargetAsset<<I as Interpolator>::Item>, I>;

/// Convenient alias for [`Tween`] that [`TargetAsset`] with dyanmic [`Interpolator`].
#[cfg(feature = "bevy_asset")]
pub type AssetDynTween<A> =
    Tween<TargetAsset<A>, Box<dyn Interpolator<Item = A>>>;

/// Tell the tween what asset of what type to tween.
#[cfg(feature = "bevy_asset")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
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
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> TweenTarget for TargetAsset<A> {
    type Item = A;
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

/// Tween any [`Tween`] with the [`Interpolator`] that [`TargetAsset`] with
/// value provided by [`TweenInterpolationValue`] component.
#[cfg(feature = "bevy_asset")]
#[deprecated(
    since = "0.3.0",
    note = "Use `asset_tween_system` instead with less required generic"
)]
pub fn asset_tween_system_full<A, I>(
    q_tween: Query<(&Tween<TargetAsset<A>, I>, &TweenInterpolationValue)>,
    asset: Option<ResMut<Assets<I::Item>>>,
) where
    A: Asset,
    I: Interpolator<Item = A> + Send + Sync + 'static,
{
    asset_tween_system(q_tween, asset);
}

/// System alias for [`asset_tween_system_full`] that uses generic [`Interpolator`].
#[cfg(feature = "bevy_asset")]
#[allow(clippy::type_complexity)]
pub fn asset_tween_system<I>(
    q_tween: Query<(&Tween<TargetAsset<I::Item>, I>, &TweenInterpolationValue)>,
    asset: Option<ResMut<Assets<I::Item>>>,
) where
    I: Interpolator,
    I::Item: Asset,
{
    let Some(mut asset) = asset else {
        warn!("Asset resource does not exists for an asset tween.");
        return;
    };
    q_tween
        .iter()
        .for_each(|(tween, ease_value)| match &tween.target {
            TargetAsset::Asset(a) => {
                let Some(asset) = asset.get_mut(a) else {
                    warn!("Asset not found for an asset tween");
                    return;
                };
                tween.interpolator.interpolate(asset, ease_value.0);
            }
            TargetAsset::Assets(assets) => {
                for a in assets {
                    let Some(a) = asset.get_mut(a) else {
                        warn!("Asset not found for an asset tween");
                        continue;
                    };
                    tween.interpolator.interpolate(a, ease_value.0);
                }
            }
        })
}

/// System alias for [`asset_tween_system_full`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
#[cfg(feature = "bevy_asset")]
#[deprecated(
    since = "0.3.0",
    note = "Use `asset_tween_system::<BoxedInterpolator<...>>` for consistency"
)]
pub fn asset_dyn_tween_system<A>() -> SystemConfigs
where
    A: Asset,
{
    asset_tween_system::<Box<dyn Interpolator<Item = A>>>.into_configs()
}
