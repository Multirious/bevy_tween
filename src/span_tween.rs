//! Module containing span tween implementation
//!
//! # Span tween
//!
//! **Components**:
//! - [`SpanTweener`]
//! - [`TweenTimeSpan`]
//!
//! **Bundles**:
//! - [`SpanTweenerBundle`]
//! - [`SpanTweenBundle`]
//!
//! **Events**:
//! - [`SpanTweenerEnded`]
//!
//! ## Entity structure
//!
//! If we have this entity:
//!   ```no_run
//!   # use bevy::prelude::*;
//!   # use bevy_tween::prelude::*;
//!   # let world = World::new();
//!   # let mut commands_queue = bevy::ecs::system::CommandQueue::default();
//!   # let mut commands = Commands::new(&mut commands_queue, &world);
//!   let my_entity = commands.spawn(SpriteBundle::default()).id();
//!   ```
//!  
//!   We can create a span tweener with span tween in 2 ways:
//! - Span tween in the same entity as a span tweener.<br/>
//!   This is the case where you might want to make a simple animation where
//!   there's not many parameteres. Because an entity can only have one unique
//!   component, it limits on what animation you can achieve with this.
//!   ```no_run
//!   # use bevy::prelude::*;
//!   # use bevy_tween::prelude::*;
//!   # let world = World::new();
//!   # let mut commands_queue = bevy::ecs::system::CommandQueue::default();
//!   # let mut commands = Commands::new(&mut commands_queue, &world);
//!   # let my_entity = commands.spawn(SpriteBundle::default()).id();
//!   // Spawning some span tweener
//!   commands.spawn((
//!       // The span tweener:
//!       SpanTweenerBundle::new(Duration::from_secs(1)),
//!       // The tween:
//!       // Tween this from the start to the second 1.
//!       SpanTweenBundle::new(..Duration::from_secs(1)),
//!       // Tween this with ease quadratic out.
//!       EaseFunction::QuadraticOut,
//!       // Tween a component.
//!       ComponentTween::new_target(
//!           // Tween the component of this entity
//!           my_entity,
//!           // Tween transform's translation of the entity
//!           interpolate::Translation {
//!               start: Vec3::new(0., 0., 0.),
//!               end: Vec3::new(0., 100., 0.),
//!           }
//!       )
//!   ));
//!   ```
//! - Span tween(s) as a child of a span tweener.<br/>
//!   This is the case where you want to make a more complex animation. By having
//!   span tweens as span tweener's children, you can have any number of
//!   span tween types you wanted .
//!   ```no_run
//!   # use bevy::prelude::*;
//!   # use bevy_tween::prelude::*;
//!   # let world = World::new();
//!   # let mut commands_queue = bevy::ecs::system::CommandQueue::default();
//!   # let mut commands = Commands::new(&mut commands_queue, &world);
//!   # let my_entity = commands.spawn(SpriteBundle::default()).id();
//!   // Spawning some span tweener
//!   commands.spawn(
//!       // The span tweener:
//!       SpanTweenerBundle::new(Duration::from_secs(1)),
//!   ).with_children(|c| {
//!       // The span tween:
//!       c.spawn((
//!           SpanTweenBundle::new(..Duration::from_secs(1)),
//!           EaseFunction::QuadraticOut,
//!           ComponentTween::new_target(
//!               my_entity,
//!               interpolate::Translation {
//!                   start: Vec3::new(0., 0., 0.),
//!                   end: Vec3::new(0., 100., 0.),
//!               }
//!           )
//!       ));
//!      // spawn some more span tween if needed.
//!      // c.spawn( ... );
//!   });
//!   ```
//! - Also the above 2 combined will works just fine btw.

use std::{cmp::Ordering, ops, time::Duration};

use bevy::{ecs::system::EntityCommands, prelude::*};
use tween_timer::{Repeat, RepeatStyle};

use crate::{
    interpolation::Interpolation,
    prelude::EaseFunction,
    tween::{SkipTweener, TweenProgressed, TweenerMarker},
    tween_timer::{self, AnimationDirection, TickResult, TweenTimer},
};

