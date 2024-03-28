use super::*;
use bevy::ecs::schedule::SystemConfigs;
use std::any::type_name;

/// Tween any [`Tween`] with the [`Interpolator`] that [`TargetComponent`] with
/// value provided by [`TweenInterpolationValue`] component.
#[allow(clippy::type_complexity)]
#[deprecated(
    since = "0.3.0",
    note = "Use `component_tween_system` instead with less required generic"
)]
pub fn component_tween_system_full<C, I>(
    q_tweener: Query<(Option<&Parent>, Has<TweenerMarker>)>,
    q_tween: Query<
        (Entity, &Tween<TargetComponent, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
    q_component: Query<&mut I::Item>,
) where
    C: Component,
    I: Interpolator<Item = C> + Send + Sync + 'static,
{
    component_tween_system(q_tweener, q_tween, q_component);
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetComponent`] with
/// value provided by [`TweenInterpolationValue`] component.
///
/// The system uses generic for you to quickly make your interpolators work.
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Component)]
/// struct Size(f32);
///
/// struct InterpolateSize {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateSize {
///     type Item = Size;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
///
/// fn main() {
///     let mut app = App::new();
///
///     // Generic interpolator:
///     app.add_tween_systems(
///         bevy_tween::component_tween_system::<InterpolateSize>,
///     );
///
///     // Dynamic interpolator:
///     app.add_tween_systems(
///         bevy_tween::component_tween_system::<BoxedInterpolator<Size>>,
///     );
/// }
/// ```
#[allow(clippy::type_complexity)]
pub fn component_tween_system<I>(
    q_tweener: Query<(Option<&Parent>, Has<TweenerMarker>)>,
    q_tween: Query<
        (Entity, &Tween<TargetComponent, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
    mut q_component: Query<&mut I::Item>,
) where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Component,
{
    q_tween.iter().for_each(|(entity, tween, ease_value)| {
        let target = match &tween.target {
            TargetComponent::TweenerEntity => match q_tweener.get(entity) {
                Ok((_, true)) => entity,
                Ok((Some(this_parent), false)) => {
                    match q_tweener.get(this_parent.get()) {
                        Ok((_, true)) => this_parent.get(),
                        _ => return,
                    }
                }
                _ => return,
            },
            TargetComponent::TweenerParent => match q_tweener.get(entity) {
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
            TargetComponent::Entity(e) => *e,
            TargetComponent::Entities(e) => {
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

/// Tween any [`Tween`] with the [`Interpolator`] that [`TargetResource`] with
/// value provided by [`TweenInterpolationValue`] component.
#[deprecated(
    since = "0.3.0",
    note = "Use `resource_tween_system` instead with less required generic"
)]
#[allow(clippy::type_complexity)]
pub fn resource_tween_system_full<R, I>(
    q_tween: Query<
        (&Tween<TargetResource, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
    resource: Option<ResMut<I::Item>>,
) where
    R: Resource,
    I: Interpolator<Item = R> + Send + Sync + 'static,
{
    resource_tween_system(q_tween, resource);
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetResource`] with
/// value provided by [`TweenInterpolationValue`] component.
#[allow(clippy::type_complexity)]
pub fn resource_tween_system<I>(
    q_tween: Query<
        (&Tween<TargetResource, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
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

/// Tween any [`Tween`] with the [`Interpolator`] that [`TargetAsset`] with
/// value provided by [`TweenInterpolationValue`] component.
#[cfg(feature = "bevy_asset")]
#[deprecated(
    since = "0.3.0",
    note = "Use `asset_tween_system` instead with less required generic"
)]
#[allow(clippy::type_complexity)]
pub fn asset_tween_system_full<A, I>(
    q_tween: Query<
        (&Tween<TargetAsset<A>, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
    asset: Option<ResMut<Assets<I::Item>>>,
) where
    A: Asset,
    I: Interpolator<Item = A> + Send + Sync + 'static,
{
    asset_tween_system(q_tween, asset);
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetAsset`] with
/// value provided by [`TweenInterpolationValue`] component.
#[cfg(feature = "bevy_asset")]
#[allow(clippy::type_complexity)]
pub fn asset_tween_system<I>(
    q_tween: Query<
        (&Tween<TargetAsset<I::Item>, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
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
