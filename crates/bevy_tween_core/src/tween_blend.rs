use std::hash::Hash;

use bevy_animation::animatable::{Animatable, BlendInput};
use bevy_ecs::system::Resource;
use bevy_utils::HashMap;

use crate::Alter;

#[derive(Resource)]
pub struct TweenBlend<A>
where
    A: Alter,
    A::Target: Eq + Hash + Clone,
    A::Value: Animatable + Clone,
{
    inputs: HashMap<A::Target, (Vec<BlendInput<A::Value>>, Option<A::Value>)>,
}

impl<A> TweenBlend<A>
where
    A: Alter,
    A::Target: Eq + Hash + Clone,
    A::Value: Animatable + Clone,
{
    pub fn new() -> TweenBlend<A> {
        TweenBlend {
            inputs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, target: &A::Target, value: BlendInput<A::Value>) {
        let (_, inputs) = self
            .inputs
            .raw_entry_mut()
            .from_key(target)
            .or_insert_with(|| (target.clone(), (Vec::with_capacity(1), None)));
        inputs.0.push(value);
    }

    pub fn clear_inputs(&mut self) {
        self.inputs.iter_mut().for_each(|(_, v)| {
            v.0.clear();
        });
    }

    pub fn blend(&self, target: &A::Target) -> Option<A::Value> {
        let inputs = self.inputs.get(target)?;
        if inputs.0.is_empty() {
            return None;
        }
        let iter = inputs.0.iter().map(|a| BlendInput {
            weight: a.weight,
            value: a.value.clone(),
            additive: a.additive,
        });
        let blended = <A::Value as Animatable>::blend(iter);
        Some(blended)
    }

    pub(crate) fn blend_all_and_set_final_value(&mut self) {
        for (inputs, final_value) in self.inputs.values_mut() {
            if inputs.is_empty() {
                *final_value = None;
                continue;
            }
            let iter = inputs.iter().map(|a| BlendInput {
                weight: a.weight,
                value: a.value.clone(),
                additive: a.additive,
            });
            let blended = <A::Value as Animatable>::blend(iter);
            *final_value = Some(blended);
        }
    }

    pub fn final_value(&self, target: &A::Target) -> Option<&A::Value> {
        self.inputs.get(target)?.1.as_ref()
    }

    pub(crate) fn iter_targets_value(
        &self,
    ) -> impl Iterator<Item = (&A::Target, &A::Value)> {
        self.inputs
            .iter()
            .filter_map(|(target, (_, value))| match value {
                Some(value) => Some((target, value)),
                None => None,
            })
    }
}