/// Plugin for using span tween
#[derive(Debug)]
pub struct SpanTweenPlugin;
impl Plugin for SpanTweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                tick_span_tweener_system
                    .in_set(crate::TweenSystemSet::TickTweener),
                span_tweener_system.in_set(crate::TweenSystemSet::Tweener),
            ),
        )
        .register_type::<SpanTweener>()
        .register_type::<TimeBound>()
        .register_type::<TweenTimeSpan>()
        .add_event::<SpanTweenerEnded>();
    }
}

/// Span tweener
#[derive(Debug, Default, Component, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct SpanTweener {
    /// The inner timer
    pub timer: TweenTimer,
}

impl From<TweenTimer> for SpanTweener {
    fn from(value: TweenTimer) -> Self {
        SpanTweener { timer: value }
    }
}

/// Bounding enum for [`Duration`] to be exclusivively checked or inclusivively
/// checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TimeBound {
    /// Inclusively check this duration
    Inclusive(Duration),
    /// Exclusively check this duration
    Exclusive(Duration),
}

impl TimeBound {
    /// Get the inner duration
    pub fn duration(&self) -> Duration {
        match self {
            TimeBound::Inclusive(d) | TimeBound::Exclusive(d) => *d,
        }
    }
}

impl Default for TimeBound {
    fn default() -> Self {
        TimeBound::Inclusive(Duration::ZERO)
    }
}

#[derive(Debug, Clone, Copy)]
enum DurationQuotient {
    Before,
    Inside,
    After,
}

/// Error type for when creating a new [`TweenTimeSpan`].
#[derive(Debug)]
pub enum NewTweenTimeSpanError {
    /// The provided min, max will result in a [`TweenTimeSpan`] that does not
    /// appear on a timeline
    NotTime {
        #[allow(missing_docs)]
        min: TimeBound,
        #[allow(missing_docs)]
        max: TimeBound,
    },
    /// The provided min is greater than max and it's not allowed.
    MinGreaterThanMax {
        #[allow(missing_docs)]
        min: TimeBound,
        #[allow(missing_docs)]
        max: TimeBound,
    },
}

impl std::error::Error for NewTweenTimeSpanError {}
impl std::fmt::Display for NewTweenTimeSpanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewTweenTimeSpanError::NotTime { min, max } => {
                write!(f, "This span does not contain any time: min {min:?} max {max:?}")
            }
            NewTweenTimeSpanError::MinGreaterThanMax { min, max } => {
                write!(f, "This span has min greater than max: min {min:?} max {max:?}")
            }
        }
    }
}

/// Define the range of time for a span tween that will be interpolating for.
#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TweenTimeSpan {
    /// Minimum time for the span tween.
    min: TimeBound,
    /// Maximum time for the span tween.
    max: TimeBound,
}
impl TweenTimeSpan {
    /// Create a new [`TweenTimeSpan`] unchecked for invalid min, max.
    pub(crate) fn new_unchecked(
        min: TimeBound,
        max: TimeBound,
    ) -> TweenTimeSpan {
        TweenTimeSpan { min, max }
    }

    /// Create a new [`TweenTimeSpan`]
    pub fn new(
        min: TimeBound,
        max: TimeBound,
    ) -> Result<TweenTimeSpan, NewTweenTimeSpanError> {
        if matches!(
            (min, max),
            (TimeBound::Exclusive(_), TimeBound::Exclusive(_))
        ) && min.duration() == max.duration()
        {
            return Err(NewTweenTimeSpanError::NotTime { min, max });
        } else if min.duration() > max.duration() {
            return Err(NewTweenTimeSpanError::MinGreaterThanMax { min, max });
        }
        Ok(Self::new_unchecked(min, max))
    }

    fn quotient(&self, duration: Duration) -> DurationQuotient {
        let after_min = match self.min {
            TimeBound::Inclusive(min) => duration >= min,
            TimeBound::Exclusive(min) => duration > min,
        };
        let before_max = match self.max {
            TimeBound::Inclusive(max) => duration <= max,
            TimeBound::Exclusive(max) => duration < max,
        };
        match (after_min, before_max) {
            (true, true) => DurationQuotient::Inside,
            (true, false) => DurationQuotient::After,
            (false, true) => DurationQuotient::Before,
            (false, false) => unreachable!(),
        }
    }

    /// Get the min time
    pub fn min(&self) -> TimeBound {
        self.min
    }

    /// Get the max time
    pub fn max(&self) -> TimeBound {
        self.max
    }
}

