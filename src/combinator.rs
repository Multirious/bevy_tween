//! Combinators for tweens using this crate's default tweener

use std::time::Duration;

use crate::prelude::TweenEventData;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_time_runner::{Repeat, RepeatStyle, TimeRunner, TimeSpan};

/// Trait for types that can spawn an entity regards to animation.
pub trait SpawnAnimation {
    /// Output from [`Self::spawn`].
    type SpawnOutput<'o>
    where
        Self: 'o;

    /// Spawn an entity.
    fn spawn(&mut self, bundle: impl Bundle) -> Self::SpawnOutput<'_>;
}

pub struct AnimationSpawner<'r, 'a> {
    child_builder: &'r mut ChildBuilder<'a>,
}

impl<'r, 'a> AnimationSpawner<'r, 'a> {
    pub(crate) fn new(
        child_builder: &'r mut ChildBuilder<'a>,
    ) -> AnimationSpawner<'r, 'a> {
        AnimationSpawner { child_builder }
    }
}

impl<'r, 'a> SpawnAnimation for AnimationSpawner<'r, 'a> {
    type SpawnOutput<'o> = EntityCommands<'o>
        where Self: 'o;

    fn spawn(&mut self, bundle: impl Bundle) -> Self::SpawnOutput<'_> {
        self.child_builder.spawn(bundle)
    }
}

// pub struct WorldAnimationSpawner<'r, 'a> {
//     world_child_builder: &'r mut WorldChildBuilder<'a>,
// }

// impl<'r, 'a> WorldAnimationSpawner<'r, 'a> {
//     pub(crate) fn new(
//         world_child_builder: &'r mut WorldChildBuilder<'a>,
//     ) -> WorldAnimationSpawner<'r, 'a> {
//         WorldAnimationSpawner {
//             world_child_builder,
//         }
//     }
// }

// impl<'r, 'a> SpawnAnimation for WorldAnimationSpawner<'r, 'a> {
//     type SpawnOutput<'o> = EntityWorldMut<'o>
//         where Self: 'o;

//     fn spawn(&mut self, bundle: impl Bundle) -> Self::SpawnOutput<'_> {
//         self.world_child_builder.spawn(bundle)
//     }
// }

/// Extension trait for types that can be used to make an animation.
pub trait InsertAnimationExt {
    /// Construct [`InsertAnimation`]
    fn insert_animation(&mut self) -> InsertAnimation<'_>;
}

impl<'a> InsertAnimationExt for EntityCommands<'a> {
    fn insert_animation(&mut self) -> InsertAnimation<'_> {
        InsertAnimation {
            entity_commands: self.reborrow(),
            time_runner: TimeRunner::default(),
        }
    }
}

impl<'w, 's> InsertAnimationExt for Commands<'w, 's> {
    fn insert_animation(&mut self) -> InsertAnimation<'_> {
        let entity_commands = self.spawn_empty();
        InsertAnimation {
            entity_commands,
            time_runner: TimeRunner::default(),
        }
    }
}

impl<'a> InsertAnimationExt for ChildBuilder<'a> {
    fn insert_animation(&mut self) -> InsertAnimation<'_> {
        let entity_commands = self.spawn_empty();
        InsertAnimation {
            entity_commands,
            time_runner: TimeRunner::default(),
        }
    }
}

/// Configure [`TimeRunner`] through a builder API and add animation entities
pub struct InsertAnimation<'a> {
    entity_commands: EntityCommands<'a>,
    time_runner: TimeRunner,
}
impl<'a> InsertAnimation<'a> {
    /// Configure [`TimeRunner`]'s [`Repeat`]
    pub fn repeat(mut self, repeat: Repeat) -> Self {
        match self.time_runner.repeat() {
            Some((_, repeat_style)) => {
                self.time_runner.set_repeat(Some((repeat, repeat_style)))
            }
            None => self
                .time_runner
                .set_repeat(Some((repeat, RepeatStyle::default()))),
        }
        self
    }

    /// Configure [`TimeRunner`]'s [`RepeatStyle`]
    pub fn repeat_style(mut self, repeat_style: RepeatStyle) -> Self {
        match self.time_runner.repeat() {
            Some((repeat, _)) => {
                self.time_runner.set_repeat(Some((repeat, repeat_style)))
            }
            None => self
                .time_runner
                .set_repeat(Some((Repeat::Infinitely, repeat_style))),
        }
        self
    }

