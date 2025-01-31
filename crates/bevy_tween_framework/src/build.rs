use std::hash::Hash;
use std::time::Duration;

use bevy_ecs::{bundle::Bundle, entity::Entity, system::EntityCommands};
use bevy_hierarchy::{ChildBuild as _, ChildBuilder};
use bevy_math::{
    curve::{Ease, EaseFunction, EasingCurve},
    Curve,
};
use bevy_time_runner::TimeSpan;
use bevy_tween_core::{argument, Alter};

/// Commands to use within an animation combinator
pub struct AnimationCommands<'r, 'a> {
    child_builder: &'r mut ChildBuilder<'a>,
}

impl<'r, 'a> AnimationCommands<'r, 'a> {
    pub(crate) fn new(
        child_builder: &'r mut ChildBuilder<'a>,
    ) -> AnimationCommands<'r, 'a> {
        AnimationCommands { child_builder }
    }

    /// Spawn an entity as a child.
    /// Currently always spawn as a child of animation root that should contains [`bevy_time_runner::TimeRunner`].
    pub fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
        self.child_builder.spawn(bundle)
    }
}

pub trait BuildAnimation {
    fn build(self, commands: &mut AnimationCommands, position: &mut Duration);
}

impl<F> BuildAnimation for F
where
    F: FnOnce(&mut AnimationCommands, &mut Duration),
{
    fn build(self, commands: &mut AnimationCommands, position: &mut Duration) {
        self(commands, position)
    }
}

#[derive(Clone, Copy)]
pub struct BuildTween<A, C>
where
    A: Alter,
{
    pub duration: Duration,
    pub target: A::Target,
    pub alter: A,
    pub curve: C,
}
impl<A, C> BuildTween<A, C>
where
    A: Alter,
{
    pub fn new(
        duration: Duration,
        target: A::Target,
        alter: A,
        curve: C,
    ) -> Self {
        BuildTween {
            duration,
            target,
            alter,
            curve,
        }
    }
}

impl<A, C> BuildAnimation for BuildTween<A, C>
where
    A: Alter,
    C: Curve<A::Value> + Send + Sync + 'static,
{
    fn build(self, commands: &mut AnimationCommands, position: &mut Duration) {
        let start = *position;
        let end = *position + self.duration;
        commands.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            argument::Target(self.target),
            argument::Alterer(self.alter),
            argument::Curve::new(self.curve),
        ));
        *position = end;
    }
}

#[derive(Clone, Copy)]
pub struct TweenBuilder<Args>(Args);
impl<T> TweenBuilder<Target<T>>
where
    T: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub fn curve_via<A, C>(
        &self,
        via: A,
        curve: C,
        for_duration: Duration,
    ) -> BuildTween<A, C>
    where
        A: Alter<Target = T>,
        C: Curve<A::Value> + Send + Sync + 'static,
    {
        BuildTween {
            duration: for_duration,
            target: self.0.target.clone(),
            alter: via,
            curve,
        }
    }

    pub fn ease_via<A>(
        &self,
        via: A,
        from: A::Value,
        to: A::Value,
        by: EaseFunction,
        for_duration: Duration,
    ) -> BuildTween<A, EasingCurve<A::Value>>
    where
        A: Alter<Target = T>,
        A::Value: Ease,
    {
        self.curve_via(via, EasingCurve::new(from, to, by), for_duration)
    }

    pub fn lerp_via<A>(
        &self,
        via: A,
        from: A::Value,
        to: A::Value,
        for_duration: Duration,
    ) -> BuildTween<A, EasingCurve<A::Value>>
    where
        A: Alter<Target = T>,
        A::Value: Ease,
    {
        self.ease_via(via, from, to, EaseFunction::Linear, for_duration)
    }

    pub fn via<A>(self, via: A) -> TweenBuilder<TargetAlter<A>>
    where
        A: Alter<Target = T>,
    {
        TweenBuilder(TargetAlter {
            target: self.0.target,
            alter: via,
        })
    }
}
impl<A> TweenBuilder<TargetAlter<A>>
where
    A: Alter + Clone,
{
    pub fn curve<C>(&self, curve: C, for_duration: Duration) -> BuildTween<A, C>
    where
        C: Curve<A::Value>,
    {
        BuildTween {
            duration: for_duration,
            target: self.0.target.clone(),
            alter: self.0.alter.clone(),
            curve,
        }
    }

    pub fn ease(
        &self,
        from: A::Value,
        to: A::Value,
        by: EaseFunction,
        for_duration: Duration,
    ) -> BuildTween<A, EasingCurve<A::Value>>
    where
        A::Value: Ease,
    {
        self.curve(EasingCurve::new(from, to, by), for_duration)
    }

    pub fn lerp(
        &self,
        from: A::Value,
        to: A::Value,
        for_duration: Duration,
    ) -> BuildTween<A, EasingCurve<A::Value>>
    where
        A::Value: Ease,
    {
        self.ease(from, to, EaseFunction::Linear, for_duration)
    }

    pub fn ease_from(
        self,
        from: A::Value,
    ) -> TweenBuilder<TargetAlterEaseState<A>> {
        TweenBuilder(TargetAlterEaseState {
            alter: self.0.alter,
            target: self.0.target,
            state: from,
        })
    }
}

impl<A> TweenBuilder<TargetAlterEaseState<A>>
where
    A: Alter + Clone,
{
    pub fn ease_to(
        &mut self,
        to: A::Value,
        by: EaseFunction,
        for_duration: Duration,
    ) -> BuildTween<A, EasingCurve<A::Value>>
    where
        A::Value: Ease,
    {
        let from = std::mem::replace(&mut self.0.state, to.clone());
        BuildTween {
            duration: for_duration,
            target: self.0.target.clone(),
            alter: self.0.alter.clone(),
            curve: EasingCurve::new(from, to, by),
        }
    }

    pub fn lerp_to(
        &mut self,
        to: A::Value,
        for_duration: Duration,
    ) -> BuildTween<A, EasingCurve<A::Value>>
    where
        A::Value: Ease,
    {
        self.ease_to(to, EaseFunction::Linear, for_duration)
    }
}

#[derive(Clone, Copy)]
pub struct Target<T> {
    pub target: T,
}

#[derive(Clone, Copy)]
pub struct TargetAlter<A>
where
    A: Alter,
{
    pub alter: A,
    pub target: A::Target,
}

#[derive(Clone, Copy)]
pub struct TargetAlterEaseState<A>
where
    A: Alter,
{
    pub alter: A,
    pub target: A::Target,
    pub state: A::Value,
}

pub trait TweenBuilderExt: Eq + Hash + Clone + Send + Sync + 'static {
    fn tween(&self) -> TweenBuilder<Target<Self>>;
    fn tween_via<A>(&self, via: A) -> TweenBuilder<TargetAlter<A>>
    where
        A: Alter<Target = Self>;
}
impl TweenBuilderExt for Entity {
    fn tween(&self) -> TweenBuilder<Target<Self>> {
        TweenBuilder(Target { target: *self })
    }

    fn tween_via<A>(&self, via: A) -> TweenBuilder<TargetAlter<A>>
    where
        A: Alter<Target = Self>,
    {
        TweenBuilder(TargetAlter {
            target: *self,
            alter: via,
        })
    }
}