impl Default for TweenTimeSpan {
    fn default() -> Self {
        TweenTimeSpan::try_from(Duration::ZERO..Duration::ZERO).unwrap()
    }
}

impl TryFrom<ops::Range<Duration>> for TweenTimeSpan {
    type Error = NewTweenTimeSpanError;

    fn try_from(range: ops::Range<Duration>) -> Result<Self, Self::Error> {
        TweenTimeSpan::new(
            TimeBound::Inclusive(range.start),
            TimeBound::Exclusive(range.end),
        )
    }
}
impl TryFrom<ops::RangeInclusive<Duration>> for TweenTimeSpan {
    type Error = NewTweenTimeSpanError;

    fn try_from(
        range: ops::RangeInclusive<Duration>,
    ) -> Result<Self, Self::Error> {
        TweenTimeSpan::new(
            TimeBound::Inclusive(*range.start()),
            TimeBound::Inclusive(*range.end()),
        )
    }
}

impl TryFrom<ops::RangeTo<Duration>> for TweenTimeSpan {
    type Error = NewTweenTimeSpanError;

    fn try_from(range: ops::RangeTo<Duration>) -> Result<Self, Self::Error> {
        TweenTimeSpan::new(
            TimeBound::Inclusive(Duration::ZERO),
            TimeBound::Exclusive(range.end),
        )
    }
}

impl TryFrom<ops::RangeToInclusive<Duration>> for TweenTimeSpan {
    type Error = NewTweenTimeSpanError;

    fn try_from(
        range: ops::RangeToInclusive<Duration>,
    ) -> Result<Self, Self::Error> {
        TweenTimeSpan::new(
            TimeBound::Inclusive(Duration::ZERO),
            TimeBound::Inclusive(range.end),
        )
    }
}

/// Bundle for a span tweener
#[derive(Default, Bundle)]
pub struct SpanTweenerBundle {
    /// [`SpanTweener`] span tweener intestine
    pub span_tweener: SpanTweener,
    /// [`TweenTimer`] marker to declare a tweener
    pub tweener_marker: TweenerMarker,
}

impl SpanTweenerBundle {
    /// Create new [`SpanTweenerBundle`] with `duration`
    pub fn new(duration: Duration) -> Self {
        let mut t = SpanTweenerBundle::default();
        t.span_tweener.timer.set_length(duration);
        t
    }

    /// [`SpanTweenerBundle`] with the specified `paused` for the inner
    /// [`TweenTimer`]
    pub fn with_paused(mut self, paused: bool) -> Self {
        self.span_tweener.timer.set_paused(paused);
        self
    }

    /// [`SpanTweenerBundle`] with the specified `direction` for the inner
    /// [`TweenTimer`]
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.span_tweener.timer.set_direction(direction);
        self
    }

    /// [`SpanTweenerBundle`] with the specified `repeat`
    /// setting the inner [`TweenTimer`]'s repeat to Some
    pub fn with_repeat(mut self, repeat: tween_timer::Repeat) -> Self {
        self.span_tweener.timer.set_repeat(Some(repeat));
        self
    }

    /// [`SpanTweenerBundle`] with the specified `repeat_style`
    /// setting the inner [`TweenTimer`]'s repeat_style to Some
    pub fn with_repeat_style(mut self, repeat_style: RepeatStyle) -> Self {
        self.span_tweener.timer.set_repeat_style(Some(repeat_style));
        self
    }

    /// [`SpanTweenerBundle`] with without repeat,
    /// setting the inner [`TweenTimer`]'s repeat to None.
    pub fn without_repeat(mut self) -> Self {
        self.span_tweener.timer.set_repeat(None);
        self
    }

    /// [`SpanTweenerBundle`] with without repeat_style
    /// setting the inner [`TweenTimer`]'s repeat_style to None.
    pub fn without_repeat_style(mut self) -> Self {
        self.span_tweener.timer.set_repeat_style(None);
        self
    }

    /// [`SpanTweenerBundle`] with [`SpanTweenBundle`] that spans the whole
    /// length of the tweener.
    /// Quick creation of tween when you want to tween in the same entity
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_tween::prelude::*;
    /// # let world = World::default();
    /// # let mut queue = bevy::ecs::system::CommandQueue::default();
    /// # let mut commands = Commands::new(&mut queue, &world);
    /// # let start = Vec3::ZERO;
    /// # let end = Vec3::ZERO;
    /// commands.spawn((
    ///     SpriteBundle::default(),
    ///     SpanTweenerBundle::new(Duration::from_secs(5))
    ///         .with_repeat(Repeat::infinitely())
    ///         .with_repeat_style(RepeatStyle::PingPong)
    ///         .tween_here(),
    ///     EaseFunction::QuadraticInOut,
    ///     ComponentTween::tweener_entity(interpolate::Translation { start, end }),
    /// ));
    /// ```
    pub fn tween_here(self) -> SpanTweenHereBundle {
        let dur = self.span_tweener.timer.length;
        SpanTweenHereBundle {
            span_tweener_bundle: self,
            span_tween_bundle: SpanTweenBundle::new(..dur),
        }
    }
}

