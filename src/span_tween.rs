use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{ops, time::Duration};

use crate::{
    interpolation::Interpolator,
    tween::TweenState,
    tween_player::{self, AnimationDirection, TweenPlayerState},
};

#[derive(Debug)]
pub struct SpanTweenPlugin;
impl Plugin for SpanTweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            span_tween_player_system.in_set(crate::TweenSystemSet::TweenPlayer),
        )
        .register_type::<SpanTweenPlayer>()
        .register_type::<TimeBound>()
        .register_type::<DurationQuotient>()
        .register_type::<TweenTimeSpan>();
    }
}

#[derive(Debug, Default, Component, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct SpanTweenPlayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TimeBound {
    Inclusive(Duration),
    Exclusive(Duration),
}

impl TimeBound {
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

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect,
)]
pub enum DurationQuotient {
    #[default]
    Before,
    Inside,
    After,
}

#[derive(Debug)]
pub enum NewTweenTimeSpanError {
    NotTime { min: TimeBound, max: TimeBound },
    MinGreaterThanMax { min: TimeBound, max: TimeBound },
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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TweenTimeSpan {
    min: TimeBound,
    max: TimeBound,
}
impl TweenTimeSpan {
    pub(crate) fn new_unchecked(
        min: TimeBound,
        max: TimeBound,
    ) -> TweenTimeSpan {
        TweenTimeSpan { min, max }
    }

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

    pub fn quotient(&self, duration: Duration) -> DurationQuotient {
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
    pub fn min(&self) -> TimeBound {
        self.min
    }
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
            TimeBound::Exclusive(*range.end()),
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
            TimeBound::Exclusive(range.end),
        )
    }
}

#[derive(Default, Bundle)]
pub struct SpanTweenPlayerBundle {
    pub tween_player: TweenPlayerState,
    pub span_player: SpanTweenPlayer,
}

impl SpanTweenPlayerBundle {
    pub fn new(duration: Duration) -> Self {
        let mut t = SpanTweenPlayerBundle::default();
        t.tween_player.set_duration(duration);
        t
    }
    pub fn with_paused(mut self, paused: bool) -> Self {
        self.tween_player.set_paused(paused);
        self
    }
    pub fn with_elasped(mut self, elasped: Duration) -> Self {
        self.tween_player.set_elasped(elasped);
        self
    }
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.tween_player.set_direction(direction);
        self
    }

    pub fn with_repeat(mut self, repeat: Option<tween_player::Repeat>) -> Self {
        self.tween_player.set_repeat(repeat);
        self
    }
    pub fn with_repeat_style(
        mut self,
        repeat_style: Option<tween_player::RepeatStyle>,
    ) -> Self {
        self.tween_player.set_repeat_style(repeat_style);
        self
    }
}

impl From<TweenPlayerState> for SpanTweenPlayerBundle {
    fn from(value: TweenPlayerState) -> Self {
        SpanTweenPlayerBundle {
            tween_player: value,
            span_player: SpanTweenPlayer,
        }
    }
}

#[derive(Default, Bundle)]
pub struct SpanTweenBundle<I>
where
    I: Component + Interpolator,
{
    pub span: TweenTimeSpan,
    pub interpolation: I,
    pub state: TweenState,
}

impl<I> SpanTweenBundle<I>
where
    I: Component + Interpolator,
{
    pub fn new<S>(span: S, interpolation: I) -> Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
    {
        SpanTweenBundle {
            span: span.try_into().expect("valid span"),
            interpolation,
            state: Default::default(),
        }
    }

    pub fn with_interpolation<NewI>(
        self,
        interpolation: NewI,
    ) -> SpanTweenBundle<NewI>
    where
        NewI: Component + Interpolator,
    {
        SpanTweenBundle {
            span: self.span,
            interpolation,
            state: self.state,
        }
    }

    pub fn with_span<S>(mut self, span: S) -> Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
    {
        self.span = span.try_into().expect("valid span");
        self
    }
}