    /// Add animations from a closure. Animation entities will be subjected
    /// as a children of this entity.
    pub fn animate<F>(self, animation: F) -> EntityCommands<'a>
    where
        F: FnOnce(&mut AnimationSpawner, Duration) -> Duration,
    {
        let InsertAnimation {
            mut entity_commands,
            mut time_runner,
        } = self;
        let mut dur = Duration::ZERO;
        entity_commands.with_children(|c| {
            let mut a = AnimationSpawner::new(c);
            dur = animation(&mut a, dur);
        });
        time_runner.set_length(dur);
        entity_commands.insert(time_runner);
        entity_commands
    }

    /// Add animations directly to this entity.
    /// No children will be added like in [`MakeAnimation::animate`].
    /// Has less entities as a result but cannot create complex animations.
    pub fn animate_here<I, T>(
        self,
        duration: Duration,
        interpolation: I,
        tweens: T,
    ) -> EntityCommands<'a>
    where
        I: Bundle,
        T: Bundle,
    {
        let InsertAnimation {
            mut entity_commands,
            mut time_runner,
        } = self;
        time_runner.set_length(duration);
        entity_commands.insert((
            TimeSpan::try_from(Duration::ZERO..duration).unwrap(),
            interpolation,
            tweens,
        ));
        entity_commands
    }
}

// fn test_system(mut commands: Commands) {
//     use crate::{interpolate::translation, prelude::*, tween::TargetComponent};

//     let my_entity = commands.spawn_empty().id();
//     let target = TargetComponent::Entity(my_entity);
//     commands
//         .make_animation()
//         .repeat(Repeat::Infinitely)
//         .animate(|a, pos| {
//             let walk = || {
//                 tween(
//                     Duration::from_secs(1),
//                     EaseFunction::Linear,
//                     target.with(translation(Vec3::ZERO, Vec3::ONE)),
//                 )
//             };
//             sequence((walk(), walk()))(a, pos)
//         });

//     let target = TargetComponent::TweenerEntity;
//     let my_entity = commands
//         .spawn_empty()
//         .make_animation()
//         .repeat(Repeat::Infinitely)
//         .animate(|a, pos| {
//             let walk = || {
//                 tween(
//                     Duration::from_secs(1),
//                     EaseFunction::Linear,
//                     target.with(translation(Vec3::ZERO, Vec3::ONE)),
//                 )
//             };
//             sequence((walk(), walk()))(a, pos)
//         });

//     let target = TargetComponent::TweenerEntity;
//     let my_entity = commands.spawn_empty().with_children(|c| {
//         c.make_animation()
//             .repeat(Repeat::Infinitely)
//             .animate(|a, pos| {
//                 let walk = || {
//                     tween(
//                         Duration::from_secs(1),
//                         EaseFunction::Linear,
//                         target.with(translation(Vec3::ZERO, Vec3::ONE)),
//                     )
//                 };
//                 sequence((walk(), walk()))(a, pos)
//             });
//     });
// }

/// Animations in sequence.
///
/// Each animation output will be passed to the next one.
/// Returns position from the last animation.
pub fn sequence<A, S>(sequence: S) -> impl FnOnce(&mut A, Duration) -> Duration
where
    A: SpawnAnimation,
    S: Sequence<A>,
{
    move |b, pos| sequence.call(b, pos)
}

/// Animations in parallel.
///
/// Each animation will receive the same starting position.
/// Returns the longest offset from the passed animations.
pub fn parallel<A, P>(parallel: P) -> impl FnOnce(&mut A, Duration) -> Duration
where
    A: SpawnAnimation,
    P: Parallel<A>,
{
    move |b, pos| parallel.call(b, pos)
}

pub fn tween<I, T, A>(
    duration: Duration,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut A, Duration) -> Duration
where
    I: Bundle,
    T: Bundle,
    A: SpawnAnimation,
{
    move |a, pos| {
        let start = pos;
        let end = pos + duration;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            interpolation,
            tween,
        ));
        end
    }
}

pub fn tween_exact<S, I, T, A>(
    span: S,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut A, Duration) -> Duration
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    I: Bundle,
    T: Bundle,
    A: SpawnAnimation,
{
    move |a, pos| {
        a.spawn((span.try_into().unwrap(), interpolation, tween));
        pos
    }
}