impl From<TweenTimer> for SpanTweenerBundle {
    fn from(value: TweenTimer) -> Self {
        SpanTweenerBundle {
            span_tweener: SpanTweener { timer: value },
            tweener_marker: TweenerMarker,
        }
    }
}

/// Returns from [`SpanTweenerBundle::tween_here`].
/// This combine [`SpanTweenerBundle`] with [`SpanTweenBundle`] that spans the
/// whole length of the tweener.
#[derive(Bundle)]
pub struct SpanTweenHereBundle {
    span_tweener_bundle: SpanTweenerBundle,
    span_tween_bundle: SpanTweenBundle,
}

/// Bundle for a span tween
#[derive(Default, Bundle)]
pub struct SpanTweenBundle {
    /// [`TweenTimeSpan`] to define the range of time this span tween will work for.
    pub span: TweenTimeSpan,
}

impl SpanTweenBundle {
    /// Create a new [`SpanTweenBundle`] from this `span`
    pub fn new<S>(span: S) -> Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
    {
        SpanTweenBundle {
            span: span.try_into().expect("valid span"),
        }
    }
}

// had to do this to silence deprecated warning
#[allow(deprecated)]
mod lol {
    use super::*;

    /// Let you quickly create a span tweener and tween in the same entity with
    /// least amount of boiler-plate possible.
    /// Returns from [`span_tween`]
    #[deprecated(
        since = "0.3.0",
        note = "Use `SpanTweener` with `SpanTweener::tween_here` instead"
    )]
    #[derive(Default, Bundle)]
    pub struct QuickSpanTweenBundle {
        pub(super) span_tweener: SpanTweenerBundle,
        pub(super) span_tween: SpanTweenBundle,
    }
}
#[allow(deprecated)]
pub use lol::QuickSpanTweenBundle;

#[allow(deprecated)]
impl QuickSpanTweenBundle {
    /// Create new [`QuickSpanTweenBundle`]
    #[deprecated(
        since = "0.3.0",
        note = "Use `SpanTweener` with `SpanTweener::tween_here` instead"
    )]
    fn new(duration: Duration) -> Self {
        let mut q = QuickSpanTweenBundle::default();
        q.span_tweener.span_tweener.timer.set_length(duration);
        q.span_tween.span = (..duration).try_into().unwrap();
        q
    }

    /// Span tweener with this repeat
    #[deprecated(
        since = "0.3.0",
        note = "Use `SpanTweener` with `SpanTweener::tween_here` instead"
    )]
    pub fn with_repeat(mut self, repeat: Repeat) -> Self {
        self.span_tweener
            .span_tweener
            .timer
            .set_repeat(Some(repeat));
        self
    }

    /// Span tweener with this repeat style
    #[deprecated(
        since = "0.3.0",
        note = "Use `SpanTweener` with `SpanTweener::tween_here` instead"
    )]
    pub fn with_repeat_style(mut self, repeat_style: RepeatStyle) -> Self {
        self.span_tweener
            .span_tweener
            .timer
            .set_repeat_style(Some(repeat_style));
        self
    }

    /// Delays the starting point of a tween for this amount of duration
    /// Note that this delay will be repeated too.
    #[deprecated(
        since = "0.3.0",
        note = "Use `SpanTweener` with `SpanTweener::tween_here` instead"
    )]
    pub fn with_delay(mut self, delay: Duration) -> Self {
        let min = self.span_tween.span.min();
        let max = self.span_tween.span.max();
        let length = max.duration() - min.duration();
        self.span_tween.span = (delay..(delay + length)).try_into().unwrap();
        self.span_tweener
            .span_tweener
            .timer
            .set_length(delay + length);
        self
    }
}

