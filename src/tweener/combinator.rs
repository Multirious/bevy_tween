//! Combinators for tweens using this crate's default tweener

use std::time::Duration;

use crate::prelude::TweenEventData;

use super::{EntitySpawner, TimeSpan, TweensBuilder};
use bevy::prelude::*;

/// Tweens in sequence
#[macro_export]
macro_rules! sequence {
    ($($c:expr,)+) => {
        {
            #[allow(clippy::redundant_closure_call)]
            let c = |b: &mut $crate::tweener::TweensBuilder<_>| {
                $($c(b);)+
            };
            c
        }
    };
}

/// Tweens in parrallel using the latest offset.
/// The last offset will be the resulting furthest.
#[macro_export]
macro_rules! parallel {
    ($($c:expr,)+) => {
        {
            #[allow(clippy::redundant_closure_call)]
            let c = |b: &mut $crate::tweener::TweensBuilder<_>| {
                let offset = b.offset();
                let mut furthest_offset = b.offset();
                $(
                    $c(b);
                    furthest_offset = if b.offset() > furthest_offset {
                        b.offset()
                    } else {
                        furthest_offset
                    };
                    b.go(offset);
                )+
                b.go(furthest_offset);

            };
            c
        }
    };
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
