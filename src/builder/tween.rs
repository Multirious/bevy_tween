use std::{any::TypeId, marker::PhantomData, time::Duration};

use bevy::{prelude::*, reflect::ParsedPath};
use bevy_time_runner::TimeSpan;

use crate::{
    curve::AToB,
    set::{DynamicSetter, Set, SetterValue},
    targets::{TargetAsset, TargetComponent, TargetResource},
};

use super::{AnimationCommands, BuildAnimation};

pub trait TargetSetExt<S> {
    type Builder;
    fn set(&self, setter: S) -> Self::Builder;
}

impl<S: Set> TargetSetExt<S> for TargetComponent {
    type Builder = TargetSetter<Self, S, S::Value>;

    fn set(&self, setter: S) -> Self::Builder {
        TargetSetter {
            target: self.clone(),
            setter,
            value_marker: PhantomData,
        }
    }
}
impl<A: Asset, S: Set> TargetSetExt<S> for TargetAsset<A> {
    type Builder = TargetSetter<Self, S, S::Value>;

    fn set(&self, setter: S) -> Self::Builder {
        TargetSetter {
            target: self.clone(),
            setter,
            value_marker: PhantomData,
        }
    }
}
impl<S: Set> TargetSetExt<S> for TargetResource {
    type Builder = TargetSetter<Self, S, S::Value>;

    fn set(&self, setter: S) -> Self::Builder {
        TargetSetter {
            target: self.clone(),
            setter,
            value_marker: PhantomData,
        }
    }
}

pub trait TargetDynamicSetExt {
    type Builder;
    fn dynamic_set(&self) -> Self::Builder;
}

impl TargetDynamicSetExt for TargetComponent {
    type Builder = TargetComponentDynamicSetter;

    fn dynamic_set(&self) -> Self::Builder {
        TargetComponentDynamicSetter {
            target: self.clone(),
        }
    }
}

pub struct TargetComponentDynamicSetter {
    target: TargetComponent,
}

impl TargetComponentDynamicSetter {
    pub fn component<F, C, V>(
        &self,
        set: F,
    ) -> TargetSetter<TargetComponent, DynamicSetter, V>
    where
        F: Send + Sync + 'static + Fn(&mut C, &V),
        C: Component,
        V: Send + Sync + 'static + Clone,
    {
        TargetSetter {
            target: self.target.clone(),
            setter: DynamicSetter::component(set),
            value_marker: PhantomData,
        }
    }

    pub fn component_handle<FH, FP, C, A, V>(
        &self,
        select_handle: FH,
        set: FP,
    ) -> TargetSetter<TargetComponent, DynamicSetter, V>
    where
        FH: Send + Sync + 'static + Fn(&C) -> &Handle<A>,
        FP: Send + Sync + 'static + Fn(&mut A, &V),
        C: Component,
        A: Asset,
        V: Send + Sync + 'static + Clone,
    {
        TargetSetter {
            target: self.target.clone(),
            setter: DynamicSetter::component_handle(select_handle, set),
            value_marker: PhantomData,
        }
    }

    pub fn path<C, V>(
        &self,
        path: ParsedPath,
    ) -> TargetSetter<TargetComponent, DynamicSetter, V>
    where
        C: Component,
        V: Send + Sync + 'static + Clone,
    {
        self.path_raw(path, TypeId::of::<C>(), TypeId::of::<SetterValue<V>>())
    }

    pub fn path_raw<V>(
        &self,
        path: ParsedPath,
        component_type_id: TypeId,
        setter_value_type_id: TypeId,
    ) -> TargetSetter<TargetComponent, DynamicSetter, V> {
        TargetSetter {
            target: self.target.clone(),
            setter: DynamicSetter::component_path(
                path,
                component_type_id,
                setter_value_type_id,
            ),
            value_marker: PhantomData,
        }
    }
}

impl<A: Asset> TargetDynamicSetExt for TargetAsset<A> {
    type Builder = TargetAssetDynamicSetter<A>;

    fn dynamic_set(&self) -> Self::Builder {
        TargetAssetDynamicSetter {
            target: self.clone(),
        }
    }
}

pub struct TargetAssetDynamicSetter<A: Asset> {
    target: TargetAsset<A>,
}

impl<A: Asset> TargetAssetDynamicSetter<A> {
    pub fn asset<F, V>(
        &self,
        set: F,
    ) -> TargetSetter<TargetAsset<A>, DynamicSetter, V>
    where
        F: Send + Sync + 'static + Fn(&mut A, &V),
        V: Send + Sync + 'static + Clone,
    {
        TargetSetter {
            target: self.target.clone(),
            setter: DynamicSetter::asset(set),
            value_marker: PhantomData,
        }
    }
}

impl TargetDynamicSetExt for TargetResource {
    type Builder = TargetResourceDynamicSetter;

    fn dynamic_set(&self) -> Self::Builder {
        TargetResourceDynamicSetter {
            target: self.clone(),
        }
    }
}

pub struct TargetResourceDynamicSetter {
    target: TargetResource,
}

impl TargetResourceDynamicSetter {
    pub fn resource<F, R, V>(
        &self,
        set: F,
    ) -> TargetSetter<TargetResource, DynamicSetter, V>
    where
        F: Send + Sync + 'static + Fn(&mut R, &V),
        V: Send + Sync + 'static + Clone,
        R: Resource,
    {
        TargetSetter {
            target: self.target.clone(),
            setter: DynamicSetter::resource(set),
            value_marker: PhantomData,
        }
    }
}

pub struct TargetSetter<T, S, V> {
    target: T,
    setter: S,
    value_marker: PhantomData<V>,
}

impl<T, S, V> TargetSetter<T, S, V> {
    pub fn new(target: T, setter: S) -> TargetSetter<T, S, V> {
        TargetSetter {
            target,
            setter,
            value_marker: PhantomData,
        }
    }
}

impl<T, S, V> TargetSetter<T, S, V>
where
    T: Clone + Bundle,
    S: Clone + Bundle,
    V: Send + Sync + 'static,
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
        start: V,
        end: V,
        duration: Duration,
        ease_curve: C,
    ) -> BuildTween<T, S, AToB<V, C>>
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

    pub fn state(self, value: V) -> TargetSetterState<T, S, V> {
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
    S: Clone + Bundle,
    V: Clone + Send + Sync + 'static,
{
    pub fn tween<C>(
        &mut self,
        start: V,
        end: V,
        duration: Duration,
        ease_curve: C,
    ) -> BuildTween<T, S, AToB<V, C>>
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
        to: V,
        duration: Duration,
        ease_curve: C,
    ) -> BuildTween<T, S, AToB<V, C>>
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
    ) -> BuildTween<T, S, AToB<V, C>>
    where
        C: Send + Sync + 'static,
        F: FnOnce(&mut V) -> V,
    {
        let start = self.state.clone();
        let end = with(&mut self.state);
        self.tween(start, end, duration, ease_curve)
    }
}

pub struct BuildTween<T: Bundle, S: Bundle, C: Bundle> {
    pub duration: Duration,
    pub target: T,
    pub setter: S,
    pub curve: C,
}

impl<T, S, C> BuildAnimation for BuildTween<T, S, C>
where
    T: Bundle,
    S: Bundle,
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
