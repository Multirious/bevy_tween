//! Combinators for tweens using this crate's default tweener

use std::time::Duration;

use crate::prelude::TweenEventData;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_time_runner::{Repeat, RepeatStyle, TimeRunner, TimeSpan};

mod state {
    use tween::ComponentTween;

    use crate::interpolate::*;
    use crate::tween::{self, TargetComponent, Tween};
    use bevy::prelude::*;

    pub struct TargetState<T, V> {
        target: T,
        value: V,
    }

    impl<T, V> TargetState<T, V> {
        pub fn new(target: T, value: V) -> Self {
            TargetState { target, value }
        }
    }

    impl<T, V> TargetState<T, V>
    where
        T: Clone,
    {
        pub fn with<I>(&mut self, f: impl FnOnce(&mut V) -> I) -> Tween<T, I> {
            let interpolator = f(&mut self.value);
            Tween {
                target: self.target.clone(),
                interpolator,
            }
        }
    }

    pub trait TransformTargetStateExt {
        fn transform_state(&self, value: Transform) -> TransformTargetState;
    }
    impl TransformTargetStateExt for TargetComponent {
        fn transform_state(&self, value: Transform) -> TransformTargetState {
            TransformTargetState::new(self.clone(), value)
        }
    }

    pub struct TransformTargetState {
        target: TargetComponent,
        value: Transform,
    }

    impl TransformTargetState {
        pub fn new(
            target: TargetComponent,
            value: Transform,
        ) -> TransformTargetState {
            TransformTargetState { target, value }
        }

        pub fn transform_with<I>(
            &mut self,
            f: impl FnOnce(&mut Transform) -> I,
        ) -> Tween<TargetComponent, I> {
            let interpolator = f(&mut self.value);
            Tween {
                target: self.target.clone(),
                interpolator,
            }
        }

        pub fn translation_with<I>(
            &mut self,
            f: impl FnOnce(&mut Vec3) -> I,
        ) -> Tween<TargetComponent, I> {
            self.transform_with(|v| f(&mut v.translation))
        }

        pub fn rotation_with<I>(
            &mut self,
            f: impl FnOnce(&mut Quat) -> I,
        ) -> Tween<TargetComponent, I> {
            self.transform_with(|v| f(&mut v.rotation))
        }

        pub fn scale_with<I>(
            &mut self,
            f: impl FnOnce(&mut Vec3) -> I,
        ) -> Tween<TargetComponent, I> {
            self.transform_with(|v| f(&mut v.scale))
        }

        pub fn translation_to(
            &mut self,
            to: Vec3,
        ) -> ComponentTween<Translation> {
            self.translation_with(translation_to(to))
        }

        pub fn rotation_to(&mut self, to: Quat) -> ComponentTween<Rotation> {
            self.rotation_with(rotation_to(to))
        }

        pub fn scale_to(&mut self, to: Vec3) -> ComponentTween<Scale> {
            self.scale_with(scale_to(to))
        }

        pub fn translation_by(
            &mut self,
            by: Vec3,
        ) -> ComponentTween<Translation> {
            self.translation_with(translation_by(by))
        }

        pub fn rotation_by(&mut self, by: Quat) -> ComponentTween<Rotation> {
            self.rotation_with(rotation_by(by))
        }

        pub fn scale_by(&mut self, by: Vec3) -> ComponentTween<Scale> {
            self.scale_with(scale_by(by))
        }
    }
}

pub use state::{TargetState, TransformTargetState, TransformTargetStateExt};

pub struct AnimationSpawner<'r, 'a> {
    child_builder: &'r mut ChildBuilder<'a>,
}

impl<'r, 'a> AnimationSpawner<'r, 'a> {
    pub(crate) fn new(
        child_builder: &'r mut ChildBuilder<'a>,
    ) -> AnimationSpawner<'r, 'a> {
        AnimationSpawner { child_builder }
    }

    pub fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
        self.child_builder.spawn(bundle)
    }
}

/// Extension trait for types that can be used to make an animation.
pub trait AnimationBuilderExt {
    /// Construct [`InsertAnimation`]
    fn animation(&mut self) -> AnimationBuilder<'_>;
}

impl<'a> AnimationBuilderExt for EntityCommands<'a> {
    fn animation(&mut self) -> AnimationBuilder<'_> {
        AnimationBuilder {
            entity_commands: self.reborrow(),
            time_runner: TimeRunner::default(),
        }
    }
}

impl<'w, 's> AnimationBuilderExt for Commands<'w, 's> {
    fn animation(&mut self) -> AnimationBuilder<'_> {
        let entity_commands = self.spawn_empty();
        AnimationBuilder {
            entity_commands,
            time_runner: TimeRunner::default(),
        }
    }
}

