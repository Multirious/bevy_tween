use std::{any::TypeId, sync::Arc};

use bevy::{prelude::*, reflect::ParsedPath};
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
    Reflect {
        path: ParsedPath,
        component_type: TypeId,
        setter_value_type: TypeId,
    },
}

impl DynamicSetter {
    pub fn new<F>(setter: F) -> DynamicSetter
    where
        F: Fn(Entity, &mut World) + 'static + Send + Sync,
    {
        DynamicSetter(_DynamicSetter::Custom(Arc::new(setter)))
    }

    pub fn component_path(
        path: ParsedPath,
        component_type: TypeId,
        setter_value_type: TypeId,
    ) -> DynamicSetter {
        DynamicSetter(_DynamicSetter::Reflect {
            path,
            component_type,
            setter_value_type,
        })
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
    for tween_entity in entities {
        let Some(set_reflect) = world.get::<DynamicSetter>(tween_entity) else {
            return;
        };
        match &set_reflect.0 {
            _DynamicSetter::Custom(set) => {
                let set = set.clone();
                set(tween_entity, world);
            }
            _DynamicSetter::Reflect {
                path,
                component_type,
                setter_value_type,
            } => {
                let Some(target) = world.get::<TargetComponent>(tween_entity)
                else {
                    continue;
                };
                match target {
                    TargetComponent::None => continue,
                    TargetComponent::Entity(target_entity) => {
                        let Some(type_registry) =
                            world.get_resource::<AppTypeRegistry>()
                        else {
                            continue;
                        };
                        let type_registry = type_registry.read();
                        let Some(component) = type_registry
                            .get_type_data::<ReflectComponent>(*component_type)
                        else {
                            continue;
                        };
                        let component = component.clone();

                        let Some(setter_value) = type_registry
                            .get_type_data::<ReflectComponent>(
                                *setter_value_type,
                            )
                        else {
                            continue;
                        };
                        let setter_value_component = setter_value.clone();

                        drop(type_registry);
                        let path = path.clone();

                        let Some(tween) = world.get_entity(tween_entity) else {
                            continue;
                        };
                        let Some(setter_value) =
                            setter_value_component.reflect(tween)
                        else {
                            continue;
                        };
                        let Ok(setter_value) = setter_value.reflect_path(".0")
                        else {
                            continue;
                        };
                        let setter_value = setter_value.clone_value();

                        let Some(entity_mut) =
                            world.get_entity_mut(*target_entity)
                        else {
                            continue;
                        };
                        let Some(mut component) =
                            component.reflect_mut(entity_mut)
                        else {
                            continue;
                        };
                        let Ok(value) = component.reflect_path_mut(&path)
                        else {
                            continue;
                        };
                        let Ok(()) = value.try_apply(&*setter_value) else {
                            continue;
                        };
                    }
                    TargetComponent::Entities(_) => todo!(),
                }
            }
        }
    }
}
