use bevy::prelude::*;

/// Tell the tween what component of what entity to tween.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TargetComponent {
    /// The target is not yet selected or resolved.
    None,
    /// Target this entity.
    Entity(Entity),
    /// Target these entities.
    Entities(Vec<Entity>),
}

impl TargetComponent {
    /// Target this entity.
    pub fn entity(entity: Entity) -> TargetComponent {
        TargetComponent::Entity(entity)
    }

    /// Target these entities.
    pub fn entities<I>(entities: I) -> TargetComponent
    where
        I: IntoIterator<Item = Entity>,
    {
        TargetComponent::from_iter(entities)
    }

    // /// Create a new [`TargetState`] with the initial value out of this target.
    // pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
    //     TargetState::new(self.clone(), value)
    // }

    // /// Create a new tween with the supplied interpolator out of this target.
    // pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
    //     Tween {
    //         target: self.clone(),
    //         interpolator,
    //     }
    // }

    // /// Create a new tween with the supplied closure out of this target.
    // pub fn with_closure<F, C>(
    //     &self,
    //     closure: F,
    // ) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    // where
    //     F: Fn(&mut C, f32) + Send + Sync + 'static,
    //     C: Component,
    // {
    //     let closure = crate::interpolate::closure(closure);
    //     let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
    //     Tween {
    //         target: self.clone(),
    //         interpolator,
    //     }
    // }
}

impl Default for TargetComponent {
    fn default() -> Self {
        TargetComponent::None
    }
}

impl From<Entity> for TargetComponent {
    fn from(value: Entity) -> Self {
        TargetComponent::entity(value)
    }
}

impl FromIterator<Entity> for TargetComponent {
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        TargetComponent::Entities(iter.into_iter().collect())
    }
}

impl<const N: usize> From<[Entity; N]> for TargetComponent {
    fn from(value: [Entity; N]) -> Self {
        TargetComponent::entities(value)
    }
}

impl From<Vec<Entity>> for TargetComponent {
    fn from(value: Vec<Entity>) -> Self {
        TargetComponent::entities(value)
    }
}

