use bevy::prelude::*;
use bevy_time_runner::TimeSpanProgress;

use crate::{
    targets::{TargetAsset, TargetComponent, TargetResource},
    SkipTween, TweenAppResource, TweenSystemSet,
};

use super::SetterValue;

pub struct SetWorldPlugin;

impl Plugin for SetWorldPlugin {
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<TweenAppResource>()
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            set_world_system.in_set(TweenSystemSet::Apply),
        );
    }
}

#[derive(Component)]
#[allow(clippy::type_complexity)]
pub struct SetWorld(
    pub(crate) Option<Box<dyn Fn(Entity, &mut World) + 'static + Send + Sync>>,
);

impl SetWorld {
    pub fn component<F, C, V>(select_property: F) -> SetWorld
    where
        F: Send + Sync + 'static + Fn(&mut C) -> &mut V,
        C: Component,
        V: Send + Sync + 'static + Copy,
    {
        SetWorld(Some(Box::new(move |tween_entity, world| {
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
                    let value = value.0;

                    let Some(mut component) = world.get_mut::<C>(*entity)
                    else {
                        return;
                    };
                    let field = select_property(&mut component);

                    *field = value
                }
                TargetComponent::Entities(entities) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0;

                    let entities = entities.clone();
                    for entity in entities {
                        let Some(mut component) = world.get_mut::<C>(entity)
                        else {
                            return;
                        };
                        let field = select_property(&mut component);

                        *field = value
                    }
                }
            }
        })))
    }

    pub fn asset<F, A, V>(select_property: F) -> SetWorld
    where
        F: Send + Sync + 'static + Fn(&mut A) -> &mut V,
        A: Asset,
        V: Send + Sync + 'static + Copy,
    {
        SetWorld(Some(Box::new(move |tween_entity, world| {
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
                    let value = value.0;

                    let handle = handle.clone();
                    let Some(mut assets) =
                        world.get_resource_mut::<Assets<A>>()
                    else {
                        return;
                    };
                    let Some(asset) = assets.get_mut(&handle) else {
                        return;
                    };
                    let field = select_property(asset);

                    *field = value
                }
                TargetAsset::Assets(handles) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0;

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
                        let field = select_property(asset);

                        *field = value
                    }
                }
            }
        })))
    }

    pub fn resource<F, R, V>(select_property: F) -> SetWorld
    where
        F: Send + Sync + 'static + Fn(&mut R) -> &mut V,
        R: Resource,
        V: Send + Sync + 'static + Copy,
    {
        SetWorld(Some(Box::new(move |tween_entity, world| {
            let Some(_target_resource) =
                world.get::<TargetResource>(tween_entity)
            else {
                return;
            };

            let Some(value) = world.get::<SetterValue<V>>(tween_entity) else {
                return;
            };
            let value = value.0;

            let Some(mut resource) = world.get_resource_mut::<R>() else {
                return;
            };
            let property = select_property(&mut resource);
            *property = value;
        })))
    }

    pub fn handle_component<F, A, V>(select_property: F) -> SetWorld
    where
        F: Send + Sync + 'static + Fn(&mut A) -> &mut V,
        A: Asset,
        V: Send + Sync + 'static + Copy,
    {
        SetWorld(Some(Box::new(move |tween_entity, world| {
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
                    let value = value.0;

                    let Some(handle) = world.get::<Handle<A>>(*entity) else {
                        return;
                    };
                    let handle = handle.clone();

                    let Some(mut assets_res) =
                        world.get_resource_mut::<Assets<A>>()
                    else {
                        return;
                    };
                    let Some(asset) = assets_res.get_mut(&handle) else {
                        return;
                    };

                    let property = select_property(asset);

                    *property = value
                }
                TargetComponent::Entities(entities) => {
                    let Some(value) = world.get::<SetterValue<V>>(tween_entity)
                    else {
                        return;
                    };
                    let value = value.0;

                    let entities = entities.clone();
                    for entity in entities {
                        let Some(handle) = world.get::<Handle<A>>(entity)
                        else {
                            return;
                        };
                        let handle = handle.clone();
                        let Some(mut assets_res) =
                            world.get_resource_mut::<Assets<A>>()
                        else {
                            return;
                        };

                        let Some(asset) = assets_res.get_mut(&handle) else {
                            return;
                        };

                        let property = select_property(asset);

                        *property = value
                    }
                }
            }
        })))
    }
}

fn set_world_system(world: &mut World) {
    let mut query = world.query_filtered::<Entity, (
        With<SetWorld>,
        Without<SkipTween>,
        With<TimeSpanProgress>,
    )>();
    let entities = query.iter(world).collect::<Vec<_>>();
    for entity in entities {
        let Some(mut set_reflect) = world.get_mut::<SetWorld>(entity) else {
            return;
        };
        let Some(set) = set_reflect.0.take() else {
            return;
        };
        set(entity, world);

        let Some(mut set_reflect) = world.get_mut::<SetWorld>(entity) else {
            return;
        };
        set_reflect.0 = Some(set);
    }
}