/// Convenient function to quickly create a span tweener with tween for
/// simple tweening
///
/// # Examples
///
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_tween::prelude::*;
/// # let world = World::default();
/// # let mut queue = bevy::ecs::system::CommandQueue::default();
/// # let mut commands = Commands::new(&mut queue, &world);
/// # let start = Vec3::ZERO;
/// # let end = Vec3::ZERO;
/// commands.spawn((
///     SpriteBundle::default(),
///     span_tween(Duration::from_secs(5))
///         .with_repeat(Repeat::infinitely())
///         .with_repeat_style(RepeatStyle::PingPong)
///         .with_delay(Duration::from_secs(2)),
///     EaseFunction::QuadraticInOut,
///     ComponentTween::tweener_entity(interpolate::Translation { start, end }),
/// ));
/// ```
#[deprecated(
    since = "0.3.0",
    note = "Use `SpanTweener` with `SpanTweener::tween_here` instead"
)]
#[allow(deprecated)]
pub fn span_tween(duration: Duration) -> QuickSpanTweenBundle {
    QuickSpanTweenBundle::new(duration)
}

/// Fired when a span tweener repeated or completed
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event, Reflect)]
pub struct SpanTweenerEnded {
    /// Tween timer that just ended
    pub tweener: Entity,
    /// Currently timer direction. If is [`RepeatStyle::PingPong`], the current
    /// direction will be its already changed direction.
    pub current_direction: AnimationDirection,
    /// The repeat this tweener had.
    pub with_repeat: Option<Repeat>,
}

impl SpanTweenerEnded {
    /// Returns true if the tweener's timer is completed.
    /// Completed meaning that there will be nore more ticking and all
    /// configured repeat is exhausted.
    pub fn is_completed(&self) -> bool {
        self.with_repeat
            .map(|repeat| repeat.exhausted())
            .unwrap_or(true)
    }
}

/// Tick span tweeners then send [`SpanTweenerEnded`] event if qualified for.
pub fn tick_span_tweener_system(
    time: Res<Time<Real>>,
    mut q_span_tweener: Query<(Entity, &mut SpanTweener)>,
    mut ended_writer: EventWriter<SpanTweenerEnded>,
) {
    let delta = time.delta();
    q_span_tweener.iter_mut().for_each(|(entity, mut tweener)| {
        let timer = &mut tweener.timer;
        if timer.paused {
            return;
        }

        if timer.is_completed() {
            return;
        }

        let delta = Duration::from_secs_f32(
            delta.as_secs_f32() * timer.speed_scale.as_secs_f32(),
        );

        let tick_result = timer.tick(delta, timer.direction);

        match tick_result {
            TickResult::Completed | TickResult::Repeated => {
                ended_writer.send(SpanTweenerEnded {
                    tweener: entity,
                    current_direction: timer.direction,
                    with_repeat: timer.repeat,
                });
            }
            TickResult::Continue => {}
        }
    });
}

