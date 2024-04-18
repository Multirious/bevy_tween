//! Module containing this crate's default tweener implementation
//!
//! # Span tween
//!
//! **Plugins**:
//! - [`TweenerPlugin`]
//!
//! **Components**:
//! - [`Tweener`]
//! - [`TimeSpan`]
//!
//! **Bundles**:
//! - [`TweenerBundle`]
//!
//! **Events**:
//! - [`TweenerEnded`]
//!
//! **Systems**:
//! - [`tweener_system`]
//! - [`tick_tweener_system`]
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
//! We can create a tweener by:
//! - Tween in the same entity as a tweener.<br/>
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
//!   // Spawning some tweener
//!   commands.spawn((
//!       // The tweener:
//!       TweenerBundle::new(Duration::from_secs(1)),
//!       // The tween:
//!       // Tween this from the start to the second 1.
//!       TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
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
//! - Tween(s) as a child of a tweener.<br/>
//!   This is the case where you want to make a more complex animation. By having
//!   tweens as tweener's children, you can have any number of
//!   tween types you wanted .
//!   ```no_run
//!   # use bevy::prelude::*;
//!   # use bevy_tween::prelude::*;
//!   # let world = World::new();
//!   # let mut commands_queue = bevy::ecs::system::CommandQueue::default();
//!   # let mut commands = Commands::new(&mut commands_queue, &world);
//!   # let my_entity = commands.spawn(SpriteBundle::default()).id();
//!   // Spawning some tweener
//!   commands.spawn(
//!       // The tweener:
//!       TweenerBundle::new(Duration::from_secs(1)),
//!   ).with_children(|c| {
//!       // The tween:
//!       c.spawn((
//!           TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
//!           EaseFunction::QuadraticOut,
//!           ComponentTween::new_target(
//!               my_entity,
//!               interpolate::Translation {
//!                   start: Vec3::new(0., 0., 0.),
//!                   end: Vec3::new(0., 100., 0.),
//!               }
//!           )
//!       ));
//!      // spawn some more tween if needed.
//!      // c.spawn( ... );
//!
//!      // we can also uses the builder
//!      c.tweens().tween(
//!          Duration::from_secs(1),
//!          EaseFunction::QuadraticOut,
//!          ComponentTween::new_target(
//!              my_entity,
//!              interpolate::Translation {
//!                  start: Vec3::new(0., 0., 0.),
//!                  end: Vec3::new(0., 100., 0.),
//!              }
//!          )
//!      );
//!   });
//!   ```
//! - Also the above 2 combined will works just fine btw.

use std::{cmp::Ordering, ops, time::Duration};

use crate::utils;
use bevy::prelude::*;
#[cfg(feature = "bevy_eventlistener")]
use bevy_eventlistener::prelude::*;
use tween_timer::{Repeat, RepeatStyle};

use crate::{
    prelude::TweenEventData,
    tween::{SkipTweener, TweenProgress, TweenerMarker},
    tween_timer::{self, AnimationDirection, TweenTimer},
};

/// Plugin for using span tween
#[derive(Debug)]
pub struct TweenerPlugin;

impl Plugin for TweenerPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be is inserted to world");
        app.add_systems(
            app_resource.schedule,
            (
                tick_tweener_system.in_set(crate::TweenSystemSet::TickTweener),
                tweener_system.in_set(crate::TweenSystemSet::Tweener),
            ),
        )
        .register_type::<Tweener>()
        .register_type::<TimeBound>()
        .register_type::<TimeSpan>()
        .add_event::<TweenerEnded>();

        #[cfg(feature = "bevy_eventlistener")]
        app.add_plugins(EventListenerPlugin::<TweenerEnded>::default());
    }
}

#[deprecated(since = "0.5.0", note = "`SpanTweener` is renamed to `Tweener`")]
#[allow(missing_docs)]
pub type SpanTweener = Tweener;

/// Span tweener
#[derive(Debug, Default, Component, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Tweener {
    /// The inner timer
    pub timer: TweenTimer,
}

impl From<TweenTimer> for Tweener {
    fn from(value: TweenTimer) -> Self {
        Tweener { timer: value }
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

/// Error type for when creating a new [`TimeSpan`].
#[derive(Debug)]
pub enum NewTimeSpanError {
    /// The provided min, max will result in a [`TimeSpan`] that does not
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

impl std::error::Error for NewTimeSpanError {}
impl std::fmt::Display for NewTimeSpanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewTimeSpanError::NotTime { min, max } => {
                write!(f, "This span does not contain any time: min {min:?} max {max:?}")
            }
            NewTimeSpanError::MinGreaterThanMax { min, max } => {
                write!(f, "This span has min greater than max: min {min:?} max {max:?}")
            }
        }
    }
}

#[deprecated(
    since = "0.5.0",
    note = "`TweenTimeSpan ` is renamed to `TimeSpan`"
)]
#[allow(missing_docs)]
pub type TweenTimeSpan = TimeSpan;

/// Define the range of time for a span tween that will be interpolating for.
#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TimeSpan {
    /// Minimum time for the tween.
    min: TimeBound,
    /// Maximum time for the tween.
    max: TimeBound,
}
impl TimeSpan {
    /// Create a new [`TimeSpan`] unchecked for invalid min, max.
    pub(crate) fn new_unchecked(min: TimeBound, max: TimeBound) -> TimeSpan {
        TimeSpan { min, max }
    }

    /// Create a new [`TimeSpan`]
    pub fn new(
        min: TimeBound,
        max: TimeBound,
    ) -> Result<TimeSpan, NewTimeSpanError> {
        if matches!(
            (min, max),
            (TimeBound::Exclusive(_), TimeBound::Exclusive(_))
        ) && min.duration() == max.duration()
        {
            return Err(NewTimeSpanError::NotTime { min, max });
        } else if min.duration() > max.duration() {
            return Err(NewTimeSpanError::MinGreaterThanMax { min, max });
        }
        Ok(Self::new_unchecked(min, max))
    }

