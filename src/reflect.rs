use std::any::TypeId;

use bevy::{
    prelude::*,
    reflect::{self},
};

use crate::{
    prelude::Interpolator,
    tween::{SkipTween, TweenInterpolationValue},
};

pub trait DefaultLerp {
    type V;
    fn lerp(&mut self, to: &Self, value: &Self::V);
}

pub struct ReflectDefaultLerp {
    lerp: fn(&mut dyn Reflect, &dyn Reflect, &dyn Reflect) -> bool,
}

impl ReflectDefaultLerp {
    pub fn lerp(
        &self,
        this: &mut dyn Reflect,
        to: &dyn Reflect,
        value: &dyn Reflect,
    ) {
        (self.lerp)(this, to, value);
    }
}

impl Clone for ReflectDefaultLerp {
    fn clone(&self) -> Self {
        ReflectDefaultLerp { lerp: self.lerp }
    }
}

impl<T> reflect::FromType<T> for ReflectDefaultLerp
where
    T: DefaultLerp + Reflect,
    T::V: Reflect,
{
    fn from_type() -> Self {
        Self {
            lerp: |s, to, value| {
                let Some(s) = s.downcast_mut::<T>() else {
                    return false;
                };
                let Some(to) = to.downcast_ref::<T>() else {
                    return false;
                };
                let Some(value) = value.downcast_ref::<T::V>() else {
                    return false;
                };
                s.lerp(to, value);
                true
            },
        }
    }
}

// fn lerp(&mut self, to: &Box<dyn Reflect>, value: &Box<dyn Reflect>) {
//     let Some(to) = to.downcast_ref::<f32>() else {
//         return;
//     };
//     let Some(value) = value.downcast_ref::<f32>() else {
//         return;
//     };
//     self = Lerp self.lerp(to, value);
// }

#[test]
fn test_app() {
    App::new()
        .add_systems(Update, apply_reflect_tween_system)
        .run();
}

#[derive(Component)]
pub struct TweenReflect {
    target_entity: Entity,
    tween: Option<
        Box<
            dyn Fn(AppTypeRegistry, &mut EntityWorldMut, &dyn Reflect)
                + 'static
                + Send
                + Sync,
        >,
    >,
}

impl TweenReflect {
    fn run<F>(entity: Entity, tween: F) -> TweenReflect
    where
        F: Fn(AppTypeRegistry, &mut EntityWorldMut, &dyn Reflect)
            + 'static
            + Send
            + Sync,
    {
        TweenReflect {
            target_entity: entity,
            tween: Some(Box::new(tween)),
        }
    }

    fn with_interpolator<I>(entity: Entity, interpolator: I) -> TweenReflect
    where
        I: Interpolator,
        I::Item: Component,
    {
        Self::run(entity, move |_, entity, value| {
            let Some(mut component) = entity.get_mut::<I::Item>() else {
                return;
            };
            let Some(value) = value.downcast_ref::<f32>() else {
                return;
            };
            interpolator.interpolate(&mut component, *value);
        })
    }

    fn lerp<F, C, V>(entity: Entity, f: F, to: V) -> TweenReflect
    where
        F: Fn(&mut C) -> &mut V + Send + Sync + 'static,
        C: Component,
        V: DefaultLerp<V = f32> + Send + Sync + 'static,
    {
        Self::run(entity, move |_, entity, value| {
            let Some(mut component) = entity.get_mut::<C>() else {
                return;
            };
            let Some(value) = value.downcast_ref::<f32>() else {
                return;
            };
            let target = f(&mut component);
            target.lerp(&to, value);
        })
    }
}

#[allow(clippy::type_complexity)]
fn apply_reflect_tween_system(
    mut commands: Commands,
    q_tween: Query<
        Entity,
        (
            Without<SkipTween>,
            With<TweenReflect>,
            With<TweenInterpolationValue>,
        ),
    >,
) {
    q_tween.iter().for_each(|tween_entity| {
        commands.add(move |world: &mut World| {
            let Some(mut tween) = world.get_mut::<TweenReflect>(tween_entity)
            else {
                return;
            };
            let tween_fn = tween.tween.take().unwrap();
            let target_entity = tween.target_entity;
            let Some(value) =
                world.get::<TweenInterpolationValue>(tween_entity)
            else {
                return;
            };
            let value = value.0;
            let Some(registry) = world.get_resource::<AppTypeRegistry>() else {
                return;
            };
            let registry = registry.clone();
            let Some(mut target_entity) = world.get_entity_mut(target_entity)
            else {
                return;
            };
            tween_fn(registry, &mut target_entity, value.as_reflect());
        });
    });
}