/// System for updating any span tweens to the correct [`TweenProgressed`]
/// by its span tweener then will call `collaspe_elasped` on the timer.
pub fn span_tweener_system(
    mut commands: Commands,
    q_other_tweener: Query<(), With<TweenerMarker>>,
    mut q_span_tweener: Query<
        (Entity, &mut SpanTweener, Option<&Children>),
        Without<SkipTweener>,
    >,
    mut q_tween: Query<(Option<&mut TweenProgressed>, &TweenTimeSpan)>,
) {
    use AnimationDirection::*;
    use DurationQuotient::*;

    use crate::tween_timer::RepeatStyle::*;

    q_span_tweener.iter_mut().for_each(
        |(tweener_entity, mut tweener, children)| {
            let timer = &tweener.timer;

            if timer.is_completed() {
                return;
            }

            let children = children
                .iter()
                .flat_map(|a| a.iter())
                .filter(|c| !q_other_tweener.contains(**c));
            let tweens = [&tweener_entity].into_iter().chain(children);
            for &tween_entity in tweens {
                let Ok((tween_progressed, tween_span)) =
                    q_tween.get_mut(tween_entity)
                else {
                    continue;
                };

                let now_quotient = tween_span.quotient(timer.elasped().now);
                let previous_quotient =
                    tween_span.quotient(timer.elasped().previous);

                let tween_local_min = Duration::ZERO;
                let tween_local_max =
                    tween_span.max().duration() - tween_span.min().duration();
                let local_elasped = timer
                    .elasped()
                    .now
                    .saturating_sub(tween_span.min().duration())
                    .min(tween_local_max);
                let direction = if timer.elasped().repeat_style.is_some() {
                    match timer.elasped().previous.cmp(&timer.elasped().now) {
                        Ordering::Less => AnimationDirection::Forward,
                        Ordering::Equal => timer.direction,
                        Ordering::Greater => AnimationDirection::Backward,
                    }
                } else {
                    timer.direction
                };
                // Look at this behemoth of edge case handling.
                //
                // The edge cases are the time when the tween are really short
                // or delta is really long per frame.
                //
                // This is likely only an issue with this tweener implementation.
                //
                // This is not accounted for when the tween might repeat
                // multiple time in one frame. When that tween is this ridiculously
                // fast or the game heavily lagged, I don't think that need to
                // be accounted.
                let new_tween_elasped = match (
                    direction,
                    previous_quotient,
                    now_quotient,
                    timer.elasped().repeat_style,
                ) {
                    (_, Inside, Inside, None) => Some(local_elasped),
                    // -------------------------------------------------------
                    | (Forward, Before, Inside, None)
                    | (Forward, Inside, After, None)
                    | (Forward, Before, After, None)
                        => Some(local_elasped),

                    // -------------------------------------------------------
                    | (Backward, After, Inside, None)
                    | (Backward, Inside, Before, None)
                    | (Backward, After, Before, None)
                        => Some(local_elasped),

                    // --------------------------------------------------------
                    // don't remove these comments, may use for debugging in the future
                    | (Forward, Before, Before, Some(WrapAround)) // 1&2 max
                    | (Forward, Inside, Before, Some(WrapAround)) // 1 max
                        => Some(tween_local_max),
                    | (Forward, Before, Inside, Some(WrapAround)) // 2 now
                    | (Forward, Before, After, Some(WrapAround)) // 2 now, max
                    | (Forward, Inside, Inside, Some(WrapAround)) // 1&2 now
                    | (Forward, Inside, After, Some(WrapAround)) // 2 now, max
                    | (Forward, After, Inside, Some(WrapAround)) // 1 now 
                    | (Forward, After, After, Some(WrapAround)) // 1&2 now, max
                    // | (Forward, After, Before, Some(WrapAround)) // 1
                        => Some(local_elasped),

                    // -------------------------------------------------------
                    | (Backward, After, After, Some(WrapAround)) // 1&2 min
                    | (Backward, Inside, After, Some(WrapAround)) // 1 min
                        => Some(tween_local_min),
                    | (Backward, Before, Before, Some(WrapAround)) // 1&2 now, min
                    | (Backward, Before, Inside, Some(WrapAround)) // 1 now 
                    | (Backward, Inside, Before, Some(WrapAround)) // 2 now, min
                    | (Backward, Inside, Inside, Some(WrapAround)) // 1&2 now
                    | (Backward, After, Before, Some(WrapAround)) // 2 now, min
                    | (Backward, After, Inside, Some(WrapAround)) // 2 now
                    // | (Backward, Before, After, Some(WrapAround)) // 1
                        => Some(local_elasped),

                    // -------------------------------------------------------
                    | (Backward, Before, Before, Some(PingPong)) // 1&2 now, min
                    | (Backward, Before, Inside, Some(PingPong)) // 1 now
                    | (Backward, Before, After, Some(PingPong)) // 1 now, max
                    | (Backward, Inside, Before, Some(PingPong)) // 2 now, min
                    | (Backward, Inside, Inside, Some(PingPong)) // 1&2 now
                    | (Backward, Inside, After, Some(PingPong)) // 1 now, max
                    | (Backward, After, Before, Some(PingPong)) // 2 now, min
                    | (Backward, After, Inside, Some(PingPong)) // 2 now
                    // | (Backward, After, After, Some(PingPong)) // 1&2
                        => Some(local_elasped),

                    // -------------------------------------------------------
                    // | (Forward, Before, Before, Some(PingPong)) // 1&2
                    | (Forward, Before, Inside, Some(PingPong)) // 2 now
                    | (Forward, Before, After, Some(PingPong)) // 2 now, max
                    | (Forward, Inside, Before, Some(PingPong)) // 1 now, min
                    | (Forward, Inside, Inside, Some(PingPong)) // 1&2 now
                    | (Forward, Inside, After, Some(PingPong)) // 2 now, max
                    | (Forward, After, Before, Some(PingPong)) // 1 now, min
                    | (Forward, After, Inside, Some(PingPong)) // 1 now
                    | (Forward, After, After, Some(PingPong)) // 1&2 now, max
                        => Some(local_elasped),
                    _ => None,
                };
                match new_tween_elasped {
                    Some(elasped) => {
                        let progressed = if tween_local_max > Duration::ZERO {
                            TweenProgressed(
                                elasped.as_secs_f32()
                                    / tween_local_max.as_secs_f32(),
                            )
                        } else {
                            match timer.direction {
                                Forward => TweenProgressed(1.),
                                Backward => TweenProgressed(0.),
                            }
                        };
                        match tween_progressed {
                            Some(mut tween_progressed) => {
                                *tween_progressed = progressed;
                            }
                            None => {
                                commands
                                    .entity(tween_entity)
                                    .insert(progressed);
                            }
                        }
                    }
                    None => {
                        if tween_progressed.is_some() {
                            commands
                                .entity(tween_entity)
                                .remove::<TweenProgressed>();
                        }
                    }
                }
            }
            tweener.timer.collaspe_elasped();
        },
    );
}

