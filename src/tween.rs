//! Module containg implementations for tween
//!
//! This crate currently have 2 tween implementation which is:
//! - [`Tween`], containg information about a target and a lens which uses
//!   generic all the way.
//!   Built-in supported [`TweenTarget`]s are:
//!   - [`TargetComponent`]
//!   - [`TargetResource`]
//!   - [`TargetAsset`]
//!   
//! - [`TweenBoxed`], like [`Tween`] but the inner [`Interpolator`] is boxed which
//!   came with the pros and cons of boxing such as missing reflect but let you
//!   use closure as a [`Interpolator`]!.
//!
//! See available lenses in [`lenses`].
//!
//! [`lenses`]: crate::lenses

use bevy::prelude::*;
use std::{marker::PhantomData, time::Duration};

#[cfg(any(feature = "tween_boxed", feature = "tween_unboxed",))]
use crate::lenses::Interpolator;
use crate::tween_player::AnimationDirection;
#[cfg(any(feature = "tween_boxed", feature = "tween_unboxed",))]
use std::any::type_name;

/// `TweenState` should be automatically managed by a tween player.
/// User just have to add this component to a tween entity and an assigned
/// tween player will take care of it.
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
/// See [`sample_interpolator_system`]
///
/// [`sample_interpolator_system`]: crate::interpolation::sample_interpolator_system
/// [`Interpolation`]: crate::interpolation::Interpolation
/// [`EaseFunction`]: crate::interpolation::EaseFunction
/// [`EaseClosure`]: crate::interpolation::EaseClosure
#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct TweenInterpolationValue(pub f32);

/// A [`Tween`] is used as a information for a tween player like:
/// - "What to tween" by using [`TweenTarget`]
/// - "How to tween" by using [`Interpolator`]
///
/// [`Interpolator`]: crate::lenses::Interpolator
#[cfg(feature = "tween_unboxed")]
#[derive(
    Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash, Reflect,
)]
#[reflect(Component)]
pub struct Tween<T, L>
where
    T: TweenTarget,
    L: Interpolator<Item = T::Item>,
{
    #[allow(missing_docs)]
    pub target: T,
    #[allow(missing_docs)]
    pub lens: L,
}
#[cfg(feature = "tween_unboxed")]
impl<T, L> Tween<T, L>
where
    T: TweenTarget,
    L: Interpolator<Item = T::Item>,
{
    /// Create a new [`Tween`] with the following target and lens.
    pub fn new_target<G>(target: G, lens: L) -> Self
    where
        G: Into<T>,
    {
        Tween {
            lens,
            target: target.into(),
        }
    }
}

#[cfg(feature = "tween_unboxed")]
impl<T> Tween<T, fn(&mut T::Item, f32)>
where
    T: TweenTarget,
{
    /// Create a new [`Tween`] with the following target and lens as function pointer.
    pub fn new_target_map<G>(target: G, lens: fn(&mut T::Item, f32)) -> Self
    where
        G: Into<T>,
    {
        Tween {
            lens,
            target: target.into(),
        }
    }
}

#[cfg(feature = "tween_unboxed")]
impl<T, L> Tween<T, L>
where
    T: TweenTarget + Default,
    L: Interpolator<Item = T::Item>,
{
    /// Create a new [`Tween`] with the following lens and using the default target.
    pub fn new(lens: L) -> Self {
        Tween::new_target(T::default(), lens)
    }
}

#[cfg(feature = "tween_unboxed")]
impl<T> Tween<T, fn(&mut T::Item, f32)>
where
    T: TweenTarget + Default,
{
    /// Create a new [`Tween`] with the following lens as a function pointer and using the default target.
    pub fn new_map(lens: fn(&mut T::Item, f32)) -> Self {
        Tween::new_target(T::default(), lens)
    }
}