pub fn span_tween_player_system(
    q_no_tween_player: Query<(), Without<TweenPlayerState>>,
    q_tween_span_player: Query<
        (Entity, &TweenPlayerState, Option<&Children>),
        With<SpanTweenPlayer>,
    >,
    mut q_tween: Query<(&mut TweenState, &TweenTimeSpan)>,
) {
    use crate::tween_player::RepeatStyle::*;
    use AnimationDirection::*;
    use DurationQuotient::*;
    q_tween_span_player
        .iter()
        .for_each(|(player_entity, player, children)| {
            let children = children
                .iter()
                .flat_map(|a| a.iter())
                .filter(|c| q_no_tween_player.contains(**c));
            let tweens = [&player_entity].into_iter().chain(children);
            for &tween_entity in tweens {
                let Ok((mut tween_state, tween_span)) =
                    q_tween.get_mut(tween_entity)
                else {
                    continue;
                };

                let elasped_quotient = tween_span.quotient(player.elasped.now);
                let previous_quotient =
                    tween_span.quotient(player.elasped.previous);

                let tween_min = Duration::ZERO;
                let tween_max =
                    tween_span.max().duration() - tween_span.min().duration();
                let tween_elasped = player
                    .elasped
                    .now
                    .saturating_sub(tween_span.min().duration())
                    .min(tween_max);
                // Look at this behemoth of edge case handling.
                //
                // I manually take care of all this shit out.
                // The edge cases are the time when the tween are really short
                // or delta is really long per frame.
                //
                // This is likely only an issue with this player implementation.
                //
                // This is not accounted for when the tween might repeat
                // multiple time in one frame. When that tween is this ridiculously
                // fast or the game heavily lagged,
                // I don't think that need to be accounted.
                let new_tween_elasped = match (
                    player.direction,
                    previous_quotient,
                    elasped_quotient,
                    player.elasped.repeat_style,
                ) {
                    (_, Inside, Inside, None) => Some(tween_elasped),
                    // -------------------------------------------------------
                    (Forward, Before, Inside, None)
                        | (Forward, Inside, After, None)
                        | (Forward, Before, After, None)
                        => Some(tween_elasped),
                    // -------------------------------------------------------
                    (Backward, After, Inside, None)
                        | (Backward, Inside, Before, None)
                        | (Backward, After, Before, None)
                        => Some(tween_elasped),
                    // --------------------------------------------------------
                    (Forward, Before, Before, Some(WrapAround)) // 1&2 max
                        | (Forward, Inside, Before, Some(WrapAround)) // 1 max
                        => Some(tween_max),
                    (Forward, Before, Inside, Some(WrapAround)) // 2 now
                        | (Forward, Before, After, Some(WrapAround)) // 2 now, max
                        | (Forward, Inside, Inside, Some(WrapAround)) // 1&2 now
                        | (Forward, Inside, After, Some(WrapAround)) // 2 now, max
                        | (Forward, After, Inside, Some(WrapAround)) // 1 now 
                        | (Forward, After, After, Some(WrapAround)) // 1&2 now, max
                        // | (Forward, After, Before, Some(WrapAround)) // 1
                        => Some(tween_elasped),
                    // -------------------------------------------------------
                    (Backward, After, After, Some(WrapAround)) // 1&2 min
                        | (Backward, Inside, After, Some(WrapAround)) // 1 min
                        => Some(tween_min),
                    (Backward, Before, Before, Some(WrapAround)) // 1&2 now, min
                        | (Backward, Before, Inside, Some(WrapAround)) // 1 now 
                        | (Backward, Inside, Before, Some(WrapAround)) // 2 now, min
                        | (Backward, Inside, Inside, Some(WrapAround)) // 1&2 now
                        | (Backward, After, Before, Some(WrapAround)) // 2 now, min
                        | (Backward, After, Inside, Some(WrapAround)) // 2 now
                        // | (Backward, Before, After, Some(WrapAround)) // 1
                        => Some(tween_elasped),
                    // -------------------------------------------------------
                    (Backward, Before, Before, Some(PingPong)) // 1&2 now, min
                        | (Backward, Before, Inside, Some(PingPong)) // 1 now
                        | (Backward, Before, After, Some(PingPong)) // 1 now, max
                        | (Backward, Inside, Before, Some(PingPong)) // 2 now, min
                        | (Backward, Inside, Inside, Some(PingPong)) // 1&2 now
                        | (Backward, Inside, After, Some(PingPong)) // 1 now, max
                        | (Backward, After, Before, Some(PingPong)) // 2 now, min
                        | (Backward, After, Inside, Some(PingPong)) // 2 now
                        => Some(tween_elasped),
                        // | (Backward, After, After, Some(PingPong)) // 1&2
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
                        => Some(tween_elasped),
                    _ => None,
                };
                let new_tween_state = TweenState {
                    local_elasped: new_tween_elasped,
                    local_previous_elasped: tween_state.local_elasped,
                    local_end: tween_max,
                    direction: player.direction,
                };
                *tween_state = new_tween_state;
            }
        });
}

pub trait BuildSpanTweens<'a> {
    fn build_tweens(&mut self) -> SpanTweensBuilder<'a, '_>;
}

impl<'a> BuildSpanTweens<'a> for ChildBuilder<'a> {
    fn build_tweens(&mut self) -> SpanTweensBuilder<'a, '_> {
        SpanTweensBuilder {
            child_builder: self,
        }
    }
}

pub struct SpanTweensBuilder<'a, 'b> {
    child_builder: &'b mut ChildBuilder<'a>,
}

impl<'a, 'b> SpanTweensBuilder<'a, 'b> {
    pub fn tween<S, I, T>(
        &mut self,
        span: S,
        interpolation: I,
        tween: T,
    ) -> &mut Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
        I: Component + Interpolator,
        T: Bundle,
    {
        self.child_builder
            .spawn((SpanTweenBundle::new(span, interpolation), tween));
        self
    }

    pub fn tween_and<S, I, T, F>(
        &mut self,
        span: S,
        interpolation: I,
        tween: T,
        f: F,
    ) -> &mut Self
    where
        S: TryInto<TweenTimeSpan>,
        S::Error: std::fmt::Debug,
        I: Component + Interpolator,
        T: Bundle,
        F: FnOnce(EntityCommands),
    {
        let commands = self
            .child_builder
            .spawn((SpanTweenBundle::new(span, interpolation), tween));
        f(commands);
        self
    }
}