// #[doc(hidden)]
// pub trait EntitySpawner: sealed::EntitySpawnerSealed {
//     type CommandOutput<'c>
//     where
//         Self: 'c;
//     fn spawn(&mut self, bundle: impl Bundle) -> Self::CommandOutput<'_>;
// }

// impl<'a> EntitySpawner for ChildBuilder<'a> {
//     type CommandOutput<'c> = bevy::ecs::system::EntityCommands<'c>
//     where Self: 'c;

//     fn spawn(&mut self, bundle: impl Bundle) -> Self::CommandOutput<'_> {
//         self.spawn(bundle)
//     }
// }

// impl<'a> EntitySpawner for WorldChildBuilder<'a> {
//     type CommandOutput<'c> = EntityWorldMut<'c>
//     where Self: 'c;

//     fn spawn(&mut self, bundle: impl Bundle) -> Self::CommandOutput<'_> {
//         self.spawn(bundle)
//     }
// }

// mod sealed {
//     use bevy::prelude::*;

//     pub trait EntitySpawnerSealed {}

//     impl<'a> EntitySpawnerSealed for ChildBuilder<'a> {}
//     impl<'a> EntitySpawnerSealed for WorldChildBuilder<'a> {}
// }

/// Convenient builder for building multiple children tweens
pub struct ChildSpanTweenBuilder<'r, 'b> {
    child_builder: &'r mut ChildBuilder<'b>,
}

impl<'r, 'b> ChildSpanTweenBuilder<'r, 'b> {
    /// Create a new span tween.
    pub fn tween<S, I, T>(
        &mut self,
        span: S,
        interpolation: I,
        tween: T,
    ) -> &mut Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
        I: Component + Interpolation,
        T: Bundle,
    {
        self.tween_and(span, interpolation, tween, |_| {})
    }

