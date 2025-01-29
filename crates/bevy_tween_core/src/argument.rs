use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};

#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;

use crate::alter::Alter;

#[derive(Default, Debug, Component, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct Tween;

#[derive(Default, Debug, Component, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct Target<T>(pub T)
where
    T: Send + Sync + 'static;

#[derive(Debug, Component, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct Alterer<A: Alter>(pub A);

#[derive(Default, Component)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct BlendInputs<V>(pub Vec<bevy_animation::animatable::BlendInput<V>>)
where
    V: bevy_animation::prelude::Animatable;

#[derive(Default, Debug, Component, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct FinalValue<V>(pub V)
where
    V: Send + Sync + 'static;

#[derive(Debug, Component, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct ValueInputId(pub Entity);

#[derive(Default, Debug, Component, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct Curve<C, V>(pub C, PhantomData<V>)
where
    C: bevy_math::curve::Curve<V> + Send + Sync + 'static,
    V: Send + Sync + 'static;

impl<C, V> Curve<C, V>
where
    C: bevy_math::curve::Curve<V> + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    pub fn new(curve: C) -> Curve<C, V> {
        Curve(curve, PhantomData)
    }
}
