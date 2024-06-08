use super::*;
use bevy::{
    ecs::{query::QueryEntityError, schedule::SystemConfigs},
    utils::{HashMap, HashSet},
};
use bevy_time_runner::TimeSpanProgress;
use std::any::type_name;

/// Alias for [`apply_component_tween_system`] and may contains more systems
/// in the future.
pub fn component_tween_system<I>() -> SystemConfigs
where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Component,
{
    apply_component_tween_system::<I>.into_configs()
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetComponent`] with
/// value provided by [`TweenInterpolationValue`] component.
///
/// The system uses generic with the trait [`Interpolator`] for you to quickly
/// make your interpolators work. The trait is only necessary to be used with
/// this built-in system.
///
/// # Examples
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
///     // You might want to use `bevy_tween::component_tween_system` instead.
///
///     // Generic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_component_tween_system::<InterpolateSize>,
///     );
///
///     // Dynamic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_component_tween_system::<
///             BoxedInterpolator<Size>,
///         >,
///     );
/// }
/// ```
#[allow(clippy::type_complexity)]
pub fn apply_component_tween_system<I>(
    q_animation_target: Query<(Option<&Parent>, Has<AnimationTarget>)>,
    q_tween: Query<
        (Entity, &Tween<TargetComponent, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
    mut q_component: Query<&mut I::Item>,
    mut last_entity_error: Local<HashMap<Entity, QueryEntityError>>,
    mut last_search_error: Local<HashSet<Entity>>,
) where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Component,
{
    let mut entity_error = HashMap::new();
    let mut search_error = HashSet::new();
    q_tween
        .iter()
        .for_each(|(entity, tween, ease_value)| match &tween.target {
            TargetComponent::Entities(e) => {
                e.iter().for_each(|target| {
                    let mut target_component =
                        match q_component.get_mut(*target) {
                            Ok(target_component) => target_component,
                            Err(e) => {
                                if last_entity_error
                                    .get(target)
                                    .map(|old_e| old_e != &e)
                                    .unwrap_or(true)
                                    && entity_error
                                        .get(target)
                                        .map(|old_e| old_e != &e)
                                        .unwrap_or(true)
                                {
                                    error!(
                                        "{} attempted to tween {} component but got query error: {e}",
                                        type_name::<I>(),
                                        type_name::<I::Item>()
                                    );
                                }
                                entity_error.insert(*target, e);
                                return;
                            }
                        };
                    tween
                        .interpolator
                        .interpolate(&mut target_component, ease_value.0);
                });
            }
            _ => {
                let target = match &tween.target {
                    TargetComponent::Marker => {
                        let mut curr = entity;
                        let found = 'l: loop {
                            match q_animation_target.get(curr) {
                                Ok((parent, has_marker)) => {
                                    if has_marker {
                                        break 'l Some(curr);
                                    } else {
                                        match parent {
                                            Some(parent) => curr = parent.get(),
                                            None => break 'l None,
                                        }
                                    }
                                }
                                _ => break 'l None,
                            }
                        };
                        match found {
                            Some(found) => found,
                            None => {
                                if !last_search_error.contains(&entity) && !search_error.contains(&entity) {
                                    error!(
                                        "Tween {:?} {} cannot find AnimationTarget marker",
                                        entity,
                                        type_name::<I>(),
                                    );
                                }
                                search_error.insert(entity);
                                return;
                            },
                        }
                    }
                    TargetComponent::Entity(e) => *e,
                    _ => unreachable!(),
                };

                let mut target_component = match q_component.get_mut(target) {
                    Ok(target_component) => target_component,
                    Err(e) => {
                        if last_entity_error
                            .get(&target)
                            .map(|old_e| old_e != &e)
                            .unwrap_or(true)
                            && entity_error
                                .get(&target)
                                .map(|old_e| old_e != &e)
                                .unwrap_or(true)
                        {
                            error!(
                                "{} attempted to tween {} component but got query error: {e}",
                                type_name::<I>(),
                                type_name::<I::Item>()
                            );
                        }
                        entity_error.insert(target, e);
                        return;
                    }
                };
                tween
                    .interpolator
                    .interpolate(&mut target_component, ease_value.0);
            }
        });
    *last_entity_error = entity_error;
    *last_search_error = search_error;
}

/// System alias for [`component_tween_system`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
///
/// This currently exists for backward compatibility and there's not really any big reason to deprecate it just yet.
/// You might want to use `component_tween_system::<BoxedInterpolator<...>>()` for consistency
pub fn component_dyn_tween_system<C>() -> SystemConfigs
where
    C: Component,
{
    apply_component_tween_system::<Box<dyn Interpolator<Item = C>>>
        .into_configs()
}

/// Alias for [`apply_resource_tween_system`] and may contains more systems
/// in the future.
pub fn resource_tween_system<I>() -> SystemConfigs
where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Resource,
{
    apply_resource_tween_system::<I>.into_configs()
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetResource`] with
/// value provided by [`TweenInterpolationValue`] component.
///
/// The system uses generic with the trait [`Interpolator`] for you to quickly
/// make your interpolators work. The trait is only necessary to be used with
/// this built-in system.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Resource)]
/// struct ScreenFade(f32);
///
/// struct InterpolateScreenFade {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateScreenFade {
///     type Item = ScreenFade;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
///
/// fn main() {
///     let mut app = App::new();
///
///     // You might want to use `bevy_tween::resource_tween_system` instead.
///
///     // Generic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_resource_tween_system::<
///             InterpolateScreenFade,
///         >,
///     );
///
///     // Dynamic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_resource_tween_system::<
///             BoxedInterpolator<ScreenFade>,
///         >,
///     );
/// }
/// ```
#[allow(clippy::type_complexity)]
pub fn apply_resource_tween_system<I>(
    q_tween: Query<
        (&Tween<TargetResource, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
    resource: Option<ResMut<I::Item>>,
    mut last_error: Local<bool>,
) where
    I: Interpolator,
    I::Item: Resource,
{
    let Some(mut resource) = resource else {
        if !*last_error {
            error!(
                "{} resource tween system cannot find the resource",
                type_name::<I>()
            );
            *last_error = true;
        }
        return;
    };
    *last_error = false;
    q_tween.iter().for_each(|(tween, ease_value)| {
        tween.interpolator.interpolate(&mut resource, ease_value.0);
    })
}

/// System alias for [`apply_resource_tween_system`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
///
/// This currently exists for backward compatibility and there's not really any big reason to deprecate it just yet.
/// You might want to use `resource_tween_system::<BoxedInterpolator<...>>()` for consistency
pub fn resource_dyn_tween_system<R>() -> SystemConfigs
where
    R: Resource,
{
    apply_resource_tween_system::<Box<dyn Interpolator<Item = R>>>
        .into_configs()
}

/// Alias for [`apply_asset_tween_system`] and may contains more systems
/// in the future.
#[cfg(feature = "bevy_asset")]
pub fn asset_tween_system<I>() -> SystemConfigs
where
    I: Interpolator + Send + Sync + 'static,
    I::Item: Asset,
{
    apply_asset_tween_system::<I>.into_configs()
}

/// Apply any [`Tween`] with the [`Interpolator`] that [`TargetAsset`] with
/// value provided by [`TweenInterpolationValue`] component.
///
/// The system uses generic with the trait [`Interpolator`] for you to quickly
/// make your interpolators work. The trait is only necessary to be used with
/// this built-in system.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_tween::prelude::*;
///
/// #[derive(Asset, TypePath)]
/// struct Rainbow(f32);
///
/// struct InterpolateRainbow {
///     start: f32,
///     end: f32,
/// }
///
/// impl Interpolator for InterpolateRainbow {
///     type Item = Rainbow;
///
///     fn interpolate(&self, item: &mut Self::Item, value: f32) {
///         item.0 = self.start.lerp(self.end, value);
///     }
/// }
///
/// fn main() {
///     let mut app = App::new();
///
///     // You might want to use `bevy_tween::resource_tween_system` instead.
///
///     // Generic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_asset_tween_system::<InterpolateRainbow>,
///     );
///
///     // Dynamic interpolator:
///     app.add_tween_systems(
///         bevy_tween::tween::apply_asset_tween_system::<
///             BoxedInterpolator<Rainbow>,
///         >,
///     );
/// }
/// ```
#[cfg(feature = "bevy_asset")]
#[allow(clippy::type_complexity)]
pub fn apply_asset_tween_system<I>(
    q_tween: Query<
        (&Tween<TargetAsset<I::Item>, I>, &TweenInterpolationValue),
        Without<SkipTween>,
    >,
    asset: Option<ResMut<Assets<I::Item>>>,
    mut last_resource_error: Local<bool>,
    mut last_asset_error: Local<HashSet<AssetId<I::Item>>>,
) where
    I: Interpolator,
    I::Item: Asset,
{
    let mut asset_error = HashSet::new();

    let Some(mut asset) = asset else {
        if !*last_resource_error {
            error!(
                "{} asset tween system cannot find the asset resource",
                type_name::<I>()
            );
            *last_resource_error = true;
        }
        return;
    };
    *last_resource_error = false;
    q_tween
        .iter()
        .for_each(|(tween, ease_value)| match &tween.target {
            TargetAsset::Asset(a) => {
                let Some(asset) = asset.get_mut(a) else {
                    if !last_asset_error.contains(&a.id())
                        && !asset_error.contains(&a.id())
                    {
                        error!(
                            "{} attempted to tween {} asset {} but it does not exists",
                                type_name::<I>(),
                            type_name::<I::Item>(),
                            a.id()
                        );
                    }
                    asset_error.insert(a.id());
                    return;
                };
                tween.interpolator.interpolate(asset, ease_value.0);
            }
            TargetAsset::Assets(assets) => {
                for a in assets {
                    let Some(a) = asset.get_mut(a) else {
                        if !last_asset_error.contains(&a.id())
                            && !asset_error.contains(&a.id())
                        {
                            error!(
                                "{} attempted to tween {} asset {} but it does not exists",
                                type_name::<I>(),
                                type_name::<I::Item>(),
                                a.id()
                            );
                        }
                        asset_error.insert(a.id());
                        continue;
                    };
                    tween.interpolator.interpolate(a, ease_value.0);
                }
            }
        });

    *last_asset_error = asset_error;
}

/// System alias for [`apply_asset_tween_system`] that uses boxed dynamic [`Interpolator`]. (`Box<dyn Interpolator`)
///
/// This currently exists for backward compatibility and there's not really any big reason to deprecate it just yet.
/// You might want to use `asset_tween_system::<BoxedInterpolator<...>>()` for consistency
#[cfg(feature = "bevy_asset")]
pub fn asset_dyn_tween_system<A>() -> SystemConfigs
where
    A: Asset,
{
    apply_asset_tween_system::<Box<dyn Interpolator<Item = A>>>.into_configs()
}

/// Fires [`TweenEvent`] with optional user data whenever [`TimeSpanProgress`]
/// and [`TweenEventData`] exist in the same entity and data is `Some`,
/// cloning the data.
#[allow(clippy::type_complexity)]
pub fn tween_event_system<Data>(
    q_tween_event_data: Query<
        (
            Entity,
            &TweenEventData<Data>,
            &TimeSpanProgress,
            Option<&TweenInterpolationValue>,
        ),
        Without<SkipTween>,
    >,
    mut event_writer: EventWriter<TweenEvent<Data>>,
) where
    Data: Clone + Send + Sync + 'static,
{
    q_tween_event_data.iter().for_each(
        |(entity, event_data, progress, interpolation_value)| {
            if let Some(data) = event_data.0.as_ref() {
                event_writer.send(TweenEvent {
                    data: data.clone(),
                    progress: *progress,
                    interpolation_value: interpolation_value.map(|v| v.0),
                    entity,
                });
            }
        },
    );
}

/// Fires [`TweenEvent`] with optional user data whenever [`TimeSpanProgress`]
/// and [`TweenEventData`] exist in the same entity and data is `Some`,
/// taking the data and leaves the value `None`.
#[allow(clippy::type_complexity)]
pub fn tween_event_taking_system<Data>(
    mut q_tween_event_data: Query<
        (
            Entity,
            &mut TweenEventData<Data>,
            &TimeSpanProgress,
            Option<&TweenInterpolationValue>,
        ),
        Without<SkipTween>,
    >,
    mut event_writer: EventWriter<TweenEvent<Data>>,
) where
    Data: Send + Sync + 'static,
{
    q_tween_event_data.iter_mut().for_each(
        |(entity, mut event_data, progress, interpolation_value)| {
            if let Some(data) = event_data.0.take() {
                event_writer.send(TweenEvent {
                    data,
                    progress: *progress,
                    interpolation_value: interpolation_value.map(|v| v.0),
                    entity,
                });
            }
        },
    );
}
