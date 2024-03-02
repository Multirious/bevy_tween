use bevy::prelude::*;
use std::{ops, time::Duration};

use crate::{
    tween::TweenState,
    tween_player::{AnimationDirection, TweenPlayerState},
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

    pub fn new(min: TimeBound, max: TimeBound) -> Option<TweenTimeSpan> {
        match (min, max) {
            (TimeBound::Exclusive(_), TimeBound::Exclusive(_))
                if min.duration() != max.duration() =>
            {
                Some(Self::new_unchecked(min, max))
            }
            _ if min.duration() <= max.duration() => {
                Some(Self::new_unchecked(min, max))
            }
            _ => None,
        }
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
        TweenTimeSpan::from(Duration::ZERO..=Duration::ZERO)
    }
}

impl From<ops::Range<Duration>> for TweenTimeSpan {
    fn from(range: ops::Range<Duration>) -> Self {
        TweenTimeSpan {
            min: TimeBound::Inclusive(range.start),
            max: TimeBound::Exclusive(range.end),
        }
    }
}
impl From<ops::RangeInclusive<Duration>> for TweenTimeSpan {
    fn from(range: ops::RangeInclusive<Duration>) -> Self {
        TweenTimeSpan {
            min: TimeBound::Inclusive(*range.start()),
            max: TimeBound::Exclusive(*range.end()),
        }
    }
}

impl From<ops::RangeTo<Duration>> for TweenTimeSpan {
    fn from(range: ops::RangeTo<Duration>) -> Self {
        TweenTimeSpan {
            min: TimeBound::Inclusive(Duration::ZERO),
            max: TimeBound::Exclusive(range.end),
        }
    }
}

impl From<ops::RangeToInclusive<Duration>> for TweenTimeSpan {
    fn from(range: ops::RangeToInclusive<Duration>) -> Self {
        TweenTimeSpan {
            min: TimeBound::Inclusive(Duration::ZERO),
            max: TimeBound::Exclusive(range.end),
        }
    }
}

#[derive(Default, Bundle)]
pub struct SpanTweenPlayerBundle {
    pub tween_player: TweenPlayerState,
    pub span_player: SpanTweenPlayer,
}

impl SpanTweenPlayerBundle {
    pub fn new(tween_player: TweenPlayerState) -> Self {
        SpanTweenPlayerBundle {
            tween_player,
            ..Default::default()
        }
    }
}

#[derive(Default, Bundle)]
pub struct SpanTweenBundle<I>
where
    I: Send + Sync + 'static + Component,
{
    pub span: TweenTimeSpan,
    pub interpolation: I,
    pub state: TweenState,
}

impl<E> SpanTweenBundle<E>
where
    E: Send + Sync + 'static + Component,
{
    pub fn new<S: Into<TweenTimeSpan>>(span: S, interpolation: E) -> Self {
        SpanTweenBundle {
            span: span.into(),
            interpolation,
            state: Default::default(),
        }
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
