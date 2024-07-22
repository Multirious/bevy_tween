use std::sync::Arc;

use bevy::prelude::*;
use bevy_time_runner::TimeSpanProgress;

use crate::{
    targets::{TargetAsset, TargetComponent, TargetResource},
    SkipTween, TweenAppResource, TweenSystemSet,
};

use super::SetterValue;

pub struct DynamicSetterPlugin;

impl Plugin for DynamicSetterPlugin {
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<TweenAppResource>()
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            dynamic_setter_system.in_set(TweenSystemSet::Apply),
        );
    }
}

#[derive(Component, Clone)]
pub struct DynamicSetter(_DynamicSetter);

#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub(crate) enum _DynamicSetter {
    Custom(Arc<dyn Fn(Entity, &mut World) + 'static + Send + Sync>),
}

impl DynamicSetter {
    pub fn new<F>(setter: F) -> DynamicSetter
    where
        F: Fn(Entity, &mut World) + 'static + Send + Sync,
    {
        DynamicSetter(_DynamicSetter::Custom(Arc::new(setter)))
    }

    pub fn component<F, C, V>(set: F) -> DynamicSetter
    where
        F: Send + Sync + 'static + Fn(&mut C, &V),
        C: Component,
        V: Send + Sync + 'static + Clone,
    {
        DynamicSetter::new(move |tween_entity, world| {
            let Some(target_entity) =
                world.get::<TargetComponent>(tween_entity)
            else {
                return;
            };

            match target_entity {
                TargetComponent::None => {}
                TargetComponent::Entity(entity) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0.clone();

                    let Some(mut component) = world.get_mut::<C>(*entity)
                    else {
                        return;
                    };
                    set(&mut component, &value);
                }
                TargetComponent::Entities(entities) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0.clone();

                    let entities = entities.clone();
                    for entity in entities {
                        let Some(mut component) = world.get_mut::<C>(entity)
                        else {
                            return;
                        };
                        set(&mut component, &value);
                    }
                }
            }
        })
    }

    pub fn asset<F, A, V>(set: F) -> DynamicSetter
    where
        F: Send + Sync + 'static + Fn(&mut A, &V),
        A: Asset,
        V: Send + Sync + 'static + Clone,
    {
        DynamicSetter::new(move |tween_entity, world| {
            let Some(target_asset) = world.get::<TargetAsset<A>>(tween_entity)
            else {
                return;
            };

            match target_asset {
                TargetAsset::None => {}
                TargetAsset::Asset(handle) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0.clone();

                    let handle = handle.clone();
                    let Some(mut assets) =
                        world.get_resource_mut::<Assets<A>>()
                    else {
                        return;
                    };
                    let Some(asset) = assets.get_mut(&handle) else {
                        return;
                    };
                    set(asset, &value);
                }
                TargetAsset::Assets(handles) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0.clone();

                    let handles = handles.clone();
                    let Some(mut assets) =
                        world.get_resource_mut::<Assets<A>>()
                    else {
                        return;
                    };
                    for handle in handles {
                        let Some(asset) = assets.get_mut(&handle) else {
                            return;
                        };
                        set(asset, &value);
                    }
                }
            }
        })
    }

    pub fn resource<F, R, V>(set: F) -> DynamicSetter
    where
        F: Send + Sync + 'static + Fn(&mut R, &V),
        R: Resource,
        V: Send + Sync + 'static + Clone,
    {
        DynamicSetter::new(move |tween_entity, world| {
            let Some(_target_resource) =
                world.get::<TargetResource>(tween_entity)
            else {
                return;
            };

            let Some(value) = world.get::<SetterValue<V>>(tween_entity) else {
                return;
            };
            let value = value.0.clone();

            let Some(mut resource) = world.get_resource_mut::<R>() else {
                return;
            };
            set(&mut resource, &value);
        })
    }

    pub fn component_handle<FH, FP, C, A, V>(
        select_handle: FH,
        set: FP,
    ) -> DynamicSetter
    where
        FH: Send + Sync + 'static + Fn(&C) -> &Handle<A>,
        FP: Send + Sync + 'static + Fn(&mut A, &V),
        C: Component,
        A: Asset,
        V: Send + Sync + 'static + Clone,
    {
        DynamicSetter::new(move |tween_entity, world| {
            let Some(target_entity) =
                world.get::<TargetComponent>(tween_entity)
            else {
                return;
            };

            match target_entity {
                TargetComponent::None => {}
                TargetComponent::Entity(entity) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0.clone();

                    let Some(component) = world.get::<C>(*entity) else {
                        return;
                    };
                    let handle = select_handle(component).clone();

                    let Some(mut assets_res) =
                        world.get_resource_mut::<Assets<A>>()
                    else {
                        return;
                    };
                    let Some(asset) = assets_res.get_mut(&handle) else {
                        return;
                    };

                    set(asset, &value);
                }
                TargetComponent::Entities(entities) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0.clone();

                    let entities = entities.clone();
                    for entity in entities {
                        let Some(component) = world.get::<C>(entity) else {
                            return;
                        };
                        let handle = select_handle(component).clone();

                        let Some(mut assets_res) =
                            world.get_resource_mut::<Assets<A>>()
                        else {
                            return;
                        };

                        let Some(asset) = assets_res.get_mut(&handle) else {
                            return;
                        };

                        set(asset, &value)
                    }
                }
            }
        })
    }
}

fn dynamic_setter_system(world: &mut World) {
    let mut query = world.query_filtered::<Entity, (
        With<DynamicSetter>,
        Without<SkipTween>,
        With<TimeSpanProgress>,
    )>();
    let entities = query.iter(world).collect::<Vec<_>>();
    for entity in entities {
        let Some(set_reflect) = world.get::<DynamicSetter>(entity) else {
            return;
        };
        match &set_reflect.0 {
            _DynamicSetter::Custom(set) => {
                let set = set.clone();
                set(entity, world);
            }
        }
    }
}