/// [`Tween`] but the inner lens is boxed.
///
/// See [`Tween`] for more information.
#[cfg(feature = "tween_boxed")]
#[derive(Component)]
pub struct TweenBoxed<T>
where
    T: TweenTarget,
{
    #[allow(missing_docs)]
    pub target: T,
    #[allow(missing_docs)]
    pub lens: Box<dyn Interpolator<Item = T::Item> + Send + Sync + 'static>,
}
#[cfg(feature = "tween_boxed")]
impl<T> TweenBoxed<T>
where
    T: TweenTarget,
{
    /// Create a new [`TweenBoxed`] with the following target and lens.
    pub fn new_target<L, G>(target: G, lens: L) -> Self
    where
        L: Interpolator<Item = T::Item> + Send + Sync + 'static,
        G: Into<T>,
    {
        TweenBoxed {
            target: target.into(),
            lens: Box::new(lens),
        }
    }

    /// Create a new [`TweenBoxed`] with the following target and lens as a closure.
    pub fn new_target_map<F, G>(target: G, map: F) -> Self
    where
        F: Fn(&mut T::Item, f32) + Send + Sync + 'static,
        G: Into<T>,
        <T as TweenTarget>::Item: 'static,
    {
        TweenBoxed::new_target(
            target,
            Box::new(map)
                as Box<dyn Fn(&mut T::Item, f32) + Send + Sync + 'static>,
        )
    }
}

