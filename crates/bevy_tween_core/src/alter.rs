#[cfg(feature = "bevy_asset")]
use bevy_asset::{Asset, Assets, Handle, UntypedHandle};

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::QueryEntityError,
    system::{In, Query, ResMut, Resource, SystemParam, SystemParamItem},
};

use super::argument;

pub trait Alter: Send + Sync + 'static {
    type Target: Send + Sync + 'static;
    type Value: Send + Sync + 'static;
    type Param<'w, 's>: for<'w2, 's2> SystemParam<
        Item<'w2, 's2> = Self::Param<'w2, 's2>,
    >;
    type Error<'w>;
    fn alter<'a>(
        input: In<(Entity, &Self::Target, &Self::Value)>,
        param: &'a mut SystemParamItem<Self::Param<'a, '_>>,
    ) -> Result<(), Self::Error<'a>>;
}

pub trait AlterSingle: Send + Sync + 'static {
    type Value: Send + Sync + 'static;
    type Item: Send + Sync + 'static;
    fn alter_single(&self, value: &Self::Value, item: &mut Self::Item);
}

#[derive(Debug)]
pub struct AlterComponent<T>(pub T)
where
    T: AlterSingle,
    T::Item: Component;

#[derive(Debug, thiserror::Error)]
pub enum AlterComponentError<'w> {
    #[error("Query source error: {0}")]
    QuerySourceError(QueryEntityError<'w>),
    #[error("Query component error: {0}")]
    QueryComponentError(QueryEntityError<'w>),
}

impl<T> Alter for AlterComponent<T>
where
    T: AlterSingle,
    T::Item: Component,
{
    type Target = Entity;
    type Value = T::Value;
    type Param<'w, 's> = (
        Query<'w, 's, &'static argument::Alterer<AlterComponent<T>>>,
        Query<'w, 's, &'static mut T::Item>,
    );
    type Error<'w> = AlterComponentError<'w>;

    fn alter<'a>(
        In((source, target, value)): In<(Entity, &Self::Target, &Self::Value)>,
        (q_source, q_component): &'a mut SystemParamItem<Self::Param<'a, '_>>,
    ) -> Result<(), Self::Error<'a>> {
        let mut target_component = q_component
            .get_mut(*target)
            .map_err(AlterComponentError::QueryComponentError)?;
        let alterer = q_source
            .get_mut(source)
            .map_err(AlterComponentError::QuerySourceError)?;
        alterer.0 .0.alter_single(value, &mut *target_component);
        Ok(())
    }
}

pub struct AlterResource<T>(pub T)
where
    T: AlterSingle,
    T::Item: Resource;

#[derive(Debug, thiserror::Error)]
pub enum AlterResourceError<'w> {
    #[error("Query source error: {0}")]
    QuerySourceError(QueryEntityError<'w>),
    #[error("Resource does not exists")]
    ResourceNotExists,
}

impl<T> Alter for AlterResource<T>
where
    T: AlterSingle,
    T::Item: Resource,
{
    type Target = ();
    type Value = T::Value;
    type Param<'w, 's> = (
        Query<'w, 's, &'static argument::Alterer<AlterResource<T>>>,
        Option<ResMut<'w, T::Item>>,
    );
    type Error<'w> = AlterResourceError<'w>;

    fn alter<'a>(
        In((source, (), value)): In<(Entity, &Self::Target, &Self::Value)>,
        (q_source, resource): &'a mut SystemParamItem<Self::Param<'a, '_>>,
    ) -> Result<(), Self::Error<'a>> {
        let alterer = q_source
            .get_mut(source)
            .map_err(AlterResourceError::QuerySourceError)?;
        let resource = resource
            .as_mut()
            .ok_or(AlterResourceError::ResourceNotExists)?;
        alterer.0 .0.alter_single(value, &mut *resource);
        Ok(())
    }
}

#[cfg(feature = "bevy_asset")]
pub struct AlterAsset<T>(pub T)
where
    T: AlterSingle,
    T::Item: Asset;

#[cfg(feature = "bevy_asset")]
#[derive(Debug, thiserror::Error)]
pub enum AlterAssetError<'w> {
    #[error("Query source error: {0}")]
    QuerySourceError(QueryEntityError<'w>),
    #[error("Asset resource does not exists")]
    AssetResourceDoesNotExists,
    #[error("Asset {0:?} does not exists")]
    AssetDoesNotExists(UntypedHandle),
}

#[cfg(feature = "bevy_asset")]
impl<T> Alter for AlterAsset<T>
where
    T: AlterSingle,
    T::Item: Asset,
{
    type Target = Handle<T::Item>;
    type Value = T::Value;
    type Param<'w, 's> = (
        Query<'w, 's, &'static argument::Alterer<AlterAsset<T>>>,
        Option<ResMut<'w, Assets<T::Item>>>,
    );
    type Error<'w> = AlterAssetError<'w>;

    fn alter<'a>(
        In((source, target, value)): In<(Entity, &Self::Target, &Self::Value)>,
        (q_source, resource): &'a mut SystemParamItem<Self::Param<'a, '_>>,
    ) -> Result<(), Self::Error<'a>> {
        let alterer = q_source
            .get_mut(source)
            .map_err(AlterAssetError::QuerySourceError)?;
        let assets = resource
            .as_mut()
            .ok_or(AlterAssetError::AssetResourceDoesNotExists)?;
        let asset = assets.get_mut(target).ok_or(
            AlterAssetError::AssetDoesNotExists(target.clone_weak().untyped()),
        )?;
        alterer.0 .0.alter_single(value, asset);
        Ok(())
    }
}
