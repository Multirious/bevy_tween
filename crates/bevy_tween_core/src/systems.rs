use bevy_animation::animatable::Animatable;
use bevy_ecs::{
    query::With,
    system::{Query, ResMut},
};
use bevy_math::{curve::Curve, FloatExt};

use crate::alter::Alter;

use super::argument;

pub fn progress_curve_system<C, V>(
    mut q_curve_progresses: Query<(
        &argument::Curve<C, V>,
        &mut argument::SampledValue<V>,
        &bevy_time_runner::TimeSpanProgress,
    )>,
) where
    C: Curve<V> + Send + Sync + 'static,
    V: Animatable,
{
    q_curve_progresses.par_iter_mut().for_each(
        |(curve, mut sampled_value, progress)| {
            let domain = curve.0.domain();
            let Some(value) = curve.0.sample(
                domain.start().lerp(domain.end(), progress.now_percentage),
            ) else {
                return;
            };
            sampled_value.0 = Some(value);
        },
    );
}

pub fn update_blend_system<A>(
    mut res: ResMut<crate::TweenBlend<A>>,
    values: Query<
        (
            &argument::Target<A::Target>,
            &argument::SampledValue<A::Value>,
            Option<&argument::Blend>,
        ),
        With<argument::Alterer<A>>,
    >,
    mut q_final_values: Query<
        (
            &argument::Target<A::Target>,
            &mut argument::FinalValue<A::Value>,
        ),
        With<argument::Alterer<A>>,
    >,
) where
    A: Alter,
{
    res.clear_inputs();
    values.iter().for_each(|(target, value, blend)| {
        let Some(value) = &value.0 else { return };
        let input = match blend {
            Some(blend) => bevy_animation::prelude::BlendInput {
                weight: blend.weigth,
                value: value.clone(),
                additive: blend.additive,
            },
            None => bevy_animation::prelude::BlendInput {
                weight: 1.0,
                value: value.clone(),
                additive: false,
            },
        };
        res.insert(&target.0, input);
    });

    res.blend_all_and_set_final_value();
    q_final_values
        .par_iter_mut()
        .for_each(|(target, mut value)| {
            value.0 = res.final_value(&target.0).cloned();
        });
}