pub fn event<Data, A>(
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut A, Duration) -> Duration
where
    Data: Send + Sync + 'static,
    A: SpawnAnimation,
{
    move |a, pos| {
        a.spawn((TimeSpan::try_from(pos..=pos).unwrap(), event));
        pos
    }
}

pub fn event_at<Data, A>(
    at: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut A, Duration) -> Duration
where
    Data: Send + Sync + 'static,
    A: SpawnAnimation,
{
    move |a, pos| {
        a.spawn((TimeSpan::try_from(at..=at).unwrap(), event));
        pos
    }
}

pub fn event_for<Data, A>(
    length: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut A, Duration) -> Duration
where
    Data: Send + Sync + 'static,
    A: SpawnAnimation,
{
    move |a, pos| {
        let start = pos;
        let end = pos + length;
        a.spawn((TimeSpan::try_from(start..end).unwrap(), event));
        end
    }
}

pub fn event_exact<S, Data, A>(
    span: S,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut A, Duration) -> Duration
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    Data: Send + Sync + 'static,
    A: SpawnAnimation,
{
    move |a, pos| {
        a.spawn((span.try_into().unwrap(), event));
        pos
    }
}

pub fn forward<'a, A>(by: Duration) -> impl FnOnce(&mut A, Duration) -> Duration
where
    A: SpawnAnimation,
{
    move |_, pos| pos + by
}

pub fn backward<A>(by: Duration) -> impl FnOnce(&mut A, Duration) -> Duration
where
    A: SpawnAnimation,
{
    move |_, pos| pos.saturating_sub(by)
}

pub fn go<A>(to: Duration) -> impl FnOnce(&mut A, Duration) -> Duration
where
    A: SpawnAnimation,
{
    move |_, _| to
}

/// Tuple of FnOnces in [`sequence()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Sequence<A>: sealed::SequenceSealed<A>
where
    A: SpawnAnimation,
{
}
impl<T, A> Sequence<A> for T
where
    T: sealed::SequenceSealed<A>,
    A: SpawnAnimation,
{
}

/// Tuple of FnOnces in [`parallel()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Parallel<A>: sealed::ParallelSealed<A>
where
    A: SpawnAnimation,
{
}
impl<T, A> Parallel<A> for T
where
    T: sealed::ParallelSealed<A>,
    A: SpawnAnimation,
{
}
mod sealed {
    use super::*;

    pub(super) trait SequenceSealed<A> {
        fn call(self, a: &mut A, pos: Duration) -> Duration;
    }

    impl<A, T: FnOnce(&mut A, Duration) -> Duration> SequenceSealed<A> for T {
        fn call(self, a: &mut A, pos: Duration) -> Duration {
            self(a, pos)
        }
    }

    pub(super) trait ParallelSealed<A> {
        fn call(self, a: &mut A, pos: Duration) -> Duration;
    }

    impl<A, T: FnOnce(&mut A, Duration) -> Duration> ParallelSealed<A> for T {
        fn call(self, a: &mut A, pos: Duration) -> Duration {
            self(a, pos)
        }
    }

    macro_rules! impl_sequence {
        ($($i:tt $t:ident)+) => {
            impl<
                A, $($t: SequenceSealed<A>,)+
            > SequenceSealed<A> for ($($t,)*) {
                fn call(self, a: &mut A, pos: Duration) -> Duration {
                    $(
                        let pos = self.$i.call(a, pos);
                    )*
                    pos
                }
            }
        }
    }
    macro_rules! impl_parallel {
        ($($i:tt $t:ident)+) => {
            impl<
                A, $($t: ParallelSealed<A>,)+
            > ParallelSealed<A> for ($($t,)*) {
                fn call(self, a: &mut A, start: Duration) -> Duration {
                    let mut furthest = start;
                    $(
                        let pos = self.$i.call(a, start);
                        if pos > furthest {
                            furthest = pos;
                        }
                    )*
                    furthest
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

    impl_sequence! { 0 T0 }
    impl_sequence! { 0 T0 1 T1 }
    impl_sequence! { 0 T0 1 T1 2 T2 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 }
    impl_sequence! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 }

    impl_parallel! { 0 T0 }
    impl_parallel! { 0 T0 1 T1 }
    impl_parallel! { 0 T0 1 T1 2 T2 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 }
    impl_parallel! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 }
}
