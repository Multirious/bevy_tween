//! Combinators for tweens using this crate's default tweener

use std::time::Duration;

use crate::prelude::TweenEventData;

use super::{EntitySpawner, TimeSpan, TweensBuilder};
use bevy::prelude::*;

/// Tweens in sequence starting from the lastest offset.
pub fn sequence<E, Tuple>(tuple: Tuple) -> impl FnOnce(&mut TweensBuilder<E>)
where
    E: EntitySpawner,
    Tuple: SequenceTuple<E>,
{
    move |b| tuple.call_each(b)
}

/// Tweens in parrallel starting from the latest offset.
/// Each tweens will receive the same offset.
/// After finishing this, the last offset will be the tween the gives the furthest
/// offset.
pub fn parallel<E, Tuple>(tuple: Tuple) -> impl FnOnce(&mut TweensBuilder<E>)
where
    E: EntitySpawner,
    Tuple: ParallelTuple<E>,
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
            b.go(offset);
        });
        b.go(furthest_offset);
    }
}

pub fn tween<I, T, E>(
    duration: Duration,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut TweensBuilder<E>)
where
    I: Bundle,
    T: Bundle,
    E: EntitySpawner,
{
    move |b| {
        let start = b.offset();
        let end = b.forward(duration).offset();
        b.spawn_child((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
        ));
    }
}

pub fn tween_exact<S, I, T, E>(
    span: S,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut TweensBuilder<E>)
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    I: Bundle,
    T: Bundle,
    E: EntitySpawner,
{
    move |b| {
        b.spawn_child((span.try_into().unwrap(), interpolation, tween));
    }
}

pub fn tween_event<Data, E>(
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut TweensBuilder<E>)
where
    Data: Send + Sync + 'static,
    E: EntitySpawner,
{
    move |b| {
        b.spawn_child((
            TimeSpan::try_from(b.offset()..=b.offset()).unwrap(),
            event,
        ));
    }
}

pub fn tween_event_at<Data, E>(
    at: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut TweensBuilder<E>)
where
    Data: Send + Sync + 'static,
    E: EntitySpawner,
{
    move |b| {
        b.spawn_child((TimeSpan::try_from(at..=at).unwrap(), event));
    }
}

pub fn tween_event_for<Data, E>(
    length: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut TweensBuilder<E>)
where
    Data: Send + Sync + 'static,
    E: EntitySpawner,
{
    move |b| {
        let start = b.offset();
        let end = b.forward(length).offset();
        b.spawn_child((TimeSpan::try_from(start..end).unwrap(), event));
    }
}

pub fn tween_event_exact<S, Data, E>(
    span: S,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut TweensBuilder<E>)
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    Data: Send + Sync + 'static,
    E: EntitySpawner,
{
    move |b| {
        b.spawn_child((span.try_into().unwrap(), event));
    }
}

pub fn forward<E>(duration: Duration) -> impl FnOnce(&mut TweensBuilder<E>)
where
    E: EntitySpawner,
{
    move |b| {
        b.forward(duration);
    }
}

pub fn backward<E>(duration: Duration) -> impl FnOnce(&mut TweensBuilder<E>)
where
    E: EntitySpawner,
{
    move |b| {
        b.backward(duration);
    }
}

pub fn go<E>(duration: Duration) -> impl FnOnce(&mut TweensBuilder<E>)
where
    E: EntitySpawner,
{
    move |b| {
        b.go(duration);
    }
}

/// Tuple of FnOnces in [`sequence()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait SequenceTuple<E: EntitySpawner>:
    sealed::TupleFnOnceSealed<TweensBuilder<E>, ()>
{
}
impl<T, E> SequenceTuple<E> for T
where
    T: sealed::TupleFnOnceSealed<TweensBuilder<E>, ()>,
    E: EntitySpawner,
{
}

/// Tuple of FnOnces in [`parallel()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait ParallelTuple<E: EntitySpawner>:
    sealed::TupleFnOnceSealed<TweensBuilder<E>, ()>
{
}
impl<T, E> ParallelTuple<E> for T
where
    T: sealed::TupleFnOnceSealed<TweensBuilder<E>, ()>,
    E: EntitySpawner,
{
}
// pub trait ChainTuple<V, E: EntitySpawner>:
//     sealed::TupleFnOnceSealed<V, Box<dyn FnOnce(&mut TweensBuilder<E>)>>
// {
// }
// impl<T, E, V> ChainTuple<V, E> for T
// where
//     T: sealed::TupleFnOnceSealed<V, Box<dyn FnOnce(&mut TweensBuilder<E>)>>,
//     E: EntitySpawner,
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
