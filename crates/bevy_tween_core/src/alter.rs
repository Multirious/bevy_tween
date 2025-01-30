use bevy_animation::animatable::Animatable;
#[cfg(feature = "bevy_asset")]
use bevy_asset::{Asset, Assets, Handle};
use std::hash::Hash;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{In, Query, ResMut, Resource, SystemParam, SystemParamItem},
};

pub trait Alter: Send + Sync + 'static {
    type Target: Eq + Hash + Clone + Send + Sync + 'static;
    type Value: Animatable + Clone;
    type Param<'w, 's>: for<'w2, 's2> SystemParam<
        Item<'w2, 's2> = Self::Param<'w2, 's2>,
    >;
    fn alter<'w, 'a>(
        input: In<impl Iterator<Item = (&'a Self::Target, &'a Self::Value)>>,
        param: &'w mut SystemParamItem<Self::Param<'w, '_>>,
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

    fn alter<'w, 'a>(
        In(iter): In<impl Iterator<Item = (&'a Self::Target, &'a Self::Value)>>,
        q_component: &'w mut SystemParamItem<Self::Param<'w, '_>>,
    ) {
        for (target, value) in iter {
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

    fn alter<'w, 'a>(
        In(iter): In<impl Iterator<Item = (&'a Self::Target, &'a Self::Value)>>,
        resource: &mut SystemParamItem<Self::Param<'w, '_>>,
    ) {
        let Some(resource) = resource.as_mut() else {
            return;
        };
        for (_, value) in iter {
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

    fn alter<'w, 'a>(
        In(iter): In<impl Iterator<Item = (&'a Self::Target, &'a Self::Value)>>,
        resource: &mut SystemParamItem<Self::Param<'w, '_>>,
    ) {
        let Some(assets) = resource.as_mut() else {
            return;
        };
        for (target, value) in iter {
            let Some(asset) = assets.get_mut(target) else {
                return;
            };
            T::alter_single(asset, value);
        }
    }
}
