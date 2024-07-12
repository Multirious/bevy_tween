use std::time::Duration;

use bevy::prelude::*;
use bevy_time_runner::TimeSpan;

use crate::{
    curve::AToB,
    items::{Set, Setter},
};

use super::AnimationCommands;

pub trait SetWithExt: Sized {
    fn set_with<S: Set>(&self, setter: S) -> TargetSetter<Self, S>;
}

impl SetWithExt for crate::targets::TargetComponent {
    fn set_with<S>(&self, setter: S) -> TargetSetter<Self, S> {
        TargetSetter {
            target: self.clone(),
            setter,
        }
    }
}
impl<A: Asset> SetWithExt for crate::targets::TargetAsset<A> {
    fn set_with<S>(&self, setter: S) -> TargetSetter<Self, S> {
        TargetSetter {
            target: self.clone(),
            setter,
        }
    }
}
impl SetWithExt for crate::targets::TargetResource {
    fn set_with<S>(&self, setter: S) -> TargetSetter<Self, S> {
        TargetSetter {
            target: self.clone(),
            setter,
        }
    }
}

pub struct TargetSetter<T, S> {
    target: T,
    setter: S,
}

impl<T, S> TargetSetter<T, S>
where
    T: Clone + Bundle,
    S: Set + Clone,
{
    pub fn curve<C>(
        &self,
        curve: C,
        duration: Duration,
    ) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
    where
        C: Bundle,
    {
        let setter = self.setter.clone();
        let target = self.target.clone();
        move |a, pos| {
            let start_pos = *pos;
            let end_pos = start_pos + duration;
            a.spawn((
                TimeSpan::try_from(start_pos..end_pos).unwrap(),
                target,
                curve,
                Setter(setter),
            ));
            *pos = end_pos;
        }
    }

    pub fn tween<V, C>(
        &self,
        start: V,
        end: V,
        curve_1d: C,
        duration: Duration,
    ) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
    {
        self.curve(
            AToB {
                a: start,
                b: end,
                curve: curve_1d,
            },
            duration,
        )
    }

    pub fn with_state<V>(self, value: V) -> TargetSetterState<T, S, V> {
        TargetSetterState {
            target: self.target,
            setter: self.setter,
            state: value,
        }
    }
}

pub struct TargetSetterState<T, S, V> {
    target: T,
    setter: S,
    state: V,
}

impl<T, S, V> TargetSetterState<T, S, V>
where
    T: Clone + Bundle,
    V: Clone + Send + Sync + 'static,
    S: Set + Clone,
{
    pub fn tween<C>(
        &mut self,
        start: V,
        end: V,
        curve_1d: C,
        duration: Duration,
    ) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
    {
        self.state = end.clone();
        let setter = self.setter.clone();
        let target = self.target.clone();
        move |a, pos| {
            let start_pos = *pos;
            let end_pos = start_pos + duration;
            a.spawn((
                TimeSpan::try_from(start_pos..end_pos).unwrap(),
                target,
                AToB {
                    a: start,
                    b: end,
                    curve: curve_1d,
                },
                Setter(setter),
            ));
            *pos = end_pos;
        }
    }

    pub fn tween_to<C>(
        &mut self,
        to: V,
        curve: C,
        duration: Duration,
    ) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
    {
        let start = std::mem::replace(&mut self.state, to.clone());
        let end = to;
        self.tween(start, end, curve, duration)
    }

    pub fn tween_to_with<C, F>(
        &mut self,
        with: F,
        curve_1d: C,
        duration: Duration,
    ) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
        F: FnOnce(&mut V) -> V,
    {
        let start = self.state.clone();
        let end = with(&mut self.state);
        self.tween(start, end, curve_1d, duration)
    }
}
