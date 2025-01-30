use std::time::Duration;

use crate::build::{AnimationCommands, BuildAnimation};

/// Animations in sequence.
///
/// Each animation output will be passed to the next one.
/// Returns position from the last animation.
pub fn sequence<S>(sequence: S) -> Sequence<S>
where
    S: SequenceTuple,
{
    Sequence(sequence)
}

#[derive(Clone, Copy)]
pub struct Sequence<S: SequenceTuple>(S);

impl<S: SequenceTuple> BuildAnimation for Sequence<S> {
    fn build(self, commands: &mut AnimationCommands, position: &mut Duration) {
        self.0.call(commands, position);
    }
}

/// Animations in parallel.
///
/// Each animation will receive the same starting position.
/// Returns the longest offset from the passed animations.
pub fn parallel<P>(parallel: P) -> Parallel<P>
where
    P: ParallelTuple,
{
    Parallel(parallel)
}

#[derive(Clone, Copy)]
pub struct Parallel<P: ParallelTuple>(P);

impl<P: ParallelTuple> BuildAnimation for Parallel<P> {
    fn build(self, commands: &mut AnimationCommands, position: &mut Duration) {
        self.0.call(commands, position);
    }
}

/// Shift the position forward by provided duration
pub fn forward(by: Duration) -> Forward {
    Forward(by)
}

#[derive(Clone, Copy)]
pub struct Forward(Duration);

impl BuildAnimation for Forward {
    fn build(self, _: &mut AnimationCommands, position: &mut Duration) {
        *position += self.0;
    }
}

/// Shift the position backward by provided duration
pub fn backward(by: Duration) -> Backward {
    Backward(by)
}

#[derive(Clone, Copy)]
pub struct Backward(Duration);

impl BuildAnimation for Backward {
    fn build(self, _: &mut AnimationCommands, position: &mut Duration) {
        *position = position.saturating_sub(self.0);
    }
}

/// Set the position to the provided duration
pub fn go(to: Duration) -> Go {
    Go(to)
}

#[derive(Clone, Copy)]
pub struct Go(Duration);

impl BuildAnimation for Go {
    fn build(self, _: &mut AnimationCommands, position: &mut Duration) {
        *position = self.0;
    }
}

/// Tuple of FnOnces in [`sequence()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait SequenceTuple: sealed::SequenceTupleSealed {}
impl<T> SequenceTuple for T where T: sealed::SequenceTupleSealed {}

/// Tuple of FnOnces in [`parallel()`],
/// support up to 16 indexes but can be circumvented by nesting tuples.
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
#[allow(private_bounds)]
pub trait ParallelTuple: sealed::ParallelTupleSealed {}
impl<T> ParallelTuple for T where T: sealed::ParallelTupleSealed {}

mod sealed {
    use std::time::Duration;

    use crate::build::{AnimationCommands, BuildAnimation};

    pub(super) trait SequenceTupleSealed {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration);
    }

    impl<T: BuildAnimation> SequenceTupleSealed for T {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration) {
            self.build(a, pos)
        }
    }

    pub(super) trait ParallelTupleSealed {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration);
    }

    impl<T: BuildAnimation> ParallelTupleSealed for T {
        fn call(self, a: &mut AnimationCommands, pos: &mut Duration) {
            self.build(a, pos)
        }
    }

    macro_rules! impl_sequence {
        ($($i:tt $t:ident)+) => {
            impl< $($t: SequenceTupleSealed,)+ > SequenceTupleSealed for ($($t,)*) {
                fn call(self, a: &mut AnimationCommands, pos: &mut Duration) {
                    $(
                        self.$i.call(a, pos);
                    )*
                }
            }
        }
    }
    macro_rules! impl_parallel {
        ($($i:tt $t:ident)+) => {
            impl< $($t: ParallelTupleSealed,)+ > ParallelTupleSealed for ($($t,)*) {
                fn call(self, a: &mut AnimationCommands, main_pos: &mut Duration) {
                    let mut furthest = *main_pos;
                    let mut pos = *main_pos;
                    $(
                        self.$i.call(a, &mut pos);
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
