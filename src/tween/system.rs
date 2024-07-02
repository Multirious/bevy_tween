use crate::items::{Set, Setter};
use crate::{
    curve::CurveValue,
    targets::{TargetAsset, TargetComponent, TargetResource},
    tween::SkipTween,
};
use bevy::{
    ecs::query::QueryEntityError,
    prelude::*,
    utils::{HashMap, HashSet},
};
use std::any::type_name;

pub fn apply_component_tween_system<S>(
    q_tween: Query<
        (Entity, &TargetComponent, &Setter<S>, &CurveValue<S::Value>),
        Without<SkipTween>,
    >,
    mut q_component: Query<&mut S::Item>,
    mut last_entity_errors: Local<HashMap<Entity, QueryEntityError>>,
) where
    S: Set,
    S::Item: Component,
    S::Value: Send + Sync + 'static,
{
    let mut query_entity_errors = HashMap::new();
    q_tween.iter().for_each(
        |(tween_entity, target_data, setter, curve_value)| match target_data {
            TargetComponent::None => {}
            TargetComponent::Entity(e) => match q_component.get_mut(*e) {
                Ok(mut component) => {
                    setter.0.set(&mut component, &curve_value.0)
                }
                Err(query_error) => {
                    if last_entity_errors
                        .get(&tween_entity)
                        .map(|last_error| last_error != &query_error)
                        .unwrap_or(true)
                        && query_entity_errors
                            .get(&tween_entity)
                            .map(|last_error| last_error != &query_error)
                            .unwrap_or(true)
                    {
                        error!(
                            "{} attempted to mutate {} but got error: {}",
                            type_name::<S>(),
                            type_name::<S::Item>(),
                            query_error
                        );
                    }
                    query_entity_errors.insert(tween_entity, query_error);
                }
            },
            TargetComponent::Entities(e) => {
                let mut iter = q_component.iter_many_mut(e);
                while let Some(mut component) = iter.fetch_next() {
                    setter.0.set(&mut component, &curve_value.0);
                }
            }
        },
    );
    *last_entity_errors = query_entity_errors;
}

pub fn apply_resource_tween_system<S>(
    q_tween: Query<
        (&Setter<S>, &CurveValue<S::Value>),
        (With<TargetResource>, Without<SkipTween>),
    >,
    resource: Option<ResMut<S::Item>>,
    mut last_error: Local<bool>,
) where
    S: Set,
    S::Item: Resource,
    S::Value: Send + Sync + 'static,
{
    let Some(mut resource) = resource else {
        if !*last_error {
            error!(
                "{} apply_resource_tween_system cannot find the resource",
                type_name::<S>()
            );
            *last_error = true;
        }
        return;
    };
    *last_error = false;
    q_tween.iter().for_each(|(setter, curve_value)| {
        setter.0.set(&mut resource, &curve_value.0);
    })
}

pub fn apply_asset_tween_system<S>(
    q_tween: Query<
        (&Setter<S>, &TargetAsset<S::Item>, &CurveValue<S::Value>),
        Without<SkipTween>,
    >,
    asset: Option<ResMut<Assets<S::Item>>>,
    mut last_resource_error: Local<bool>,
    mut last_asset_errors: Local<HashSet<AssetId<S::Item>>>,
) where
    S: Set,
    S::Item: Asset,
    S::Value: Send + Sync + 'static,
{
    let mut asset_errors = HashSet::new();

    let Some(mut asset) = asset else {
        if !*last_resource_error {
            error!(
                "{} apply_asset_tween_system cannot find the asset resource",
                type_name::<S>()
            );
            *last_resource_error = true;
        }
        return;
    };
    *last_resource_error = false;
    q_tween
        .iter()
        .for_each(|(setter, target, curve_value)| match &target {
            TargetAsset::None => {},
            TargetAsset::Asset(handle) => {
                let Some(asset) = asset.get_mut(handle) else {
                    if !last_asset_errors.contains(&handle.id())
                        && !asset_errors.contains(&handle.id())
                    {
                        error!(
                            "{} attempted to tween {} asset {} but it does not exists",
                            type_name::<S>(),
                            type_name::<S::Item>(),
                            handle.id()
                        );
                    }
                    asset_errors.insert(handle.id());
                    return;
                };
                setter.0.set(asset, &curve_value.0);
            }
            TargetAsset::Assets(handles) => {
                for handle in handles {
                let Some(asset) = asset.get_mut(handle) else {
                    if !last_asset_errors.contains(&handle.id())
                        && !asset_errors.contains(&handle.id())
                    {
                        error!(
                            "{} attempted to tween {} asset {} but it does not exists",
                            type_name::<S>(),
                            type_name::<S::Item>(),
                            handle.id()
                        );
                    }
                    asset_errors.insert(handle.id());
                    return;
                };
                setter.0.set(asset, &curve_value.0);
                }
            }
        });

    *last_asset_errors = asset_errors;
}