impl<'a> AnimationBuilderExt for ChildBuilder<'a> {
    fn animation(&mut self) -> AnimationBuilder<'_> {
        let entity_commands = self.spawn_empty();
        AnimationBuilder {
            entity_commands,
            time_runner: TimeRunner::default(),
        }
    }
}

/// Configure [`TimeRunner`] through a builder API and add animation entities
pub struct AnimationBuilder<'a> {
    entity_commands: EntityCommands<'a>,
    time_runner: TimeRunner,
}
impl<'a> AnimationBuilder<'a> {
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
    pub fn insert<F>(self, animation: F) -> EntityCommands<'a>
    where
        F: FnOnce(&mut AnimationSpawner, Duration) -> Duration,
    {
        let AnimationBuilder {
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

    /// Insert tween components directly to this entity.
    /// Can be used to create a simple animation quickly.
    pub fn insert_tween_here<I, T>(
        self,
        duration: Duration,
        interpolation: I,
        tweens: T,
    ) -> EntityCommands<'a>
    where
        I: Bundle,
        T: Bundle,
    {
        let AnimationBuilder {
            mut entity_commands,
            mut time_runner,
        } = self;
        time_runner.set_length(duration);
        entity_commands.insert((
            TimeSpan::try_from(Duration::ZERO..duration).unwrap(),
            interpolation,
            tweens,
            time_runner,
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
pub fn sequence<S>(
    sequence: S,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    S: Sequence,
{
    move |b, pos| sequence.call(b, pos)
}

/// Animations in parallel.
///
/// Each animation will receive the same starting position.
/// Returns the longest offset from the passed animations.
pub fn parallel<P>(
    parallel: P,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    P: Parallel,
{
    move |b, pos| parallel.call(b, pos)
}

pub fn tween<I, T>(
    duration: Duration,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    I: Bundle,
    T: Bundle,
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

pub fn tween_exact<S, I, T>(
    span: S,
    interpolation: I,
    tween: T,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    I: Bundle,
    T: Bundle,
{
    move |a, pos| {
        a.spawn((span.try_into().unwrap(), interpolation, tween));
        pos
    }
}

pub fn event<Data>(
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        a.spawn((TimeSpan::try_from(pos..=pos).unwrap(), event));
        pos
    }
}

pub fn event_at<Data>(
    at: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        a.spawn((TimeSpan::try_from(at..=at).unwrap(), event));
        pos
    }
}

pub fn event_for<Data>(
    length: Duration,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        let start = pos;
        let end = pos + length;
        a.spawn((TimeSpan::try_from(start..end).unwrap(), event));
        end
    }
}

pub fn event_exact<S, Data>(
    span: S,
    event: TweenEventData<Data>,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        a.spawn((span.try_into().unwrap(), event));
        pos
    }
}

pub fn forward<'a>(
    by: Duration,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration {
    move |_, pos| pos + by
}

pub fn backward(
    by: Duration,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration {
    move |_, pos| pos.saturating_sub(by)
}

pub fn go(
    to: Duration,
) -> impl FnOnce(&mut AnimationSpawner, Duration) -> Duration {
    move |_, _| to
}

/// Tuple of FnOnces in [`sequence()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Sequence: sealed::SequenceSealed {}
impl<T> Sequence for T where T: sealed::SequenceSealed {}

/// Tuple of FnOnces in [`parallel()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Parallel: sealed::ParallelSealed {}
impl<T> Parallel for T where T: sealed::ParallelSealed {}

mod sealed {
    use super::*;

    pub(super) trait SequenceSealed {
        fn call(self, a: &mut AnimationSpawner, pos: Duration) -> Duration;
    }

    impl<T: FnOnce(&mut AnimationSpawner, Duration) -> Duration> SequenceSealed
        for T
    {
        fn call(self, a: &mut AnimationSpawner, pos: Duration) -> Duration {
            self(a, pos)
        }
    }

    pub(super) trait ParallelSealed {
        fn call(self, a: &mut AnimationSpawner, pos: Duration) -> Duration;
    }

    impl<T: FnOnce(&mut AnimationSpawner, Duration) -> Duration> ParallelSealed
        for T
    {
        fn call(self, a: &mut AnimationSpawner, pos: Duration) -> Duration {
            self(a, pos)
        }
    }

    macro_rules! impl_sequence {
        ($($i:tt $t:ident)+) => {
            impl< $($t: SequenceSealed,)+ > SequenceSealed for ($($t,)*) {
                fn call(self, a: &mut AnimationSpawner, pos: Duration) -> Duration {
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
            impl< $($t: ParallelSealed,)+ > ParallelSealed for ($($t,)*) {
                fn call(self, a: &mut AnimationSpawner, start: Duration) -> Duration {
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