    fn quotient(&self, secs: f32) -> DurationQuotient {
        let after_min = match self.min {
            TimeBound::Inclusive(min) => secs >= min.as_secs_f32(),
            TimeBound::Exclusive(min) => secs > min.as_secs_f32(),
        };
        let before_max = match self.max {
            TimeBound::Inclusive(max) => secs <= max.as_secs_f32(),
            TimeBound::Exclusive(max) => secs < max.as_secs_f32(),
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

impl Default for TimeSpan {
    fn default() -> Self {
        TimeSpan::try_from(Duration::ZERO..Duration::ZERO).unwrap()
    }
}

impl TryFrom<ops::Range<Duration>> for TimeSpan {
    type Error = NewTimeSpanError;

    fn try_from(range: ops::Range<Duration>) -> Result<Self, Self::Error> {
        TimeSpan::new(
            TimeBound::Inclusive(range.start),
            TimeBound::Exclusive(range.end),
        )
    }
}
impl TryFrom<ops::RangeInclusive<Duration>> for TimeSpan {
    type Error = NewTimeSpanError;

    fn try_from(
        range: ops::RangeInclusive<Duration>,
    ) -> Result<Self, Self::Error> {
        TimeSpan::new(
            TimeBound::Inclusive(*range.start()),
            TimeBound::Inclusive(*range.end()),
        )
    }
}

impl TryFrom<ops::RangeTo<Duration>> for TimeSpan {
    type Error = NewTimeSpanError;

    fn try_from(range: ops::RangeTo<Duration>) -> Result<Self, Self::Error> {
        TimeSpan::new(
            TimeBound::Inclusive(Duration::ZERO),
            TimeBound::Exclusive(range.end),
        )
    }
}

impl TryFrom<ops::RangeToInclusive<Duration>> for TimeSpan {
    type Error = NewTimeSpanError;

    fn try_from(
        range: ops::RangeToInclusive<Duration>,
    ) -> Result<Self, Self::Error> {
        TimeSpan::new(
            TimeBound::Inclusive(Duration::ZERO),
            TimeBound::Inclusive(range.end),
        )
    }
}

#[deprecated(
    since = "0.5.0",
    note = "`SpanTweenerBundle` is renamed to `TweenerBundle`"
)]
#[allow(missing_docs)]
pub type SpanTweenerBundle = TweenerBundle;

/// Bundle for a span tweener
#[derive(Default, Bundle)]
pub struct TweenerBundle {
    /// [`Tweener`] span tweener intestine
    pub tweener: Tweener,
    /// [`TweenTimer`] marker to declare a tweener
    pub tweener_marker: TweenerMarker,
}

impl TweenerBundle {
    /// Create new [`TweenerBundle`] with `duration`
    pub fn new(duration: Duration) -> Self {
        let mut t = TweenerBundle::default();
        t.tweener.timer.set_length(duration);
        t
    }

    /// [`TweenerBundle`] with the specified `paused` for the inner
    /// [`TweenTimer`]
    pub fn with_paused(mut self, paused: bool) -> Self {
        self.tweener.timer.set_paused(paused);
        self
    }

    /// [`TweenerBundle`] with the specified `direction` for the inner
    /// [`TweenTimer`]
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.tweener.timer.set_direction(direction);
        self
    }

    /// [`TweenerBundle`] with the specified `repeat`
    /// setting the inner [`TweenTimer`]'s repeat to Some
    pub fn with_repeat(mut self, repeat: tween_timer::Repeat) -> Self {
        let timer = &mut self.tweener.timer;
        match timer.repeat {
            Some((_, repeat_style)) => {
                timer.set_repeat(Some((repeat, repeat_style)));
            }
            None => {
                timer.set_repeat(Some((repeat, RepeatStyle::default())));
            }
        }
        self
    }

    /// [`TweenerBundle`] with the specified `repeat_style`
    /// setting the inner [`TweenTimer`]'s repeat_style to Some
    pub fn with_repeat_style(mut self, repeat_style: RepeatStyle) -> Self {
        let timer = &mut self.tweener.timer;
        match timer.repeat {
            Some((repeat, _)) => {
                timer.set_repeat(Some((repeat, repeat_style)));
            }
            None => {
                timer.set_repeat(Some((Repeat::infinitely(), repeat_style)));
            }
        }
        self
    }

    /// [`TweenerBundle`] with without repeat,
    /// setting the inner [`TweenTimer`]'s repeat to None.
    pub fn without_repeat(mut self) -> Self {
        self.tweener.timer.set_repeat(None);
        self
    }

    /// [`TweenerBundle`] with without repeat_style
    /// setting the inner [`TweenTimer`]'s repeat_style to None.
    #[deprecated(since = "0.3.0")]
    pub fn without_repeat_style(mut self) -> Self {
        match &mut self.tweener.timer.repeat {
            Some((_, repeat_style)) => *repeat_style = RepeatStyle::WrapAround,
            None => {}
        }
        self
    }