pub fn apply_handle_component_tween_system<S>(
    q_tween: Query<
        (Entity, &Setter<S>, &TargetComponent, &CurveValue<S::Value>),
        Without<SkipTween>,
    >,
    q_handle: Query<&Handle<S::Item>>,
    asset: Option<ResMut<Assets<S::Item>>>,
    mut last_resource_error: Local<bool>,
    mut last_asset_errors: Local<HashSet<AssetId<S::Item>>>,
    mut last_entity_errors: Local<HashMap<Entity, QueryEntityError>>,
) where
    S: Set,
    S::Item: Asset,
    S::Value: Send + Sync + 'static,
{
    let mut asset_errors = HashSet::new();
    let mut query_entity_errors = HashMap::new();

    let Some(mut asset) = asset else {
        if !*last_resource_error {
            error!(
                "{} apply_handle_component_tween_system cannot find the asset resource",
                type_name::<S>()
            );
            *last_resource_error = true;
        }
        return;
    };
    *last_resource_error = false;
    q_tween
        .iter()
        .for_each(|(tween_entity, setter, target, curve_value)| match &target {
            TargetComponent::None => {},
            TargetComponent::Entity(entity) => match q_handle.get(*entity) {
                Ok(handle) => {
                    let Some(asset) = asset.get_mut(handle) else {
                        if !last_asset_errors.contains(&handle.id())
                            && !asset_errors.contains(&handle.id())
                        {
                            error!(
                                "{} attempted to tween {} asset {} but it does not exists",
                                type_name::<S>(),
                                type_name::<S::Item>(),
                                handle.id()
                            );
                        }
                        asset_errors.insert(handle.id());
                        return;
                    };
                    setter.0.set(asset, &curve_value.0);
                },
                Err(query_error) => {
                    if last_entity_errors
                        .get(&tween_entity)
                        .map(|last_error| last_error != &query_error)
                        .unwrap_or(true)
                        && query_entity_errors
                            .get(&tween_entity)
                            .map(|last_error| last_error != &query_error)
                            .unwrap_or(true)
                    {
                        error!(
                            "{} attempted to query for Handle<{}> but got error: {}",
                            type_name::<S>(),
                            type_name::<S::Item>(),
                            query_error
                        );
                    }
                    query_entity_errors.insert(tween_entity, query_error);
                }
            },
            TargetComponent::Entities(e) => {
                let mut iter = q_handle.iter_many(e);
                while let Some(handle) = iter.fetch_next() {
                    let Some(asset) = asset.get_mut(handle) else {
                        if !last_asset_errors.contains(&handle.id())
                            && !asset_errors.contains(&handle.id())
                        {
                            error!(
                                "{} attempted to tween {} asset {} but it does not exists",
                                type_name::<S>(),
                                type_name::<S::Item>(),
                                handle.id()
                            );
                        }
                        asset_errors.insert(handle.id());
                        return;
                    };
                    setter.0.set(asset, &curve_value.0);
                }
            }
        } );

    *last_asset_errors = asset_errors;
    *last_entity_errors = query_entity_errors;
}