#[cfg(feature = "tween_boxed")]
impl<T> TweenBoxed<T>
where
    T: TweenTarget + Default,
{
    /// Create a new [`TweenBoxed`] with the following lens and using the default target.
    pub fn new<L>(lens: L) -> Self
    where
        L: Interpolator<Item = T::Item> + Send + Sync + 'static,
    {
        TweenBoxed::new_target(T::default(), lens)
    }

    /// Create a new [`TweenBoxed`] with the following lens as a closure and using the default target.
    pub fn new_map<F>(map: F) -> Self
    where
        F: Fn(&mut T::Item, f32) + Send + Sync + 'static,
        <T as TweenTarget>::Item: 'static,
    {
        TweenBoxed::new(Box::new(map)
            as Box<dyn Fn(&mut T::Item, f32) + Send + Sync + 'static>)
    }
}

/// Useful for the implmentor to specify what this `target` will return the
/// tweenable [`Self::Item`] which should match any [`Interpolator::Item`].
/// See [`TargetComponent`], [`TargetResource`], and [`TargetAsset`]
pub trait TweenTarget {
    /// Specify the item for tweens
    type Item;
}

/// Convenient alias for [`Tween`] that [`TargetComponent`].
#[cfg(feature = "tween_unboxed")]
pub type ComponentTween<L> =
    Tween<TargetComponent<<L as Interpolator>::Item>, L>;

/// Convenient alias for [`TweenBoxed`] that [`TargetComponent`].
#[cfg(feature = "tween_boxed")]
pub type ComponentTweenBoxed<C> = TweenBoxed<TargetComponent<C>>;

/// Tell the tween what component of what entity to tween.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TargetComponent<C> {
    /// Target the entity that contains this tween's tween player.
    TweenPlayerEntity(#[reflect(ignore)] PhantomData<C>),
    /// Target the parent of this tween's tween_player.
    TweenPlayerParent(#[reflect(ignore)] PhantomData<C>),
    /// Target this entity.
    Entity(Entity, #[reflect(ignore)] PhantomData<C>),
    /// Target these entities.
    Entities(Vec<Entity>, #[reflect(ignore)] PhantomData<C>),
}

impl<C> TargetComponent<C> {
    /// Target the entity that contains this tween's tween player.
    pub fn tween_player_entity() -> TargetComponent<C> {
        TargetComponent::TweenPlayerEntity(PhantomData)
    }
    /// Target the parent of this tween's tween_player.
    pub fn tween_player_parent() -> TargetComponent<C> {
        TargetComponent::TweenPlayerParent(PhantomData)
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

impl<C> TweenTarget for TargetComponent<C> {
    type Item = C;
}

impl<C> Default for TargetComponent<C> {
    fn default() -> Self {
        TargetComponent::tween_player_parent()
    }
}

impl<C> From<Entity> for TargetComponent<C> {
    fn from(value: Entity) -> Self {
        TargetComponent::entity(value)
    }
}

impl<C> FromIterator<Entity> for TargetComponent<C> {
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        TargetComponent::Entities(iter.into_iter().collect(), PhantomData)
    }
}

impl<C, const N: usize> From<[Entity; N]> for TargetComponent<C> {
    fn from(value: [Entity; N]) -> Self {
        TargetComponent::entities(value)
    }
}

impl<C> From<Vec<Entity>> for TargetComponent<C> {
    fn from(value: Vec<Entity>) -> Self {
        TargetComponent::entities(value)
    }
}

impl<C> From<&Vec<Entity>> for TargetComponent<C> {
    fn from(value: &Vec<Entity>) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl<C> From<&[Entity]> for TargetComponent<C> {
    fn from(value: &[Entity]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl<C, const N: usize> From<&[Entity; N]> for TargetComponent<C> {
    fn from(value: &[Entity; N]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

/// Tween any [`ComponentTween`] with value provided by [`TweenInterpolationValue`] component.
#[cfg(feature = "tween_unboxed")]
pub fn component_tween_system<L>(
    q_tween_player: Query<(
        Option<&Parent>,
        Has<crate::tween_player::TweenPlayerState>,
    )>,
    q_tween: Query<(Entity, &ComponentTween<L>, &TweenInterpolationValue)>,
    mut q_component: Query<&mut L::Item>,
) where
    L: Interpolator + Send + Sync + 'static,
    L::Item: Component,
{
    q_tween.iter().for_each(|(entity, tween, ease_value)| {
        let target = match &tween.target {
            TargetComponent::TweenPlayerEntity(_) => {
                match q_tween_player.get(entity) {
                    Ok((_, true)) => entity,
                    Ok((Some(this_parent), false)) => {
                        match q_tween_player.get(this_parent.get()) {
                            Ok((_, true)) => this_parent.get(),
                            _ => return,
                        }
                    }
                    _ => return,
                }
            }
            TargetComponent::TweenPlayerParent(_) => {
                match q_tween_player.get(entity) {
                    Ok((Some(this_parent), true)) => this_parent.get(),
                    Ok((Some(this_parent), false)) => {
                        match q_tween_player.get(this_parent.get()) {
                            Ok((Some(player_parent), true)) => {
                                player_parent.get()
                            }
                            _ => return,
                        }
                    }
                    _ => return,
                }
            }
            TargetComponent::Entity(e, _) => *e,
            TargetComponent::Entities(e, _) => {
                for &target in e {
                    let mut target_component = match q_component.get_mut(target)
                    {
                        Ok(target_component) => target_component,
                        Err(e) => {
                            warn!(
                                "{} query error: {e}",
                                type_name::<ComponentTween<L>>()
                            );
                            continue;
                        }
                    };
                    tween.lens.interpolate(&mut target_component, ease_value.0);
                }
                return;
            }
        };

        let mut target_component = match q_component.get_mut(target) {
            Ok(target_component) => target_component,
            Err(e) => {
                warn!("{} query error: {e}", type_name::<ComponentTween<L>>());
                return;
            }
        };
        tween.lens.interpolate(&mut target_component, ease_value.0);
    })
}

/// Tween any [`ComponentTweenBoxed`] with value provided by [`TweenInterpolationValue`] component.
#[cfg(feature = "tween_boxed")]
pub fn component_tween_boxed_system<C>(
    q_tween_player: Query<(
        Option<&Parent>,
        Has<crate::tween_player::TweenPlayerState>,
    )>,
    q_tween: Query<(Entity, &ComponentTweenBoxed<C>, &TweenInterpolationValue)>,
    mut q_component: Query<&mut C>,
) where
    C: Component,
{
    q_tween.iter().for_each(|(entity, tween, ease_value)| {
        let target = match &tween.target {
            TargetComponent::TweenPlayerEntity(_) => {
                match q_tween_player.get(entity) {
                    Ok((_, true)) => entity,
                    Ok((Some(this_parent), false)) => {
                        match q_tween_player.get(this_parent.get()) {
                            Ok((_, true)) => this_parent.get(),
                            _ => return,
                        }
                    }
                    _ => return,
                }
            }
            TargetComponent::TweenPlayerParent(_) => {
                match q_tween_player.get(entity) {
                    Ok((Some(this_parent), true)) => this_parent.get(),
                    Ok((Some(this_parent), false)) => {
                        match q_tween_player.get(this_parent.get()) {
                            Ok((Some(player_parent), true)) => {
                                player_parent.get()
                            }
                            _ => return,
                        }
                    }
                    _ => return,
                }
            }
            TargetComponent::Entity(e, _) => *e,
            TargetComponent::Entities(e, _) => {
                for &target in e {
                    let mut target_component = match q_component.get_mut(target)
                    {
                        Ok(target_component) => target_component,
                        Err(e) => {
                            warn!(
                                "{} query error: {e}",
                                type_name::<ComponentTweenBoxed<C>>()
                            );
                            continue;
                        }
                    };
                    tween.lens.interpolate(&mut target_component, ease_value.0);
                }
                return;
            }
        };

        let mut target_component = match q_component.get_mut(target) {
            Ok(target_component) => target_component,
            Err(e) => {
                warn!(
                    "{} query error: {e}",
                    type_name::<ComponentTweenBoxed<C>>()
                );
                return;
            }
        };
        tween.lens.interpolate(&mut target_component, ease_value.0);
    })
}

/// Convenient alias for [`Tween`] that [`TargetResource`].
#[cfg(feature = "tween_unboxed")]
pub type ResourceTween<L> = Tween<TargetResource<<L as Interpolator>::Item>, L>;

/// Convenient alias for [`TweenBoxed`] that [`TargetResource`].
#[cfg(feature = "tween_boxed")]
pub type ResourceTweenBoxed<R> = TweenBoxed<TargetResource<R>>;

/// Tell the tween what resource to tween.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct TargetResource<R>(#[reflect(ignore)] pub PhantomData<R>);

impl<R> TargetResource<R> {
    /// New resource target
    pub fn new() -> TargetResource<R> {
        TargetResource(PhantomData)
    }
}

impl<R> TweenTarget for TargetResource<R> {
    type Item = R;
}

/// Tween any [`ResourceTween`] with value provided by [`TweenInterpolationValue`] component.
#[cfg(feature = "tween_unboxed")]
pub fn resource_tween_system<L>(
    q_tween: Query<(&ResourceTween<L>, &TweenInterpolationValue)>,
    resource: Option<ResMut<L::Item>>,
) where
    L: Interpolator + Send + Sync + 'static,
    L::Item: Resource,
{
    let Some(mut resource) = resource else {
        warn!("Resource does not exists for a resource tween.");
        return;
    };
    q_tween.iter().for_each(|(tween, ease_value)| {
        tween.lens.interpolate(&mut resource, ease_value.0);
    })
}

/// Tween any [`ResourceTweenBoxed`] with value provided by [`TweenInterpolationValue`] component.
#[cfg(feature = "tween_boxed")]
pub fn resource_tween_boxed_system<R>(
    q_tween: Query<(&ResourceTweenBoxed<R>, &TweenInterpolationValue)>,
    resource: Option<ResMut<R>>,
) where
    R: Resource,
{
    let Some(mut resource) = resource else {
        warn!("Resource does not exists for a resource tween.");
        return;
    };
    q_tween.iter().for_each(|(tween, ease_value)| {
        tween.lens.interpolate(&mut resource, ease_value.0);
    })
}

/// Convenient alias for [`Tween`] that [`TargetAsset`].
#[cfg(all(feature = "bevy_asset", feature = "tween_unboxed"))]
pub type AssetTween<L> = Tween<TargetAsset<<L as Interpolator>::Item>, L>;

/// Convenient alias for [`TweenBoxed`] that [`TargetAsset`].
#[cfg(all(feature = "bevy_asset", feature = "tween_boxed"))]
pub type AssetTweenBoxed<A> = TweenBoxed<TargetAsset<A>>;

/// Tell the tween what asset of what type to tween.
#[cfg(feature = "bevy_asset")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TargetAsset<A: Asset> {
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

/// Tween any [`AssetTween`] with value provided by [`TweenInterpolationValue`] component.
#[cfg(all(feature = "bevy_asset", feature = "tween_unboxed"))]
pub fn asset_tween_system<L>(
    q_tween: Query<(&AssetTween<L>, &TweenInterpolationValue)>,
    asset: Option<ResMut<Assets<L::Item>>>,
) where
    L: Interpolator + Send + Sync + 'static,
    L::Item: Asset,
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
                tween.lens.interpolate(asset, ease_value.0);
            }
            TargetAsset::Assets(assets) => {
                for a in assets {
                    let Some(a) = asset.get_mut(a) else {
                        warn!("Asset not found for an asset tween");
                        continue;
                    };
                    tween.lens.interpolate(a, ease_value.0);
                }
            }
        })
}

/// Tween any [`AssetTweenBoxed`] with value provided by [`TweenInterpolationValue`] component.
#[cfg(all(feature = "bevy_asset", feature = "tween_boxed"))]
pub fn asset_tween_boxed_system<A>(
    q_tween: Query<(&AssetTweenBoxed<A>, &TweenInterpolationValue)>,
    asset: Option<ResMut<Assets<A>>>,
) where
    A: Asset,
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
                tween.lens.interpolate(asset, ease_value.0);
            }
            TargetAsset::Assets(assets) => {
                for a in assets {
                    let Some(a) = asset.get_mut(a) else {
                        warn!("Asset not found for an asset tween");
                        continue;
                    };
                    tween.lens.interpolate(a, ease_value.0);
                }
            }
        })
}