    /// [`TweenerBundle`] with [`SpanTweenBundle`] that spans the whole
    /// length of the tweener.
    /// A convenient shortcut to simply include [`SpanTweenBundle`] and [`TweenerBundle`]
    /// together to quickly create a tween in-place. Can be used to create a very
    /// simple tween entity that doesn't need to use multiple entities.
    ///
    /// # Examples
    ///
    /// ```
    #[doc = utils::doc_entity_eq_fn!()]
    #[doc = utils::doc_app_test_boilerplate!()]
    /// # const interpolation: interpolate::Translation = interpolate::Translation { start: Vec3::ZERO, end: Vec3::ZERO };
    /// # let shortcut =
    /// commands.spawn((
    ///     SpriteBundle::default(),
    ///     TweenerBundle::new(Duration::from_secs(5))
    ///         .with_repeat(Repeat::infinitely())
    ///         .with_repeat_style(RepeatStyle::PingPong)
    ///         .tween_here(),
    ///     EaseFunction::QuadraticInOut,
    ///     ComponentTween::tweener_entity(interpolation),
    /// ))
    /// # .id();
    ///
    /// // is exactly the same as
    ///
    /// # let manual =
    /// commands.spawn((
    ///     SpriteBundle::default(),
    ///     TweenerBundle::new(Duration::from_secs(5))
    ///         .with_repeat(Repeat::infinitely())
    ///         .with_repeat_style(RepeatStyle::PingPong),
    ///     TimeSpan::try_from(..Duration::from_secs(5)).unwrap(),
    ///     EaseFunction::QuadraticInOut,
    ///     ComponentTween::tweener_entity(interpolation),
    /// ))
    /// # .id();
    ///
    /// # queue.apply(&mut app.world);
    /// #
    /// # assert!(entity_eq(&app.world, shortcut, manual));
    /// ```
    pub fn tween_here(self) -> TweenHereBundle {
        let dur = self.tweener.timer.length;
        TweenHereBundle {
            tweener_bundle: self,
            time_span: TimeSpan::try_from(..dur).unwrap(),
        }
    }
}

impl From<TweenTimer> for TweenerBundle {
    fn from(value: TweenTimer) -> Self {
        TweenerBundle {
            tweener: Tweener { timer: value },
            tweener_marker: TweenerMarker,
        }
    }
}

/// Returns from [`TweenerBundle::tween_here`].
/// This combine [`TweenerBundle`] with [`SpanTweenBundle`] that spans the
/// whole length of the tweener.
#[derive(Bundle)]
pub struct TweenHereBundle {
    tweener_bundle: TweenerBundle,
    time_span: TimeSpan,
}

#[allow(deprecated)]
mod another_lol {
    use super::*;

    /// Bundle for a span tween
    #[deprecated(
        since = "0.5.0",
        note = "This bundle is unnecessary. Just use `TimeSpan` directly"
    )]
    #[derive(Default, Bundle)]
    pub struct SpanTweenBundle {
        /// [`TimeSpan`] to define the range of time this span tween will work for.
        pub span: TimeSpan,
    }
}
#[allow(deprecated)]
pub use another_lol::SpanTweenBundle;

#[allow(deprecated)]
impl SpanTweenBundle {
    /// Create a new [`SpanTweenBundle`] from this `span`
    pub fn new<S>(span: S) -> Self
    where
        S: TryInto<TimeSpan>,
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
        note = "Use `Tweener` with `Tweener::tween_here` instead"
    )]
    #[derive(Default, Bundle)]
    pub struct QuickSpanTweenBundle {
        pub(super) span_tweener: TweenerBundle,
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
        note = "Use `Tweener` with `Tweener::tween_here` instead"
    )]
    fn new(duration: Duration) -> Self {
        let mut q = QuickSpanTweenBundle::default();
        q.span_tweener.tweener.timer.set_length(duration);
        q.span_tween.span = (..duration).try_into().unwrap();
        q
    }

    /// Span tweener with this repeat
    #[deprecated(
        since = "0.3.0",
        note = "Use `Tweener` with `Tweener::tween_here` instead"
    )]
    pub fn with_repeat(mut self, repeat: Repeat) -> Self {
        let timer = &mut self.span_tweener.tweener.timer;
        match timer.repeat {
            Some((_, repeat_style)) => {
                timer.set_repeat(Some((repeat, repeat_style)));
            }
            None => {
                timer.set_repeat(Some((repeat, RepeatStyle::default())));
            }
        }
        self
    }

    /// Span tweener with this repeat style
    #[deprecated(
        since = "0.3.0",
        note = "Use `Tweener` with `Tweener::tween_here` instead"
    )]
    pub fn with_repeat_style(mut self, repeat_style: RepeatStyle) -> Self {
        let timer = &mut self.span_tweener.tweener.timer;
        match timer.repeat {
            Some((repeat, _)) => {
                timer.set_repeat(Some((repeat, repeat_style)));
            }
            None => {
                timer.set_repeat(Some((Repeat::infinitely(), repeat_style)));
            }
        }
        self
    }

    /// Delays the starting point of a tween for this amount of duration
    /// Note that this delay will be repeated too.
    #[deprecated(
        since = "0.3.0",
        note = "Use `Tweener` with `Tweener::tween_here` instead"
    )]
    pub fn with_delay(mut self, delay: Duration) -> Self {
        let min = self.span_tween.span.min();
        let max = self.span_tween.span.max();
        let length = max.duration() - min.duration();
        self.span_tween.span = (delay..(delay + length)).try_into().unwrap();
        self.span_tweener.tweener.timer.set_length(delay + length);
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
    note = "Use `Tweener` with `Tweener::tween_here` instead"
)]
#[allow(deprecated)]
pub fn span_tween(duration: Duration) -> QuickSpanTweenBundle {
    QuickSpanTweenBundle::new(duration)
}

#[deprecated(since = "0.5.0", note = "`SpanTweener` is renamed to `Tweener`")]
#[allow(missing_docs)]
pub type SpanTweenerEnded = TweenerEnded;

/// Fired when a span tweener repeated or completed
#[cfg_attr(feature = "bevy_eventlistener", derive(EntityEvent))]
#[cfg_attr(feature = "bevy_eventlistener", can_bubble)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event, Reflect)]
pub struct TweenerEnded {
    /// Tween timer that just ended
    #[cfg_attr(feature = "bevy_eventlistener", target)]
    pub tweener: Entity,
    /// Currently timer direction. If is [`RepeatStyle::PingPong`], the current
    /// direction will be its already changed direction.
    pub current_direction: AnimationDirection,
    /// The repeat this tweener had.
    pub with_repeat: Option<Repeat>,
}

impl TweenerEnded {
    /// Returns true if the tweener's timer is completed.
    /// Completed meaning that there will be nore more ticking and all
    /// configured repeat is exhausted.
    pub fn is_completed(&self) -> bool {
        self.with_repeat
            .map(|repeat| repeat.exhausted())
            .unwrap_or(true)
    }
}

