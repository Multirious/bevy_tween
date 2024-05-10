//! Combinators for tweens using this crate's default tweener

use std::time::Duration;

use crate::prelude::TweenEventData;

use bevy::prelude::*;
use bevy_time_runner::TimeSpan;

pub trait AnimationSpawner {
    type SpawnOutput<'o>
    where
        Self: 'o;
    fn spawn(&mut self, bundle: impl Bundle) -> Self::SpawnOutput<'_>;
    fn offset(&self) -> Duration;
    fn set_offset(&mut self, offset: Duration);
}

mod animation_spawner {
    use bevy::ecs::system::EntityCommands;
    use bevy::hierarchy::ChildBuilder;

    use super::*;

    pub struct SystemAnimationSpawner<'r, 'a> {
        child_builder: &'r mut ChildBuilder<'a>,
        offset: Duration,
    }

    impl<'r, 'a> SystemAnimationSpawner<'r, 'a> {
        pub fn new(
            child_builder: &'r mut ChildBuilder<'a>,
        ) -> SystemAnimationSpawner<'r, 'a> {
            SystemAnimationSpawner {
                child_builder,
                offset: Duration::ZERO,
            }
        }
    }

    impl<'r, 'a> AnimationSpawner for SystemAnimationSpawner<'r, 'a> {
        type SpawnOutput<'o> = EntityCommands<'a>
        where Self: 'o;

        fn spawn(&mut self, bundle: impl Bundle) -> Self::SpawnOutput<'_> {
            self.child_builder.spawn(bundle)
        }

        fn offset(&self) -> Duration {
            self.offset
        }

        fn set_offset(&mut self, offset: Duration) {
            self.offset = offset;
        }
    }

    pub struct WorldAnimationSpawner<'r, 'a> {
        world_child_builder: &'r mut WorldChildBuilder<'a>,
        offset: Duration,
    }

    impl<'r, 'a> WorldAnimationSpawner<'r, 'a> {
        pub fn new(
            world_child_builder: &'r mut WorldChildBuilder<'a>,
        ) -> WorldAnimationSpawner<'r, 'a> {
            WorldAnimationSpawner {
                world_child_builder,
                offset: Duration::ZERO,
            }
        }
    }

    impl<'r, 'a> AnimationSpawner for WorldAnimationSpawner<'r, 'a> {
        type SpawnOutput<'o> = EntityWorldMut<'a>
        where Self: 'o;

        fn spawn(&mut self, bundle: impl Bundle) -> Self::SpawnOutput<'_> {
            self.world_child_builder.spawn(bundle)
        }

        fn offset(&self) -> Duration {
            self.offset
        }

        fn set_offset(&mut self, offset: Duration) {
            self.offset = offset;
        }
    }
}
pub use animation_spawner::{SystemAnimationSpawner, WorldAnimationSpawner};

/// Tweens in sequence starting from the lastest offset.
pub fn sequence<A, Tuple>(tuple: Tuple) -> impl FnOnce(&mut A)
where
    A: AnimationSpawner,
    Tuple: SequenceTuple<A>,
{
    move |b| tuple.call_each(b)
}

/// Tweens in parrallel starting from the latest offset.
/// Each tweens will receive the same offset.
/// After finishing this, the last offset will be the tween the gives the furthest
/// offset.
pub fn parallel<A, Tuple>(tuple: Tuple) -> impl FnOnce(&mut A)
where
    A: AnimationSpawner,
    Tuple: ParallelTuple<A>,
{
    move |b| {
        let offset = b.offset();
        let mut furthest_offset = b.offset();
        tuple.call_each_then(b, |b| {
            furthest_offset = if b.offset() > furthest_offset {
                b.offset()
            } else {
                furthest_offset
            };
            go(offset)(b);
        });
        go(furthest_offset)(b);
    }
}

// pub fn chain<S>(state: S) -> Chain<S> {
//     Chain { state }
// }

// pub struct Chain<S> {
//     state: S,
// }

// impl<S> Chain<S> {
//     pub fn next<E: EntitySpawner, F, O>(&mut self, f: F) -> O
//     where
//         F: FnOnce(&mut S) -> O,
//     {
//         f(&mut self.state)
//     }
// }

pub fn tween<I, T, A>(
    duration: Duration,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut A)
where
    I: Bundle,
    T: Bundle,
    A: AnimationSpawner,
{
    move |a| {
        let start = a.offset();
        forward(duration)(a);
        let end = a.offset();
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
        ));
    }
}

pub fn tween_exact<S, I, T, A>(
    span: S,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut A)
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    I: Bundle,
    T: Bundle,
    A: AnimationSpawner,
{
    move |a| {
        a.spawn((span.try_into().unwrap(), interpolation, tween));
    }
}

pub fn tween_event<Data, A>(event: TweenEventData<Data>) -> impl FnOnce(&mut A)
where
    Data: Send + Sync + 'static,
    A: AnimationSpawner,
{
    move |a| {
        a.spawn((TimeSpan::try_from(a.offset()..=a.offset()).unwrap(), event));
    }
}

