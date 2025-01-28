use std::fmt::Display;

use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, In, Query},
};
use bevy_math::{curve::Curve, FloatExt};

use crate::alter::Alter;

use super::argument;

pub fn clear_blend_inputs_system<V>(
    mut q_values: Query<&mut argument::BlendInputs<V>>,
) where
    V: bevy_animation::animatable::Animatable,
{
    q_values
        .par_iter_mut()
        .for_each(|mut blend_inputs| blend_inputs.0.clear());
}

pub fn progress_curve_system<C, V>(
    q_curve_progresses: Query<(
        &argument::Curve<C, V>,
        &argument::ValueInputId,
        &bevy_time_runner::TimeSpanProgress,
    )>,
    mut q_values: Query<&mut argument::BlendInputs<V>>,
) where
    C: Curve<V> + Send + Sync + 'static,
    V: bevy_animation::animatable::Animatable,
{
    for (curve, values_id, progress) in &q_curve_progresses {
        let mut values = q_values.get_mut(values_id.0).expect("Values exists");
        let domain = curve.0.domain();

        let Some(sampled) = curve
            .0
            .sample(domain.start().lerp(domain.end(), progress.now_percentage))
        else {
            continue;
        };
        values.0.push(bevy_animation::animatable::BlendInput {
            value: sampled,
            weight: 1.0,
            additive: false,
        });
    }
}

pub fn blend_inputs_system<V>(
    mut commands: Commands,
    mut values: Query<(
        Entity,
        &argument::BlendInputs<V>,
        Option<&mut argument::FinalValue<V>>,
    )>,
) where
    V: bevy_animation::animatable::Animatable + Clone,
{
    for (id, blend_inputs, final_value) in &mut values {
        if blend_inputs.0.is_empty() && final_value.is_some() {
            commands.entity(id).remove::<argument::FinalValue<V>>();
            continue;
        }
        let value = bevy_animation::animatable::Animatable::blend(
            blend_inputs.0.iter().map(|i| {
                bevy_animation::animatable::BlendInput {
                    weight: i.weight,
                    value: i.value.clone(),
                    additive: i.additive,
                }
            }),
        );
        match final_value {
            Some(mut final_value) => {
                final_value.0 = value;
            }
            None => {
                commands.entity(id).insert(argument::FinalValue(value));
            }
        }
    }
}

pub fn alterer_system<A>(
    q_alterers: Query<
        (
            Entity,
            &argument::Target<A::Target>,
            &argument::ValueInputId,
        ),
        With<argument::Alterer<A>>,
    >,
    q_final_value: Query<&argument::FinalValue<A::Value>>,
    mut param: A::Param<'_, '_>,
) where
    A: Alter,
    for<'w> A::Error<'w>: Display,
{
    for (source_id, target, values_id) in &q_alterers {
        let values = q_final_value.get(values_id.0).expect("Values exists");
        let result =
            A::alter(In((source_id, &target.0, &values.0)), &mut param);
        match result {
            Ok(()) => {}
            Err(e) => bevy_log::warn!("{e}"),
        }
    }
}
