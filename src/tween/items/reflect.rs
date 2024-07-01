use std::any::{Any, TypeId};

use bevy::{
    prelude::*,
    reflect::{self},
};

use crate::{
    curve::CurveValue,
    tween::{SkipTween, TargetComponent},
};

#[test]
fn test_app() {
    App::new()
        .add_systems(Update, apply_component_reflect_tween_system)
        .run();
}

#[derive(Component)]
pub struct SetReflect(
    Option<
        Box<
            dyn Fn(&dyn Reflect, &mut World, &dyn Reflect)
                + 'static
                + Send
                + Sync,
        >,
    >,
);

impl SetReflect {
    fn field_of_component<F, C, V>(select_field: F) -> SetReflect
    where
        F: Fn(&mut Component) -> &mut V,
        C: Component,
        V: Send + Sync + 'static + Copy,
    {
        SetReflect(Some(Box::new(move |input, world, value| {
            let Ok(entity) = input.downcast_ref::<Entity>() else {
                return;
            };
            let Some(mut component) = world.get_mut::<C>(entity) else {
                return;
            };
            let Some(value) = value.downcast_ref::<V>() else {
                return;
            };
            let field = select_field(&mut component);
            *field = *value;
        })))
    }
}

#[allow(clippy::type_complexity)]
fn apply_component_reflect_tween_system<V: Send + Sync + 'static + Copy>(
    mut commands: Commands,
    q_tween: Query<
        Entity,
        (
            Without<SkipTween>,
            With<SetReflect>,
            With<CurveValue<V>>,
            With<TargetComponent>,
        ),
    >,
) {
    q_tween.iter().for_each(|tween_entity| {
        commands.add(move |world: &mut World| {
            let Some(mut set_reflect) =
                world.get_mut::<SetReflect>(tween_entity)
            else {
                return;
            };
            let Some(set_fn) = set_reflect.0.take() else {
                return;
            };
            let Some(target) = world.get::<TargetComponent>(tween_entity)
            else {
                return;
            };
            let targets = match target {
                TargetComponent::None => return,
                TargetComponent::Entity(entity) => vec![entity],
                TargetComponent::Entities(entities) => entities.clone(),
            };
            let Some(value) = world.get::<CurveValue<V>>(tween_entity) else {
                return;
            };
            let value = value.0;
            for target in targets {
                set_fn(target, world, &value);
            }
            let Some(mut set_reflect) =
                world.get_mut::<SetReflect>(tween_entity)
            else {
                return;
            };
        });
    });
}