pub fn tween_event_at<Data, A>(
    at: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut A)
where
    Data: Send + Sync + 'static,
    A: AnimationSpawner,
{
    move |a| {
        a.spawn((TimeSpan::try_from(at..=at).unwrap(), event));
    }
}

pub fn tween_event_for<Data, A>(
    length: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut A)
where
    Data: Send + Sync + 'static,
    A: AnimationSpawner,
{
    move |a| {
        let start = a.offset();
        forward(length)(a);
        let end = a.offset();
        a.spawn((TimeSpan::try_from(start..end).unwrap(), event));
    }
}

pub fn tween_event_exact<S, Data, A>(
    span: S,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut A)
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    Data: Send + Sync + 'static,
    A: AnimationSpawner,
{
    move |a| {
        a.spawn((span.try_into().unwrap(), event));
    }
}

pub fn forward<A>(duration: Duration) -> impl FnOnce(&mut A)
where
    A: AnimationSpawner,
{
    move |a| {
        a.set_offset(a.offset() + duration);
    }
}

pub fn backward<A>(duration: Duration) -> impl FnOnce(&mut A)
where
    A: AnimationSpawner,
{
    move |a| {
        a.set_offset(a.offset().saturating_sub(duration));
    }
}

pub fn go<A>(duration: Duration) -> impl FnOnce(&mut A)
where
    A: AnimationSpawner,
{
    move |a| {
        a.set_offset(duration);
    }
}

/// Tuple of FnOnces in [`sequence()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait SequenceTuple<A: AnimationSpawner>:
    sealed::TupleFnOnceSealed<A, ()>
{
}
impl<T, A> SequenceTuple<A> for T
where
    T: sealed::TupleFnOnceSealed<A, ()>,
    A: AnimationSpawner,
{
}

/// Tuple of FnOnces in [`parallel()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait ParallelTuple<A: AnimationSpawner>:
    sealed::TupleFnOnceSealed<A, ()>
{
}
impl<T, A> ParallelTuple<A> for T
where
    T: sealed::TupleFnOnceSealed<A, ()>,
    A: AnimationSpawner,
{
}
// pub trait ChainTuple<V, A: AnimationSpawner>:
//     sealed::TupleFnOnceSealed<V, Box<dyn FnOnce(&mut TweensBuilder<E>)>>
// {
// }
// impl<T, E, V> ChainTuple<V, E> for T
// where
//     T: sealed::TupleFnOnceSealed<V, Box<dyn FnOnce(&mut TweensBuilder<E>)>>,
//     A: AnimationSpawner,
// {
// }

mod sealed {
    pub(super) trait TupleFnOnceSealed<In, Out> {
        fn call_each(self, a: &mut In);

        fn call_each_and<F>(self, a: &mut In, and: F)
        where
            F: FnMut(Out);

        fn call_each_then<F>(self, a: &mut In, then: F)
        where
            F: FnMut(&mut In);
    }

    impl<In, Out, T: FnOnce(&mut In) -> Out> TupleFnOnceSealed<In, Out> for T {
        fn call_each(self, a: &mut In) {
            self(a);
        }

        fn call_each_and<F>(self, a: &mut In, mut and: F)
        where
            F: FnMut(Out),
        {
            and(self(a));
        }

        fn call_each_then<F>(self, a: &mut In, mut f: F)
        where
            F: FnMut(&mut In),
        {
            self(a);
            f(a);
        }
    }

    macro_rules! impl_TupleFnOnce {
        ($($i:tt $t:ident)+) => {
            impl<
                In, Out,
                $($t: TupleFnOnceSealed<In, Out>,)+
            > TupleFnOnceSealed<In, Out> for ($($t,)*) {
                fn call_each(self, a: &mut In) {
                    $(
                        self.$i.call_each(a);
                    )*
                }

                fn call_each_and<F>(self, a: &mut In, mut and: F)
                where
                    F: FnMut(Out),
                {
                    $(
                        self.$i.call_each_and(a, &mut and);
                    )*
                }

                fn call_each_then<F>(self, a: &mut In, mut then: F)
                where
                    F: FnMut(&mut In)
                {
                    $(
                        self.$i.call_each_then(a, &mut then);
                    )*
                }
            }
        }
    }

    // It's possible to make a macro that use shorter input but i'm tryna make it simple here
    //
    // Built by using Helix macro:
    //
    // xyp<S-F>=;b;vf<S-T>eyp<A-;>i<space>jk;f=;b_<C-a>f<S-T>ev<A-;>l<C-a>
    //
    // starting from
    //
    // impl_TupleFnOnce! { 0 => T0 }

    impl_TupleFnOnce! { 0 T0 }
    impl_TupleFnOnce! { 0 T0 1 T1 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 }
    impl_TupleFnOnce! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 }
}
