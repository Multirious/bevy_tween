use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;
use bevy_time_runner::TimeSpan;

use crate::{
    curve::AToB,
    set::{DynamicSetter, Set},
};

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

#[derive(Bundle, Clone)]
pub struct WorldSetterMarker<V>
where
    V: Send + Sync + 'static,
{
    #[bundle(ignore)]
    marker: PhantomData<V>,
    pub world_setter: DynamicSetter,
}
impl<V> WorldSetterMarker<V>
where
    V: Send + Sync + 'static,
{
    pub fn new(world_setter: DynamicSetter) -> WorldSetterMarker<V> {
        WorldSetterMarker {
            marker: PhantomData,
            world_setter,
        }
    }
}

impl<V> Set for WorldSetterMarker<V>
where
    V: Send + Sync + 'static,
{
    type Item = ();
    type Value = V;

    fn set(&self, _item: &mut Self::Item, _value: &Self::Value) {
        panic!(
            "This Set impl is only used for type checking and must not be used for anything else"
        );
    }
}

pub trait TargetSetComponentWorldExt: Sized {
    fn world_set_component<F, C, V>(
        &self,
        select_property: F,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        F: Send + Sync + 'static + Fn(&mut C, &V),
        C: Component,
        V: Send + Sync + 'static + Clone;
}

impl TargetSetComponentWorldExt for crate::targets::TargetComponent {
    fn world_set_component<F, C, V>(
        &self,
        set: F,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        F: Send + Sync + 'static + Fn(&mut C, &V),
        C: Component,
        V: Send + Sync + 'static + Clone,
    {
        self.set(WorldSetterMarker::new(DynamicSetter::component(set)))
    }
}

pub trait TargetSetAssetWorldExt<A: Asset>: Sized {
    fn world_set_asset<F, V>(
        &self,
        select_property: F,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        F: Send + Sync + 'static + Fn(&mut A, &V),
        V: Send + Sync + 'static + Clone;
}

impl<A> TargetSetAssetWorldExt<A> for crate::targets::TargetAsset<A>
where
    A: Asset,
{
    fn world_set_asset<F, V>(
        &self,
        set: F,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        F: Send + Sync + 'static + Fn(&mut A, &V),
        V: Send + Sync + 'static + Clone,
    {
        self.set(WorldSetterMarker::new(DynamicSetter::asset(set)))
    }
}

pub trait TargetSetResourceWorldExt: Sized {
    fn world_set_resource<F, R, V>(
        &self,
        set: F,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        F: Send + Sync + 'static + Fn(&mut R, &V),
        R: Resource,
        V: Send + Sync + 'static + Clone;
}

impl TargetSetResourceWorldExt for crate::targets::TargetResource {
    fn world_set_resource<F, R, V>(
        &self,
        set: F,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        F: Send + Sync + 'static + Fn(&mut R, &V),
        R: Resource,
        V: Send + Sync + 'static + Clone,
    {
        self.set(WorldSetterMarker::new(DynamicSetter::resource(set)))
    }
}

pub trait TargetSetHandleComponentWorldExt: Sized {
    fn world_set_component_handle<FH, FP, C, A, V>(
        &self,
        select_handle: FH,
        set: FP,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        FH: Send + Sync + 'static + Fn(&C) -> &Handle<A>,
        FP: Send + Sync + 'static + Fn(&mut A, &V),
        C: Component,
        A: Asset,
        V: Send + Sync + 'static + Clone;
}

impl TargetSetHandleComponentWorldExt for crate::targets::TargetComponent {
    fn world_set_component_handle<FH, FP, C, A, V>(
        &self,
        select_handle: FH,
        set: FP,
    ) -> TargetSetter<Self, WorldSetterMarker<V>>
    where
        FH: Send + Sync + 'static + Fn(&C) -> &Handle<A>,
        FP: Send + Sync + 'static + Fn(&mut A, &V),
        C: Component,
        A: Asset,
        V: Send + Sync + 'static + Clone,
    {
        self.set(WorldSetterMarker::new(DynamicSetter::component_handle(
            select_handle,
            set,
        )))
    }
}

pub struct TargetSetter<T, S> {
    target: T,
    setter: S,
}

impl<T, S> TargetSetter<T, S>
where
    T: Clone + Bundle,
    S: Set + Clone + Bundle,
    S::Value: Send + Sync + 'static,
{
    pub fn curve<C>(&self, duration: Duration, curve: C) -> BuildTween<T, S, C>
    where
        C: Bundle,
    {
        BuildTween {
            duration,
            target: self.target.clone(),
            setter: self.setter.clone(),
            curve,
        }
    }

    pub fn tween<C>(
        &self,
        start: S::Value,
        end: S::Value,
        duration: Duration,
        ease_curve: C,
    ) -> BuildTween<T, S, AToB<S::Value, C>>
    where
        C: Send + Sync + 'static,
    {
        BuildTween {
            duration,
            target: self.target.clone(),
            setter: self.setter.clone(),
            curve: AToB {
                a: start,
                b: end,
                ease_curve,
            },
        }
    }

    pub fn state(self, value: S::Value) -> TargetSetterState<T, S> {
        TargetSetterState {
            target: self.target,
            setter: self.setter,
            state: value,
        }
    }
}

pub struct TargetSetterState<T, S>
where
    S: Set,
{
    target: T,
    setter: S,
    state: S::Value,
}

impl<T, S> TargetSetterState<T, S>
where
    T: Clone + Bundle,
    S: Set + Clone + Bundle,
    S::Value: Clone + Send + Sync + 'static,
{
    pub fn tween<C>(
        &mut self,
        start: S::Value,
        end: S::Value,
        duration: Duration,
        ease_curve: C,
    ) -> BuildTween<T, S, AToB<S::Value, C>>
    where
        C: Send + Sync + 'static,
    {
        self.state = end.clone();
        BuildTween {
            duration,
            target: self.target.clone(),
            setter: self.setter.clone(),
            curve: AToB {
                a: start,
                b: end,
                ease_curve,
            },
        }
    }

    pub fn tween_to<C>(
        &mut self,
        to: S::Value,
        duration: Duration,
        ease_curve: C,
    ) -> BuildTween<T, S, AToB<S::Value, C>>
    where
        C: Send + Sync + 'static,
    {
        let start = std::mem::replace(&mut self.state, to.clone());
        let end = to;
        self.tween(start, end, duration, ease_curve)
    }

    pub fn tween_to_with<C, F>(
        &mut self,
        with: F,
        duration: Duration,
        ease_curve: C,
    ) -> BuildTween<T, S, AToB<S::Value, C>>
    where
        C: Send + Sync + 'static,
        F: FnOnce(&mut S::Value) -> S::Value,
    {
        let start = self.state.clone();
        let end = with(&mut self.state);
        self.tween(start, end, duration, ease_curve)
    }
}

pub struct BuildTween<T: Bundle, S: Set + Bundle, C: Bundle> {
    pub duration: Duration,
    pub target: T,
    pub setter: S,
    pub curve: C,
}

impl<T, S, C> BuildAnimation for BuildTween<T, S, C>
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
