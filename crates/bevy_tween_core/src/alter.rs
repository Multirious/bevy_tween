use bevy_animation::animatable::Animatable;
#[cfg(feature = "bevy_asset")]
use bevy_asset::{Asset, Assets, Handle};
use bevy_utils::HashMap;
use std::hash::Hash;

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectResource;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Query, Res, ResMut, Resource, SystemParam, SystemParamItem},
};

pub trait Alter: Send + Sync + 'static + Sized {
    type Target: Eq + Hash + Clone + Send + Sync + 'static;
    type Value: Animatable + Clone;
    type Param<'w, 's>: for<'w2, 's2> SystemParam<
        Item<'w2, 's2> = Self::Param<'w2, 's2>,
    >;
    fn alter_system(
        target_values: Res<'_, TweensTargetFinalValue<Self>>,
        param: SystemParamItem<Self::Param<'_, '_>>,
    );
}

pub trait AlterSingle: Send + Sync + 'static {
    type Value: Animatable + Clone;
    type Item: Send + Sync + 'static;
    fn alter_single(item: &mut Self::Item, value: &Self::Value);
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AlterComponent<T>(pub T)
where
    T: AlterSingle,
    T::Item: Component;

impl<T> Alter for AlterComponent<T>
where
    T: AlterSingle,
    T::Item: Component,
{
    type Target = Entity;
    type Value = T::Value;
    type Param<'w, 's> = Query<'w, 's, &'static mut T::Item>;

    fn alter_system(
        target_values: Res<'_, TweensTargetFinalValue<Self>>,
        mut q_component: SystemParamItem<Self::Param<'_, '_>>,
    ) {
        for (target, value) in target_values.map.iter() {
            let Ok(mut target_component) = q_component.get_mut(*target) else {
                continue;
            };
            T::alter_single(&mut *target_component, value);
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AlterResource<T>(pub T)
where
    T: AlterSingle,
    T::Item: Resource;

impl<T> Alter for AlterResource<T>
where
    T: AlterSingle,
    T::Item: Resource,
{
    type Target = ();
    type Value = T::Value;
    type Param<'w, 's> = Option<ResMut<'w, T::Item>>;

    fn alter_system(
        target_values: Res<'_, TweensTargetFinalValue<Self>>,
        mut resource: SystemParamItem<Self::Param<'_, '_>>,
    ) {
        let Some(resource) = resource.as_mut() else {
            return;
        };
        for value in target_values.map.values() {
            T::alter_single(&mut *resource, value);
        }
    }
}

#[cfg(feature = "bevy_asset")]
#[derive(Default, Debug, Clone, Copy)]
pub struct AlterAsset<T>(pub T)
where
    T: AlterSingle,
    T::Item: Asset;

#[cfg(feature = "bevy_asset")]
impl<T> Alter for AlterAsset<T>
where
    T: AlterSingle,
    T::Item: Asset,
{
    type Target = Handle<T::Item>;
    type Value = T::Value;
    type Param<'w, 's> = Option<ResMut<'w, Assets<T::Item>>>;

    fn alter_system(
        target_values: Res<'_, TweensTargetFinalValue<Self>>,
        mut resource: SystemParamItem<Self::Param<'_, '_>>,
    ) {
        let Some(assets) = resource.as_mut() else {
            return;
        };
        for (target, value) in target_values.map.iter() {
            let Some(asset) = assets.get_mut(target) else {
                return;
            };
            T::alter_single(asset, value);
        }
    }
}

#[derive(Default, Debug, Clone, Resource)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Resource))]
pub struct TweensTargetFinalValue<A>
where
    A: Alter,
{
    pub map: HashMap<A::Target, A::Value>,
}
