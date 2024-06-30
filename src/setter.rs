mod blanket_impl;

#[cfg(feature = "bevy_sprite")]
mod sprite {
    use super::Setter;
    use bevy::prelude::*;

    #[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
    #[reflect(Component)]
    pub struct SpriteColor;

    impl Setter<Sprite, Color> for SpriteColor {
        fn set(&self, item: &mut Sprite, value: &Color) {
            item.color = *value;
        }
    }

    fn plugin(app: &mut App) {
        app.add_systems(
            Update,
            super::apply_component_tween_system::<SpriteColor, _, _>,
        );
    }

    #[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
    #[reflect(Component)]
    pub struct ColorMaterial;

    impl Setter<bevy::prelude::ColorMaterial, Color> for ColorMaterial {
        fn set(&self, item: &mut bevy::prelude::ColorMaterial, value: &Color) {
            item.color = *value;
        }
    }
}

use crate::{
    curve::CurveValue,
    tween::{SkipTween, TargetAsset, TargetComponent, TargetResource},
};
use bevy::{
    ecs::query::QueryEntityError,
    prelude::*,
    utils::{HashMap, HashSet},
};
use std::any::type_name;

#[cfg(feature = "bevy_sprite")]
pub use sprite::*;

pub trait Setter<Item, Value>: Send + Sync + 'static {
    fn set(&self, item: &mut Item, value: &Value);
}

pub fn apply_component_tween_system<S, C, V>(
    q_tween: Query<
        (Entity, &TargetComponent, &S, &CurveValue<V>),
        Without<SkipTween>,
    >,
    mut q_component: Query<&mut C>,
    mut last_entity_errors: Local<HashMap<Entity, QueryEntityError>>,
) where
    S: Setter<C, V> + Component,
    C: Component,
    V: Send + Sync + 'static,
{
    let mut query_entity_errors = HashMap::new();
    q_tween.iter().for_each(
        |(tween_entity, target_data, setter, curve_value)| match target_data {
            TargetComponent::Entity(e) => match q_component.get_mut(*e) {
                Ok(mut component) => setter.set(&mut component, &curve_value.0),
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
                            type_name::<C>(),
                            query_error
                        );
                    }
                    query_entity_errors.insert(tween_entity, query_error);
                }
            },
            TargetComponent::Entities(e) => {
                let mut iter = q_component.iter_many_mut(e);
                while let Some(mut component) = iter.fetch_next() {
                    setter.set(&mut component, &curve_value.0);
                }
            }
            TargetComponent::None => {}
            TargetComponent::Marker => panic!("remove this variant later"),
        },
    );
    *last_entity_errors = query_entity_errors;
}

pub fn apply_resource_tween_system<S, R, V>(
    q_tween: Query<
        (&S, &CurveValue<V>),
        (With<TargetResource>, Without<SkipTween>),
    >,
    resource: Option<ResMut<R>>,
    mut last_error: Local<bool>,
) where
    S: Setter<R, V> + Component,
    R: Resource,
    V: Send + Sync + 'static,
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
        setter.set(&mut resource, &curve_value.0);
    })
}

pub fn apply_asset_tween_system<S, A, V>(
    q_tween: Query<(&S, &TargetAsset<A>, &CurveValue<V>), Without<SkipTween>>,
    asset: Option<ResMut<Assets<A>>>,
    mut last_resource_error: Local<bool>,
    mut last_asset_errors: Local<HashSet<AssetId<A>>>,
) where
    S: Setter<A, V> + Component,
    A: Asset,
    V: Send + Sync + 'static,
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
                            type_name::<A>(),
                            handle.id()
                        );
                    }
                    asset_errors.insert(handle.id());
                    return;
                };
                setter.set(asset, &curve_value.0);
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
                            type_name::<A>(),
                            handle.id()
                        );
                    }
                    asset_errors.insert(handle.id());
                    return;
                };
                setter.set(asset, &curve_value.0);
                }
            }
        });

    *last_asset_errors = asset_errors;
}

pub fn apply_handle_component_tween_system<S, A, V>(
    q_tween: Query<
        (Entity, &S, &TargetComponent, &CurveValue<V>),
        Without<SkipTween>,
    >,
    q_handle: Query<&Handle<A>>,
    asset: Option<ResMut<Assets<A>>>,
    mut last_resource_error: Local<bool>,
    mut last_asset_errors: Local<HashSet<AssetId<A>>>,
    mut last_entity_errors: Local<HashMap<Entity, QueryEntityError>>,
) where
    S: Setter<A, V> + Component,
    A: Asset,
    V: Send + Sync + 'static,
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
            TargetComponent::Marker => panic!("remove this variant later"),
            TargetComponent::Entity(entity) => match q_handle.get(*entity) {
                Ok(handle) => {
                    let Some(asset) = asset.get_mut(handle) else {
                        if !last_asset_errors.contains(&handle.id())
                            && !asset_errors.contains(&handle.id())
                        {
                            error!(
                                "{} attempted to tween {} asset {} but it does not exists",
                                type_name::<S>(),
                                type_name::<A>(),
                                handle.id()
                            );
                        }
                        asset_errors.insert(handle.id());
                        return;
                    };
                    setter.set(asset, &curve_value.0);
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
                            type_name::<A>(),
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
                                type_name::<A>(),
                                handle.id()
                            );
                        }
                        asset_errors.insert(handle.id());
                        return;
                    };
                    setter.set(asset, &curve_value.0);
                }
            }
        } );

    *last_asset_errors = asset_errors;
    *last_entity_errors = query_entity_errors;
}