    /// Create a new span tween then call a closure with the tween's
    /// [`EntityCommands`].
    pub fn tween_and<S, I, T, F>(
        &mut self,
        span: S,
        interpolation: I,
        bundle: T,
        f: F,
    ) -> &mut Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
        I: Component + Interpolation,
        T: Bundle,
        F: FnOnce(EntityCommands<'_>),
    {
        let commands = self.child_builder.spawn((
            SpanTweenBundle::new(span),
            interpolation,
            bundle,
        ));
        f(commands);
        self
    }

    /// Create a new span tween that's 0 seconds in duration which basically
    /// not tween anything but change the value instantly at some input time
    /// then call a closure with the tween's [`EntityCommands`].
    pub fn jump_and<T, F>(&mut self, at: Duration, bundle: T, f: F) -> &mut Self
    where
        T: Bundle,
        F: FnOnce(EntityCommands<'_>),
    {
        self.tween_and(at..=at, EaseFunction::Linear, bundle, f)
    }

    /// Create a new span tween that's 0 seconds in duration which basically
    /// not tween anything but change the value instantly at some input time.
    pub fn jump<T>(&mut self, at: Duration, bundle: T) -> &mut Self
    where
        T: Bundle,
    {
        self.tween_and(at..=at, EaseFunction::Linear, bundle, |_| {})
    }
}

/// Helper trait
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
pub trait ChildSpanTweenBuilderExt<'b>: sealed::Sealed {
    /// Create the builder
    #[deprecated(
        since = "0.3.0",
        note = "Renamed to `span_tweens` to reduce ambiguity"
    )]
    fn child_tweens<'r>(&'r mut self) -> ChildSpanTweenBuilder<'r, 'b> {
        self.span_tweens()
    }
    /// Create a child builder for span tween.
    fn span_tweens<'r>(&'r mut self) -> ChildSpanTweenBuilder<'r, 'b>;
}

impl<'b> ChildSpanTweenBuilderExt<'b> for ChildBuilder<'b> {
    fn span_tweens<'r>(&'r mut self) -> ChildSpanTweenBuilder<'r, 'b> {
        ChildSpanTweenBuilder {
            child_builder: self,
        }
    }
}

/// Convenient builder for building multiple children tweens
pub struct WorldChildSpanTweenBuilder<'r, 'b> {
    world_child_builder: &'r mut WorldChildBuilder<'b>,
}

impl<'r, 'b> WorldChildSpanTweenBuilder<'r, 'b> {
    /// Create a new span tween.
    pub fn tween<S, I, T>(
        &mut self,
        span: S,
        interpolation: I,
        tween: T,
    ) -> &mut Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
        I: Component + Interpolation,
        T: Bundle,
    {
        self.tween_and(span, interpolation, tween, |_| {})
    }

    /// Create a new span tween then call a closure with the tween's
    /// [`EntityWorldMut`].
    pub fn tween_and<S, I, T, F>(
        &mut self,
        span: S,
        interpolation: I,
        bundle: T,
        f: F,
    ) -> &mut Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
        I: Component + Interpolation,
        T: Bundle,
        F: FnOnce(EntityWorldMut<'_>),
    {
        let commands = self.world_child_builder.spawn((
            SpanTweenBundle::new(span),
            interpolation,
            bundle,
        ));
        f(commands);
        self
    }

    /// Create a new span tween that's 0 seconds in duration which basically
    /// not tween anything but change the value instantly at some input time
    /// then call a closure with the tween's [`EntityWorldMut`].
    pub fn jump_and<T, F>(&mut self, at: Duration, bundle: T, f: F) -> &mut Self
    where
        T: Bundle,
        F: FnOnce(EntityWorldMut<'_>),
    {
        self.tween_and(at..=at, EaseFunction::Linear, bundle, f)
    }

    /// Create a new span tween that's 0 seconds in duration which basically
    /// not tween anything but change the value instantly at some input time.
    pub fn jump<T>(&mut self, at: Duration, bundle: T) -> &mut Self
    where
        T: Bundle,
    {
        self.tween_and(at..=at, EaseFunction::Linear, bundle, |_| {})
    }
}

/// Helper trait
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
pub trait WorldChildSpanTweenBuilderExt<'b>: sealed::Sealed {
    /// Create the builder
    #[deprecated(
        since = "0.3.0",
        note = "Renamed to `span_tweens` to reduce ambiguity"
    )]
    fn child_tweens<'r>(&'r mut self) -> WorldChildSpanTweenBuilder<'r, 'b> {
        self.span_tweens()
    }
    /// Create a world child builder for span tween.
    fn span_tweens<'r>(&'r mut self) -> WorldChildSpanTweenBuilder<'r, 'b>;
}

impl<'b> WorldChildSpanTweenBuilderExt<'b> for WorldChildBuilder<'b> {
    fn span_tweens<'r>(&'r mut self) -> WorldChildSpanTweenBuilder<'r, 'b> {
        WorldChildSpanTweenBuilder {
            world_child_builder: self,
        }
    }
}

mod sealed {
    use super::*;

    pub trait Sealed {}

    impl<'a> Sealed for WorldChildBuilder<'a> {}
    impl<'a> Sealed for ChildBuilder<'a> {}
}
