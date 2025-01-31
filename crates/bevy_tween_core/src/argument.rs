use std::marker::PhantomData;

use bevy_ecs::component::Component;

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
pub struct TweenRoot;

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

#[derive(Debug, Component, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct SampledValue<V>(pub Option<V>)
where
    V: Send + Sync + 'static;

impl<V> Default for SampledValue<V>
where
    V: Send + Sync + 'static,
{
    fn default() -> Self {
        SampledValue(None)
    }
}

#[derive(Default, Debug, Component, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
pub struct FinalValue<V>(pub Option<V>)
where
    V: Send + Sync + 'static;

#[derive(Default, Debug, Component, Clone, Copy)]
#[require(SampledValue<V>)]
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

#[derive(Debug, Component, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect(Component))]
#[non_exhaustive]
pub struct Blend {
    pub weigth: f32,
    pub additive: bool,
}

impl Default for Blend {
    fn default() -> Blend {
        Blend {
            weigth: 1.0,
            additive: false,
        }
    }
}
