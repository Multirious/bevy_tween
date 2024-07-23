use super::AnimationCommands;
use std::time::Duration;

/// Animations in sequence.
///
/// Each animation output will be passed to the next one.
/// Returns position from the last animation.
pub fn sequence<S>(
    sequence: S,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    S: Sequence,
{
    move |b, pos| sequence.build(b, pos)
}

/// Animations in parallel.
///
/// Each animation will receive the same starting position.
/// Returns the longest offset from the passed animations.
pub fn parallel<P>(
    parallel: P,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    P: Parallel,
{
    move |b, pos| parallel.build(b, pos)
}

// /// Combinator for creating a basic tween using interpolation and a tween.
// ///
// /// Starts from last position and tween for provided `duration`
// ///
// /// Position is shifted to this tween's end.
// pub fn tween<I, T>(
//     duration: Duration,
//     interpolation: I,
//     tween: T,
// ) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
// where
//     I: Bundle,
//     T: Bundle,
// {
//     move |a, pos| {
//         let start = *pos;
//         let end = start + duration;
//         a.spawn((
//             TimeSpan::try_from(start..end).unwrap(),
//             interpolation,
//             tween,
//         ));
//         *pos = end;
//     }
// }

// /// Combinator for creating a basic tween using interpolation and a tween.
// ///
// /// Starts and ends at provided span.
// ///
// /// Position is not mutated because the operation is not relative.
// pub fn tween_exact<S, I, T>(
//     span: S,
//     interpolation: I,
//     tween: T,
// ) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
// where
//     S: TryInto<TimeSpan>,
//     S::Error: std::fmt::Debug,
//     I: Bundle,
//     T: Bundle,
// {
//     move |a, _pos| {
//         a.spawn((span.try_into().unwrap(), interpolation, tween));
//     }
// }

/// Shift the position forward by provided duration
pub fn forward(
    by: Duration,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |_, pos| *pos += by
}

/// Shift the position backward by provided duration
pub fn backward(
    by: Duration,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |_, pos| *pos = pos.saturating_sub(by)
}

/// Set the position to the provided duration
pub fn go(to: Duration) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |_, pos| *pos = to
}

/// Tuple of [`BuildAnimation`](super::BuildAnimation) in [`sequence()`],
/// support up to 16 indexes but can be nested indefinitely.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Sequence: sealed::SequenceSealed {}
impl<T> Sequence for T where T: sealed::SequenceSealed {}

/// Tuple of [`BuildAnimation`](super::BuildAnimation) in [`parallel()`],
/// support up to 16 indexes but can be nested indefinitely.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait Parallel: sealed::ParallelSealed {}
impl<T> Parallel for T where T: sealed::ParallelSealed {}

mod sealed {
    use super::super::BuildAnimation;
    use super::*;

    pub(super) trait SequenceSealed {
        fn build(self, a: &mut AnimationCommands, pos: &mut Duration);
    }

    impl<T: BuildAnimation> SequenceSealed for T {
        fn build(self, a: &mut AnimationCommands, pos: &mut Duration) {
            self.build(a, pos)
        }
    }

    pub(super) trait ParallelSealed {
        fn build(self, a: &mut AnimationCommands, pos: &mut Duration);
    }

    impl<T: BuildAnimation> ParallelSealed for T {
        fn build(self, a: &mut AnimationCommands, pos: &mut Duration) {
            self.build(a, pos)
        }
    }

    macro_rules! impl_sequence {
        ($($i:tt $t:ident)+) => {
            impl< $($t: SequenceSealed,)+ > SequenceSealed for ($($t,)*) {
                fn build(self, a: &mut AnimationCommands, pos: &mut Duration) {
                    $(
                        self.$i.build(a, pos);
                    )*
                }
            }
        }
    }
    macro_rules! impl_parallel {
        ($($i:tt $t:ident)+) => {
            impl< $($t: ParallelSealed,)+ > ParallelSealed for ($($t,)*) {
                fn build(self, a: &mut AnimationCommands, main_pos: &mut Duration) {
                    let mut furthest = *main_pos;
                    let mut pos = *main_pos;
                    $(
                        self.$i.build(a, &mut pos);
                        if pos > furthest {
                            furthest = pos;
                        }
                        #[allow(unused)]
                        {pos = *main_pos;}
                    )*
                    *main_pos = furthest;
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
