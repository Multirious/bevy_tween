use std::time::Duration;

use bevy::prelude::*;
use bevy_time_runner::TimeSpan;

use crate::{curve::AToB, set::Set};

use super::{AnimationCommands, BuildAnimation};

pub trait TargetSetExt: Sized {
    fn set<S: Set>(&self, setter: S) -> TargetSetter<Self, S>;
}

impl TargetSetExt for crate::targets::TargetComponent {
    fn set<S>(&self, setter: S) -> TargetSetter<Self, S> {
        TargetSetter {
            target: self.clone(),
            setter,
        }
    }
}
impl<A: Asset> TargetSetExt for crate::targets::TargetAsset<A> {
    fn set<S>(&self, setter: S) -> TargetSetter<Self, S> {
        TargetSetter {
            target: self.clone(),
            setter,
        }
    }
}
impl TargetSetExt for crate::targets::TargetResource {
    fn set<S>(&self, setter: S) -> TargetSetter<Self, S> {
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
    S: Set + Clone + Component,
{
    pub fn curve<C>(&self, duration: Duration, curve: C) -> Tween<T, S, C>
    where
        C: Bundle,
    {
        Tween {
            duration,
            target: self.target.clone(),
            setter: self.setter.clone(),
            curve,
        }
    }

    pub fn tween<V, C>(
        &self,
        start: V,
        end: V,
        duration: Duration,
        curve_1d: C,
    ) -> Tween<T, S, AToB<V, C>>
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
    {
        Tween {
            duration,
            target: self.target.clone(),
            setter: self.setter.clone(),
            curve: AToB {
                a: start,
                b: end,
                curve: curve_1d,
            },
        }
    }

    pub fn state<V>(self, value: V) -> TargetSetterState<T, S, V> {
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
    S: Set + Clone + Component,
{
    pub fn tween<C>(
        &mut self,
        start: V,
        end: V,
        duration: Duration,
        curve_1d: C,
    ) -> Tween<T, S, AToB<V, C>>
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
    {
        self.state = end.clone();
        Tween {
            duration,
            target: self.target.clone(),
            setter: self.setter.clone(),
            curve: AToB {
                a: start,
                b: end,
                curve: curve_1d,
            },
        }
    }

    pub fn tween_to<C>(
        &mut self,
        to: V,
        duration: Duration,
        curve: C,
    ) -> Tween<T, S, AToB<V, C>>
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
    {
        let start = std::mem::replace(&mut self.state, to.clone());
        let end = to;
        self.tween(start, end, duration, curve)
    }

    pub fn tween_to_with<C, F>(
        &mut self,
        with: F,
        duration: Duration,
        curve_1d: C,
    ) -> Tween<T, S, AToB<V, C>>
    where
        V: Send + Sync + 'static,
        C: Send + Sync + 'static,
        F: FnOnce(&mut V) -> V,
    {
        let start = self.state.clone();
        let end = with(&mut self.state);
        self.tween(start, end, duration, curve_1d)
    }
}

pub struct Tween<T: Bundle, S: Set + Bundle, C: Bundle> {
    duration: Duration,
    target: T,
    setter: S,
    curve: C,
}

impl<T, S, C> BuildAnimation for Tween<T, S, C>
where
    T: Bundle,
    S: Set + Bundle,
    C: Bundle,
{
    fn build(self, commands: &mut AnimationCommands, position: &mut Duration) {
        let start = *position;
        let end = *position + self.duration;
        commands.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            self.target,
            self.setter,
            self.curve,
        ));
        *position = end;
    }
}
