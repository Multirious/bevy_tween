//! Combinators for tweens using this crate's default tweener

use std::time::Duration;

use crate::prelude::TweenEventData;

use super::{EntitySpawner, TimeSpan, TweensBuilder};
use bevy::prelude::*;

/// Tweens in sequence starting from the lastest offset.
pub fn sequence<E: EntitySpawner, S: TupleFnOnce<E>>(
    sequence: S,
) -> impl FnOnce(&mut TweensBuilder<E>) {
    move |b| sequence.call_each(b)
}

/// Tweens in parrallel starting from the latest offset.
/// Each tweens will receive the same offset.
/// After finishing this, the last offset will be the tween the gives the furthest
/// offset.
pub fn parallel<E: EntitySpawner, S: TupleFnOnce<E>>(
    sequence: S,
) -> impl FnOnce(&mut TweensBuilder<E>) {
    move |b| {
        let offset = b.offset();
        let mut furthest_offset = b.offset();
        sequence.call_each_then(b, |b| {
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

/// Tuple of FnOnces, support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait TupleFnOnce<E: EntitySpawner>: sealed::TupleFnOnceSealed<E> {}

impl<E: EntitySpawner, T> TupleFnOnce<E> for T where
    T: sealed::TupleFnOnceSealed<E>
{
}

mod sealed {
    use super::*;

    pub(super) trait TupleFnOnceSealed<E: EntitySpawner> {
        fn call_each(self, b: &mut TweensBuilder<E>);

        fn call_each_then<F>(self, b: &mut TweensBuilder<E>, f: F)
        where
            F: FnMut(&mut TweensBuilder<E>);
    }

    impl<E: EntitySpawner, T: for<'a> FnOnce(&'a mut TweensBuilder<E>)>
        TupleFnOnceSealed<E> for T
    {
        fn call_each(self, b: &mut TweensBuilder<E>) {
            self(b);
        }

        fn call_each_then<F>(self, b: &mut TweensBuilder<E>, mut f: F)
        where
            F: FnMut(&mut TweensBuilder<E>),
        {
            self(b);
            f(b);
        }
    }

    macro_rules! impl_TupleFnOnce {
        ($($i:tt $t:ident)+) => {
            impl<
                E: EntitySpawner,
                $($t: TupleFnOnceSealed<E>,)+
            > TupleFnOnceSealed<E> for ($($t,)*) {
                fn call_each(self, b: &mut TweensBuilder<E>) {
                    $(
                        self.$i.call_each(b);
                    )*
                }

                fn call_each_then<F>(self, b: &mut TweensBuilder<E>, mut f: F)
                where
                    F: FnMut(&mut TweensBuilder<E>)
                {
                    $(
                        self.$i.call_each_then(b, &mut f);
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