impl From<&Vec<Entity>> for TargetComponent {
    fn from(value: &Vec<Entity>) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl From<&[Entity]> for TargetComponent {
    fn from(value: &[Entity]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

impl<const N: usize> From<&[Entity; N]> for TargetComponent {
    fn from(value: &[Entity; N]) -> Self {
        TargetComponent::entities(value.iter().copied())
    }
}

/// Tell the tween what resource to tween.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct TargetResource;

impl TargetResource {
    /// New resource target
    pub fn new() -> TargetResource {
        TargetResource
    }

    // /// Create a new [`TargetState`] with the initial value out of this target.
    // pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
    //     TargetState::new(self.clone(), value)
    // }

    // /// Create a new tween with the supplied interpolator out of this target.
    // pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
    //     Tween {
    //         target: self.clone(),
    //         interpolator,
    //     }
    // }

    // /// Create a new tween with the supplied closure out of this target.
    // pub fn with_closure<F, C>(
    //     &self,
    //     closure: F,
    // ) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    // where
    //     F: Fn(&mut C, f32) + Send + Sync + 'static,
    //     C: Component,
    // {
    //     let closure = crate::interpolate::closure(closure);
    //     let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
    //     Tween {
    //         target: self.clone(),
    //         interpolator,
    //     }
    // }
}

/// Tell the tween what asset of what type to tween.
#[cfg(feature = "bevy_asset")]
#[derive(Debug, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TargetAsset<A: Asset>
where
    A: Asset,
{
    /// The target is not yet selected or resolved.
    None,
    /// Target this asset
    Asset(Handle<A>),
    /// Target these assets
    Assets(Vec<Handle<A>>),
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> TargetAsset<A> {
    /// Target this asset
    pub fn asset(asset: Handle<A>) -> Self {
        TargetAsset::Asset(asset)
    }

    /// Target these assets
    pub fn assets<I>(assets: I) -> Self
    where
        I: IntoIterator<Item = Handle<A>>,
    {
        TargetAsset::from_iter(assets)
    }

    // /// Create a new [`TargetState`] with the initial value out of this target.
    // pub fn state<V>(&self, value: V) -> TargetState<Self, V> {
    //     TargetState::new(self.clone(), value)
    // }

    // /// Create a new tween with the supplied interpolator out of this target.
    // pub fn with<I>(&self, interpolator: I) -> Tween<Self, I> {
    //     Tween {
    //         target: self.clone(),
    //         interpolator,
    //     }
    // }

    // /// Create a new tween with the supplied closure out of this target.
    // pub fn with_closure<F, C>(
    //     &self,
    //     closure: F,
    // ) -> Tween<Self, Box<dyn Interpolator<Item = C>>>
    // where
    //     F: Fn(&mut C, f32) + Send + Sync + 'static,
    //     C: Component,
    // {
    //     let closure = crate::interpolate::closure(closure);
    //     let interpolator: Box<dyn Interpolator<Item = C>> = Box::new(closure);
    //     Tween {
    //         target: self.clone(),
    //         interpolator,
    //     }
    // }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> Clone for TargetAsset<A> {
    fn clone(&self) -> Self {
        match self {
            TargetAsset::None => TargetAsset::None,
            TargetAsset::Asset(handle) => TargetAsset::Asset(handle.clone()),
            TargetAsset::Assets(v) => TargetAsset::Assets(v.clone()),
        }
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> Default for TargetAsset<A> {
    fn default() -> Self {
        TargetAsset::Asset(Default::default())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<Handle<A>> for TargetAsset<A> {
    fn from(value: Handle<A>) -> Self {
        TargetAsset::Asset(value)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> FromIterator<Handle<A>> for TargetAsset<A> {
    fn from_iter<T: IntoIterator<Item = Handle<A>>>(iter: T) -> Self {
        TargetAsset::Assets(iter.into_iter().collect())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> From<[Handle<A>; N]> for TargetAsset<A> {
    fn from(value: [Handle<A>; N]) -> Self {
        TargetAsset::assets(value)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<Vec<Handle<A>>> for TargetAsset<A> {
    fn from(value: Vec<Handle<A>>) -> Self {
        TargetAsset::assets(value)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<&Vec<Handle<A>>> for TargetAsset<A> {
    fn from(value: &Vec<Handle<A>>) -> Self {
        TargetAsset::assets(value.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> From<&[Handle<A>]> for TargetAsset<A> {
    fn from(value: &[Handle<A>]) -> Self {
        TargetAsset::assets(value.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> From<&[Handle<A>; N]> for TargetAsset<A> {
    fn from(value: &[Handle<A>; N]) -> Self {
        TargetAsset::assets(value.iter().cloned())
    }
}

/// Trait for type to convert into a target type.
pub trait IntoTarget {
    /// The target type
    type Target;

    /// Convert [`Self`] into [`Self::Target`]
    fn into_target(self) -> Self::Target;
}

impl IntoTarget for Entity {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entity(self)
    }
}

impl<const N: usize> IntoTarget for [Entity; N] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self)
    }
}

impl IntoTarget for Vec<Entity> {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self)
    }
}

impl IntoTarget for &[Entity] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

impl<const N: usize> IntoTarget for &[Entity; N] {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

impl IntoTarget for &Vec<Entity> {
    type Target = TargetComponent;

    fn into_target(self) -> Self::Target {
        TargetComponent::entities(self.iter().copied())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A> IntoTarget for Handle<A>
where
    A: Asset,
{
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::asset(self)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> IntoTarget for [Handle<A>; N] {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> IntoTarget for Vec<Handle<A>> {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self)
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> IntoTarget for &[Handle<A>] {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset, const N: usize> IntoTarget for &[Handle<A>; N] {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self.iter().cloned())
    }
}

#[cfg(feature = "bevy_asset")]
impl<A: Asset> IntoTarget for &Vec<Handle<A>> {
    type Target = TargetAsset<A>;

    fn into_target(self) -> Self::Target {
        TargetAsset::assets(self.iter().cloned())
    }
}