/// Tick span tweeners then send [`TweenerEnded`] event if qualified for.
pub fn tick_tweener_system(
    time: Res<Time<Real>>,
    mut q_tweener: Query<(Entity, &mut Tweener)>,
    mut ended_writer: EventWriter<TweenerEnded>,
) {
    let delta = time.delta_seconds();
    q_tweener.iter_mut().for_each(|(entity, mut tweener)| {
        let timer = &mut tweener.timer;
        if timer.paused || timer.is_completed() {
            return;
        }
        timer.tick(delta * timer.speed_scale.as_secs_f32());
        // println!(
        //     "Ticked: {:.2}, {:.2}",
        //     timer.elasped().now,
        //     timer.elasped().now_percentage
        // );

        let n = timer.elasped().now_period;
        let send_event = match timer.repeat {
            Some((_, RepeatStyle::PingPong)) => {
                (timer.direction == AnimationDirection::Forward && n < 0.)
                    || (timer.direction == AnimationDirection::Backward
                        && n >= 1.)
            }
            _ => {
                (timer.direction == AnimationDirection::Backward && n < 0.)
                    || (timer.direction == AnimationDirection::Forward
                        && n >= 1.)
            }
        };
        if send_event {
            ended_writer.send(TweenerEnded {
                tweener: entity,
                current_direction: timer.direction,
                with_repeat: timer.repeat.map(|r| r.0),
            });
        }
    });
}

