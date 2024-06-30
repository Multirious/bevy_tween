mod blanket_impl {
    use super::Setter;
    use std::sync::Arc;

    impl<S, Item, Value> Setter<Item, Value> for Box<S>
    where
        S: Setter<Item, Value> + ?Sized,
    {
        fn set(&self, item: &mut Item, value: &Value) {
            (**self).set(item, value)
        }
    }

    impl<S, Item, Value> Setter<Item, Value> for &'static S
    where
        S: Setter<Item, Value> + ?Sized,
    {
        fn set(&self, item: &mut Item, value: &Value) {
            (**self).set(item, value)
        }
    }

    impl<S, Item, Value> Setter<Item, Value> for Arc<S>
    where
        S: Setter<Item, Value> + ?Sized,
    {
        fn set(&self, item: &mut Item, value: &Value) {
            (**self).set(item, value)
        }
    }

    impl<Item: 'static, Value: 'static> Setter<Item, Value>
        for dyn Fn(&mut Item, &Value) + Send + Sync + 'static
    {
        fn set(&self, item: &mut Item, value: &Value) {
            self(item, value)
        }
    }
}

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
            super::set_component_from_curve_value_system::<SpriteColor, _, _>,
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

use crate::{curve::CurveValue, tween::TargetComponent};
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipSetter;

#[cfg(feature = "bevy_sprite")]
pub use sprite::*;

pub trait Setter<Item, Value>: Send + Sync + 'static {
    fn set(&self, item: &mut Item, value: &Value);
}

pub fn set_component_from_curve_value_system<S, C, V>(
    q_setter: Query<
        (Entity, &TargetComponent, &S, &CurveValue<V>),
        Without<SkipSetter>,
    >,
    mut q_component: Query<&mut C>,
) where
    S: Setter<C, V> + Component,
    V: Send + Sync + 'static,
    C: Component,
{
    q_setter
        .iter()
        .for_each(|(_, target_data, setter, curve_value)| match target_data {
            TargetComponent::Entity(e) => match q_component.get_mut(*e) {
                Ok(mut component) => setter.set(&mut component, &curve_value.0),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        });
}
