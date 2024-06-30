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
            super::apply_component_tween_system::<SpriteColor, _, _>,
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

use crate::{
    curve::CurveValue,
    tween::{TargetComponent, TargetResource},
};
use bevy::{ecs::query::QueryEntityError, prelude::*, utils::HashMap};
use std::any::type_name;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct SkipSetter;

#[cfg(feature = "bevy_sprite")]
pub use sprite::*;

pub trait Setter<Item, Value>: Send + Sync + 'static {
    fn set(&self, item: &mut Item, value: &Value);
}

pub fn apply_component_tween_system<S, C, V>(
    q_tween: Query<
        (Entity, &TargetComponent, &S, &CurveValue<V>),
        Without<SkipSetter>,
    >,
    mut q_component: Query<&mut C>,
    mut last_entity_errors: Local<HashMap<Entity, QueryEntityError>>,
) where
    S: Setter<C, V> + Component,
    C: Component,
    V: Send + Sync + 'static,
{
    let mut query_entity_errors = HashMap::new();
    q_tween.iter().for_each(
        |(tween_entity, target_data, setter, curve_value)| match target_data {
            TargetComponent::Entity(e) => match q_component.get_mut(*e) {
                Ok(mut component) => setter.set(&mut component, &curve_value.0),
                Err(query_error) => {
                    if last_entity_errors
                        .get(&tween_entity)
                        .map(|last_error| last_error != &query_error)
                        .unwrap_or(true)
                        && query_entity_errors
                            .get(&tween_entity)
                            .map(|last_error| last_error != &query_error)
                            .unwrap_or(true)
                    {
                        error!(
                            "{} attempted to mutate {} but got error: {}",
                            type_name::<S>(),
                            type_name::<C>(),
                            query_error
                        );
                    }
                    query_entity_errors.insert(tween_entity, query_error);
                }
            },
            TargetComponent::Entities(e) => {
                let mut iter = q_component.iter_many_mut(e);
                while let Some(mut component) = iter.fetch_next() {
                    setter.set(&mut component, &curve_value.0);
                }
            }
            TargetComponent::None => {}
            TargetComponent::Marker => panic!("remove this variant later"),
        },
    );
    *last_entity_errors = query_entity_errors;
}
}