/// System for updating any span tweens to the correct [`TweenProgress`]
/// by its span tweener then will call `collaspe_elasped` on the timer.
pub fn tweener_system(
    mut commands: Commands,
    q_other_tweener: Query<(), With<TweenerMarker>>,
    mut q_tweener: Query<
        (Entity, &mut Tweener, Option<&Children>),
        Without<SkipTweener>,
    >,
    mut q_tween: Query<(Entity, Option<&mut TweenProgress>, &TimeSpan)>,
    q_added_skip: Query<
        (Entity, &Tweener, Option<&Children>),
        Added<SkipTweener>,
    >,
    mut tweener_just_completed: Local<Vec<Entity>>,
) {
    use AnimationDirection::*;
    use DurationQuotient::*;

    use crate::tween_timer::RepeatStyle::*;

    let mut just_completed_tweeners =
        q_tweener.iter_many(&tweener_just_completed);
    while let Some((tweener_entity, tweener, children)) =
        just_completed_tweeners.fetch_next()
    {
        let timer = &tweener.timer;

        if !timer.is_completed() {
            return;
        }

        let children = children
            .iter()
            .flat_map(|a| a.iter())
            .filter(|c| !q_other_tweener.contains(**c));
        let mut tweens = q_tween
            .iter_many_mut([&tweener_entity].into_iter().chain(children));
        while let Some((tween_entity, _, _)) = tweens.fetch_next() {
            let Some(mut entity) = commands.get_entity(tween_entity) else {
                continue;
            };
            entity.remove::<TweenProgress>();
        }
    }
    tweener_just_completed.clear();

    q_added_skip
        .iter()
        .for_each(|(tweener_entity, _, children)| {
            let children = children
                .iter()
                .flat_map(|a| a.iter())
                .filter(|c| !q_other_tweener.contains(**c));
            let mut tweens = q_tween
                .iter_many_mut([&tweener_entity].into_iter().chain(children));
            while let Some((tween_entity, _, _)) = tweens.fetch_next() {
                let Some(mut entity) = commands.get_entity(tween_entity) else {
                    continue;
                };
                entity.remove::<TweenProgress>();
            }
        });

    q_tweener
        .iter_mut()
        .for_each(|(tweener_entity, mut tweener, children)| {
            let timer = &tweener.timer;

            if timer.is_completed() {
                return;
            }

            let repeated = if timer.elasped().now_period.floor() as i32 != 0
                && !timer.is_completed()
            {
                timer.repeat.map(|r| r.1)
            } else {
                None
            };

            let timer_elasped_now = timer.elasped().now;
            let timer_elasped_previous = timer.elasped().previous;
            let timer_direction = timer.direction;

            let children = children
                .iter()
                .flat_map(|a| a.iter())
                .filter(|c| !q_other_tweener.contains(**c));
            let mut tweens = q_tween
                .iter_many_mut([&tweener_entity].into_iter().chain(children));
            while let Some((tween_entity, tween_progress, tween_span)) =
                tweens.fetch_next()
            {
                let now_quotient = tween_span.quotient(timer_elasped_now);
                let previous_quotient =
                    tween_span.quotient(timer_elasped_previous);

                let direction = if repeated.is_none() {
                    match timer_elasped_previous.total_cmp(&timer_elasped_now) {
                        Ordering::Less => AnimationDirection::Forward,
                        Ordering::Equal => timer_direction,
                        Ordering::Greater => AnimationDirection::Backward,
                    }
                } else {
                    timer_direction
                };

                let tween_visible = tween_visible(
                    direction,
                    previous_quotient,
                    now_quotient,
                    repeated,
                );

                if let Some(use_time) = tween_visible {
                    let tween_span_max =
                        tween_span.max().duration().as_secs_f32();
                    let tween_span_min =
                        tween_span.min().duration().as_secs_f32();

                    let tween_length = tween_span_max - tween_span_min;

                    let new_now = match use_time {
                        UseTime::Current => timer_elasped_now - tween_span_min,
                        UseTime::Min => 0.,
                        UseTime::Max => tween_length,
                    };
                    let new_previous = timer_elasped_previous - tween_span_min;

                    let tween_pos = tween_span_min;

                    let new_now_percentage = if tween_length > 0. {
                        new_now / tween_length
                    } else {
                        match new_now.total_cmp(&tween_pos) {
                            Ordering::Greater => f32::INFINITY,
                            Ordering::Equal => match timer_direction {
                                Forward => f32::INFINITY,
                                Backward => f32::NEG_INFINITY,
                            },
                            Ordering::Less => f32::NEG_INFINITY,
                        }
                    };
                    let new_previous_percentage = if tween_length > 0. {
                        new_previous / tween_length
                    } else {
                        match new_previous.total_cmp(&tween_pos) {
                            Ordering::Greater => f32::INFINITY,
                            Ordering::Equal => match timer_direction {
                                Forward => f32::INFINITY,
                                Backward => f32::NEG_INFINITY,
                            },
                            Ordering::Less => f32::NEG_INFINITY,
                        }
                    };

                    // match name {
                    //     Some(name) => {
                    //         println!(
                    //             "{}: {:.2}, {:.2}",
                    //             name, new_now, new_now_percentage
                    //         );
                    //     }
                    //     None => {
                    //         println!(
                    //             "-: {:.2}, {:.2}",
                    //             new_now, new_now_percentage
                    //         );
                    //     }
                    // }
                    match tween_progress {
                        Some(mut tween_progress) => {
                            tween_progress.update(new_now, new_now_percentage);
                        }
                        None => {
                            commands.entity(tween_entity).insert(
                                TweenProgress {
                                    now_percentage: new_now_percentage,
                                    now: new_now,
                                    previous_percentage:
                                        new_previous_percentage,
                                    previous: new_previous,
                                },
                            );
                        }
                    }
                } else {
                    commands.entity(tween_entity).remove::<TweenProgress>();
                }
            }
            tweener.timer.collaspe_elasped();
            if tweener.timer.is_completed() {
                tweener_just_completed.push(tweener_entity);
            }
        });

    enum UseTime {
        Current,
        Min,
        Max,
    }

    fn tween_visible(
        direction: AnimationDirection,
        previous_quotient: DurationQuotient,
        now_quotient: DurationQuotient,
        repeated: Option<RepeatStyle>,
    ) -> Option<UseTime> {
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

        match (
                    direction,
                    previous_quotient,
                    now_quotient,
                    repeated,
                ) {
                    (_, Inside, Inside, None) => {
                        // match f {
                        //     Forward => println!("forward"),
                        //     Backward => println!("backward"),
                        // }
                        Some(UseTime::Current)
                    },
                    // -------------------------------------------------------
                    | (Forward, Before, Inside, None)
                    | (Forward, Inside, After, None)
                    | (Forward, Before, After, None)
                        => {
                            // println!("inter forward");
                            Some(UseTime::Current)
                        },

                    // -------------------------------------------------------
                    | (Backward, After, Inside, None)
                    | (Backward, Inside, Before, None)
                    | (Backward, After, Before, None)
                        => {
                            // println!("inter backward");
                            Some(UseTime::Current)
                        },

                    // --------------------------------------------------------
                    // don't remove these comments, may use for debugging in the future
                    | (Forward, Before, Before, Some(WrapAround)) // 1&2 max
                    | (Forward, Inside, Before, Some(WrapAround)) // 1 max
                        => {
                            // println!("forward wrap use max");
                            Some(UseTime::Max)
                        },
                    | (Forward, Before, Inside, Some(WrapAround)) // 2 now
                    | (Forward, Before, After, Some(WrapAround)) // 2 now, max
                    | (Forward, Inside, Inside, Some(WrapAround)) // 1&2 now
                    | (Forward, Inside, After, Some(WrapAround)) // 2 now, max
                    | (Forward, After, Inside, Some(WrapAround)) // 1 now 
                    | (Forward, After, After, Some(WrapAround)) // 1&2 now, max
                    // | (Forward, After, Before, Some(WrapAround)) // 1
                        => {
                            // println!("forward wrap use current");
                            Some(UseTime::Current)
                        },

                    // -------------------------------------------------------
                    | (Backward, After, After, Some(WrapAround)) // 1&2 min
                    | (Backward, Inside, After, Some(WrapAround)) // 1 min
                        => {
                            // println!("backward wrap use min");
                            Some(UseTime::Min)
                        },
                    | (Backward, Before, Before, Some(WrapAround)) // 1&2 now, min
                    | (Backward, Before, Inside, Some(WrapAround)) // 1 now 
                    | (Backward, Inside, Before, Some(WrapAround)) // 2 now, min
                    | (Backward, Inside, Inside, Some(WrapAround)) // 1&2 now
                    | (Backward, After, Before, Some(WrapAround)) // 2 now, min
                    | (Backward, After, Inside, Some(WrapAround)) // 2 now
                    // | (Backward, Before, After, Some(WrapAround)) // 1
                        => {
                            // println!("backward wrap use current");
                            Some(UseTime::Current)
                        },

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
                        => Some(UseTime::Current),

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
                        => Some(UseTime::Current),
                    _ => None,
                }
    }
}

mod sealed {
    use super::*;

    pub trait Sealed {}

    impl<'a> Sealed for WorldChildBuilder<'a> {}
    impl<'a> Sealed for ChildBuilder<'a> {}

    /// Type that can spawn an entity from a bundle
    ///
    /// This trait is sealed and not meant to be implemented outside of the current crate.
    pub trait TweenSpanwerSealed: sealed::Sealed {
        type CommandOutput<'c>
        where
            Self: 'c;
        fn spawn(&mut self, bundle: impl Bundle) -> Self::CommandOutput<'_>;
    }

    impl<'a> TweenSpanwerSealed for ChildBuilder<'a> {
        type CommandOutput<'c> = bevy::ecs::system::EntityCommands<'c>
        where Self: 'c;

        fn spawn(&mut self, bundle: impl Bundle) -> Self::CommandOutput<'_> {
            self.spawn(bundle)
        }
    }

    impl<'a> TweenSpanwerSealed for WorldChildBuilder<'a> {
        type CommandOutput<'c> = EntityWorldMut<'c>
        where Self: 'c;

        fn spawn(&mut self, bundle: impl Bundle) -> Self::CommandOutput<'_> {
            self.spawn(bundle)
        }
    }
}

/// Type that can spawn an entity from a bundle
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
pub trait EntitySpawner: sealed::EntitySpanwerSealed {}
impl<T> EntitySpawner for T where T: sealed::EntitySpanwerSealed {}

#[deprecated(since = "0.5.0", note = "`SpanTweener` is renamed to `Tweener`")]
#[allow(missing_docs)]
pub type SpanTweensBuilder<'r, E> = TweensBuilder<'r, E>;

/// Convenient builder for building multiple children span tweens. This is return
/// from [`TweensBuilderExt::tweens`]
pub struct TweensBuilder<'r, E>
where
    E: EntitySpawner,
{
    entity_spawner: &'r mut E,
    offset: Duration,
}

impl<'r, E> TweensBuilder<'r, E>
where
    E: EntitySpawner,
{
    fn new(entity_spawner: &'r mut E) -> Self {
        TweensBuilder {
            entity_spawner,
            offset: Duration::ZERO,
        }
    }
}

impl<'r, E> TweensBuilder<'r, E>
where
    E: EntitySpawner,
{
    /// Create a new span tween with the supplied span.
    ///
    /// <div class="warning">
    ///
    /// The internal offset do not change after this call!
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = utils::doc_entity_eq_fn!()]
    #[doc = utils::doc_app_test_boilerplate!()]
    /// # let sprite =
    /// commands
    ///     .spawn((
    ///         SpriteBundle::default(),
    ///         TweenerBundle::new(Duration::from_secs(1)),
    ///     ))
    ///     .with_children(|c| {
    ///         c.tweens().tween_exact(
    ///             ..Duration::from_secs(1),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         );
    ///
    ///         // is exactly the same as
    ///
    ///         c.spawn((
    ///             TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         ));
    ///     })
    /// #    .id();
    /// #
    /// # queue.apply(&mut app.world);
    /// #
    /// # let children = app.world.entity(sprite).get::<Children>().unwrap();
    /// # assert!(entity_eq(&app.world, children[0], children[1]));
    /// ```
    pub fn tween_exact(
        &mut self,
        span: impl TryInto<TimeSpan, Error = impl std::fmt::Debug>,
        interpolation: impl Bundle,
        tween: impl Bundle,
    ) -> &mut Self {
        self.tween_exact_and(span, interpolation, tween, |_| {})
    }

    // Due to current limitations in the borrow checker, `FnOnce` implies a `'static` lifetime.
    // Privated until the limitation is lift.
    /// Create a new span tween with the supplied span then call a closure on it.
    fn tween_exact_and(
        &mut self,
        span: impl TryInto<TimeSpan, Error = impl std::fmt::Debug>,
        interpolation: impl Bundle,
        tween: impl Bundle,
        f: impl FnOnce(S::CommandOutput<'_>),
    ) -> &mut Self {
        let commands = self.entity_spawner.spawn((
            span.try_into().unwrap(),
            interpolation,
            tween,
        ));
        f(commands);
        self
    }

    // Due to current limitations in the borrow checker, `FnOnce` implies a `'static` lifetime.
    // Privated until the limitation is lift.
    /// Create a new span tween with the supplied duration starting from
    /// previous tween then call a closure on it.
    ///
    /// [`tween()`]: Self::tween
    /// [`tween_and()`]: Self::tween_and
    /// [`tween_exact()`]: Self::tween_exact
    /// [`tween_exact_and()`]: Self::tween_exact_and
    fn tween_and(
        &mut self,
        duration: Duration,
        interpolation: impl Bundle,
        tween: impl Bundle,
        f: impl FnOnce(S::CommandOutput<'_>),
    ) -> &mut Self {
        let start = self.offset;
        let end = self.offset + duration;
        self.offset = end;
        self.tween_exact_and(start..end, interpolation, tween, f)
    }

    /// Create a new span tween with the supplied duration starting from
    /// previous tween.
    /// Shifting the internal offset forward by the supplied duration.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = utils::doc_entity_eq_fn!()]
    #[doc = utils::doc_app_test_boilerplate!()]
    /// # let sprite =
    /// commands
    ///     .spawn((
    ///         SpriteBundle::default(),
    ///         TweenerBundle::new(Duration::from_secs(1)),
    ///     ))
    ///     .with_children(|c| {
    ///         c.tweens()
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Translation {
    ///                     start: Vec3::ZERO,
    ///                     end: Vec3::ONE,
    ///                 }),
    ///             )
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Translation {
    ///                     start: Vec3::ONE,
    ///                     end: Vec3::ONE * 2.,
    ///                 }),
    ///             );
    ///
    ///         // is exactly the same as
    ///
    ///         c.spawn((
    ///             TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         ));
    ///         c.spawn((
    ///             TimeSpan::try_from(
    ///                 Duration::from_secs(1)..Duration::from_secs(2)
    ///             ).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ONE,
    ///                 end: Vec3::ONE * 2.,
    ///             }),
    ///         ));
    ///     })
    /// #    .id();
    /// #
    /// # queue.apply(&mut app.world);
    /// #
    /// # let children = app.world.entity(sprite).get::<Children>().unwrap();
    /// # assert!(entity_eq(&app.world, children[0], children[2]));
    /// # assert!(entity_eq(&app.world, children[1], children[3]));
    /// ```
    /// 
    /// [`tween()`]: Self::tween
    /// [`tween_exact()`]: Self::tween_exact
    pub fn tween(
        &mut self,
        duration: Duration,
        interpolation: impl Bundle,
        tween: impl Bundle,
    ) -> &mut Self {
        self.tween_and(duration, interpolation, tween, |_| {})
    }

    /// Get the internal offset.
    pub fn offset(&self) -> Duration {
        self.offset
    }

    /// Set the internal offset to the supplied duration.
    pub fn go(&mut self, duration: Duration) -> &mut Self {
        self.offset = duration;
        self
    }

    /// Shifts the internal offset foward by the supplied duration.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = utils::doc_entity_eq_fn!()]
    #[doc = utils::doc_app_test_boilerplate!()]
    /// # let sprite =
    /// commands
    ///     .spawn((
    ///         SpriteBundle::default(),
    ///         TweenerBundle::new(Duration::from_secs(1)),
    ///     ))
    ///     .with_children(|c| {
    ///         c.tweens()
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Translation {
    ///                     start: Vec3::ZERO,
    ///                     end: Vec3::ONE,
    ///                 }),
    ///             )
    ///             .forward(Duration::from_secs(1))
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Translation {
    ///                     start: Vec3::ONE,
    ///                     end: Vec3::ONE * 2.,
    ///                 }),
    ///             );
    ///
    ///         // is exactly the same as
    ///
    ///         c.spawn((
    ///             TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         ));
    ///         c.spawn((
    ///             TimeSpan::try_from(
    ///                 Duration::from_secs(2)..Duration::from_secs(3)
    ///             ).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ONE,
    ///                 end: Vec3::ONE * 2.,
    ///             }),
    ///         ));
    ///     })
    /// #    .id();
    /// #
    /// # queue.apply(&mut app.world);
    /// #
    /// # let children = app.world.entity(sprite).get::<Children>().unwrap();
    /// # assert!(entity_eq(&app.world, children[0], children[2]));
    /// # assert!(entity_eq(&app.world, children[1], children[3]));
    /// ```
    #[doc(alias = "delay")]
    pub fn forward(&mut self, duration: Duration) -> &mut Self {
        self.offset += duration;
        self
    }

    /// Shifts the internal offset backward by the supplied duration.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = utils::doc_entity_eq_fn!()]
    #[doc = utils::doc_app_test_boilerplate!()]
    /// # let sprite =
    /// commands
    ///     .spawn((
    ///         SpriteBundle::default(),
    ///         TweenerBundle::new(Duration::from_secs(1)),
    ///     ))
    ///     .with_children(|c| {
    ///         c.tweens()
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Translation {
    ///                     start: Vec3::ZERO,
    ///                     end: Vec3::ONE,
    ///                 }),
    ///             )
    ///             .backward(Duration::from_secs(1))
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Scale {
    ///                     start: Vec3::ZERO,
    ///                     end: Vec3::ONE,
    ///                 }),
    ///             );
    ///
    ///         // is exactly the same as
    ///
    ///         c.spawn((
    ///             TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         ));
    ///         c.spawn((
    ///             TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Scale {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         ));
    ///     })
    /// #    .id();
    /// #
    /// # queue.apply(&mut app.world);
    /// #
    /// # let children = app.world.entity(sprite).get::<Children>().unwrap();
    /// # assert!(entity_eq(&app.world, children[0], children[2]));
    /// # assert!(entity_eq(&app.world, children[1], children[3]));
    /// ```
    pub fn backward(&mut self, duration: Duration) -> &mut Self {
        self.offset = self.offset.saturating_sub(duration);
        self
    }

    /// Save the current offset to a variable.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = utils::doc_entity_eq_fn!()]
    #[doc = utils::doc_app_test_boilerplate!()]
    /// # let sprite =
    /// commands
    ///     .spawn((
    ///         SpriteBundle::default(),
    ///         TweenerBundle::new(Duration::from_secs(1)),
    ///     ))
    ///     .with_children(|c| {
    ///         let mut middle = Duration::default();
    ///         c.tweens()
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Translation {
    ///                     start: Vec3::ZERO,
    ///                     end: Vec3::ONE,
    ///                 }),
    ///             )
    ///             .store_offset(&mut middle)
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Translation {
    ///                     start: Vec3::ONE,
    ///                     end: Vec3::ONE * 2.,
    ///                 }),
    ///             )
    ///             .go(middle)
    ///             .tween(
    ///                 Duration::from_secs(1),
    ///                 EaseFunction::Linear,
    ///                 ComponentTween::tweener_entity(interpolate::Scale {
    ///                     start: Vec3::ZERO,
    ///                     end: Vec3::ONE,
    ///                 }),
    ///             );
    ///
    ///         // is exactly the same as
    ///
    ///         c.spawn((
    ///             TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         ));
    ///         c.spawn((
    ///             TimeSpan::try_from(Duration::from_secs(1)..Duration::from_secs(2)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ONE,
    ///                 end: Vec3::ONE * 2.,
    ///             }),
    ///         ));
    ///         c.spawn((
    ///             TimeSpan::try_from(Duration::from_secs(1)..Duration::from_secs(2)).unwrap(),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Scale {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             }),
    ///         ));
    ///     })
    /// #    .id();
    /// #
    /// # queue.apply(&mut app.world);
    /// #
    /// # let children = app.world.entity(sprite).get::<Children>().unwrap();
    /// # assert!(entity_eq(&app.world, children[0], children[3]));
    /// # assert!(entity_eq(&app.world, children[1], children[4]));
    /// # assert!(entity_eq(&app.world, children[2], children[5]));
    /// ```
    pub fn store_offset(&mut self, v: &mut Duration) -> &mut Self {
        *v = self.offset;
        self
    }

    /// Create a tween event at the supplied span
    ///
    /// <div class="warning">
    ///
    /// The internal offset do not change after this call!
    ///
    /// </div>
    pub fn tween_event_exact<Data: Send + Sync + 'static>(
        &mut self,
        span: impl TryInto<TimeSpan, Error = impl std::fmt::Debug>,
        data: TweenEventData<Data>,
    ) -> &mut Self {
        self.entity_spawner.spawn((span.try_into().unwrap(), data));
        self
    }

    /// Create a tween event at the current offset
    pub fn tween_event<Data: Send + Sync + 'static>(
        &mut self,
        data: TweenEventData<Data>,
    ) -> &mut Self {
        self.tween_event_for(Duration::ZERO, data)
    }

    /// Create a tween event for the supplied duration at the current offset.
    /// Shifting the internal offset forward by the supplied duration.
    pub fn tween_event_for<Data: Send + Sync + 'static>(
        &mut self,
        duration: Duration,
        data: TweenEventData<Data>,
    ) -> &mut Self {
        let start = self.offset;
        let end = self.offset + duration;
        self.tween_event_exact(start..end, data);
        self.offset = end;
        self
    }

    /// Accept types that implement [`TweenPreset`].
    /// This method can be understand as a method that "adds an animation preset"
    /// though technically it can do more than that.
    ///
    /// This adds an interesting abstraction design that allow you to
    /// - reuse a group of animation or so-called preset.
    /// - organize your animations into sizable pieces.
    ///
    /// # Examples
    ///
    /// ```
    #[doc = utils::doc_entity_eq_fn!()]
    #[doc = utils::doc_app_test_boilerplate!()]
    /// use bevy_tween::prelude::*;
    /// use bevy_tween::tweener::{TweensBuilder, EntitySpawner};
    /// use bevy_tween::tween::TargetComponent::{self, TweenerEntity};
    ///
    /// fn up_down<E: EntitySpawner>(
    ///     target: impl Into<TargetComponent>,
    ///     part_a: Duration,
    ///     part_b: Duration,
    /// ) -> impl FnOnce(&mut TweensBuilder<E>) {
    ///     let target = target.into();
    ///     move |b| {
    ///         b.tween(
    ///             part_a,
    ///             EaseFunction::Linear,
    ///             ComponentTween::new_target(
    ///                 target.clone(),
    ///                 interpolate::Translation {
    ///                     start: Vec3::ZERO,
    ///                     end: Vec3::ONE,
    ///                 },
    ///             ),
    ///         )
    ///         .tween(
    ///             part_b,
    ///             EaseFunction::Linear,
    ///             ComponentTween::new_target(
    ///                 target,
    ///                 interpolate::Translation {
    ///                     start: Vec3::ONE,
    ///                     end: Vec3::ZERO,
    ///                 },
    ///             ),
    ///         );
    ///     }
    /// }
    ///
    /// fn secs(secs: f32) -> Duration {
    ///     Duration::from_secs_f32(secs)
    /// }
    ///
    /// # let sprite =
    /// commands.spawn((
    ///     SpriteBundle::default(),
    ///     TweenerBundle::new(Duration::from_secs(9))
    /// )).with_children(|c| {
    ///     c.tweens()
    ///         .add(up_down(TweenerEntity, secs(1.), secs(2.)))
    ///         .add(up_down(TweenerEntity, secs(2.), secs(1.)))
    ///         .add(up_down(TweenerEntity, secs(1.), secs(2.)));
    ///
    ///     // is exactly the same as
    ///     // (Look how much code we just saved ourselves from!)
    ///
    ///     c.tweens()
    ///         .tween(
    ///             secs(1.),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             })
    ///         ).tween(
    ///             secs(2.),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ONE,
    ///                 end: Vec3::ZERO,
    ///             })
    ///         )
    ///         .tween(
    ///             secs(2.),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             })
    ///         ).tween(
    ///             secs(1.),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ONE,
    ///                 end: Vec3::ZERO,
    ///             })
    ///         )
    ///         .tween(
    ///             secs(1.),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ZERO,
    ///                 end: Vec3::ONE,
    ///             })
    ///         ).tween(
    ///             secs(2.),
    ///             EaseFunction::Linear,
    ///             ComponentTween::tweener_entity(interpolate::Translation {
    ///                 start: Vec3::ONE,
    ///                 end: Vec3::ZERO,
    ///             })
    ///         );
    /// })
    /// # .id();
    /// #
    /// # queue.apply(&mut app.world);
    /// #
    /// # let children = app.world.entity(sprite).get::<Children>().unwrap();
    /// # assert!(entity_eq(&app.world, children[0], children[6]));
    /// # assert!(entity_eq(&app.world, children[1], children[7]));
    /// # assert!(entity_eq(&app.world, children[2], children[8]));
    /// # assert!(entity_eq(&app.world, children[3], children[9]));
    /// # assert!(entity_eq(&app.world, children[4], children[10]));
    /// # assert!(entity_eq(&app.world, children[5], children[11]));
    /// ```
    pub fn add(&mut self, f: impl TweenPreset<E>) -> &mut Self {
        f.build(self);
        self
    }
}

/// Extension trait that allows you to quickly construct [`TweensBuilder`]
///
/// This trait is sealed and not meant to be implemented outside of the current crate.
pub trait TweensBuilderExt: sealed::Sealed {
    /// Output from [`Self::tweens()`]
    type Output<'a>
    where
        Self: 'a;
    /// Create a [`TweensBuilder`] from this thing
    #[deprecated(
        since = "0.5.0",
        note = "use TweensBuilderExt::tweens instead"
    )]
    fn span_tweens(&mut self) -> Self::Output<'_>;
    /// Create a [`TweensBuilder`] from this thing
    fn tweens(&mut self) -> Self::Output<'_>;
}

impl<E> TweensBuilderExt for E
where
    E: EntitySpawner,
{
    type Output<'a> = TweensBuilder<'a, Self>
    where
        Self: 'a;

    fn span_tweens(&mut self) -> Self::Output<'_> {
        TweensBuilder::new(self)
    }

    fn tweens(&mut self) -> Self::Output<'_> {
        TweensBuilder::new(self)
    }
}

/// Reusuable group of span tweens animation, a preset.
/// Use with [`TweensBuilder::add`].
pub trait TweenPreset<E: EntitySpawner> {
    /// Build this preset to the supplied [`TweensBuilder`]
    fn build(self, b: &mut TweensBuilder<E>);
}

impl<E, F> TweenPreset<E> for F
where
    E: EntitySpawner,
    F: FnOnce(&mut TweensBuilder<E>),
{
    fn build(self, b: &mut TweensBuilder<E>) {
        self(b)
    }
}