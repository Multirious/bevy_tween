use super::{AnimationCommands, BuildAnimation};
use crate::prelude::TweenEventData;
use bevy_time_runner::TimeSpan;
use std::time::Duration;

/// Combinator for creating an tween event.
///
/// Event will be emitted at current position.
///
/// Position is not mutated because the event has no length.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin)
/// or [`TweenEventTakingPlugin`](crate::tween_event::TweenEventTakingPlugin).
///
/// </div>
pub fn event<Data>(event_data: Data) -> BuildTweenEvent<Data>
where
    Data: Send + Sync + 'static,
{
    BuildTweenEvent {
        time: LengthOrSpan::Length(Duration::ZERO),
        event_data,
    }
}

/// Combinator for creating an tween event.
///
/// Event will be emitted at the provided position.
///
/// Position is not mutated because the operation is not relative.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin)
/// or [`TweenEventTakingPlugin`](crate::tween_event::TweenEventTakingPlugin).
///
/// </div>
pub fn event_at<Data>(at: Duration, event_data: Data) -> BuildTweenEvent<Data>
where
    Data: Send + Sync + 'static,
{
    BuildTweenEvent {
        time: LengthOrSpan::Span(TimeSpan::try_from(at..=at).unwrap()),
        event_data,
    }
}

/// Combinator for creating an tween event.
///
/// Event will be emitted at the current position for provided length every frame.
///
/// Position is at the end of the event.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin)
/// or [`TweenEventTakingPlugin`](crate::tween_event::TweenEventTakingPlugin).
///
/// </div>
pub fn event_for<Data>(
    length: Duration,
    event_data: Data,
) -> BuildTweenEvent<Data>
where
    Data: Send + Sync + 'static,
{
    BuildTweenEvent {
        time: LengthOrSpan::Length(length),
        event_data,
    }
}

/// Combinator for creating an tween event.
///
/// Event will be emitted at the provided span every frame.
///
/// Position is not mutated because the operation is not relative.
///
/// <div class="warning">
///
/// Your event should be registered with [`TweenEventPlugin`](crate::tween_event::TweenEventPlugin)
/// or [`TweenEventTakingPlugin`](crate::tween_event::TweenEventTakingPlugin).
///
/// </div>
pub fn event_exact<S, Data>(span: S, event_data: Data) -> BuildTweenEvent<Data>
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    Data: Send + Sync + 'static,
{
    BuildTweenEvent {
        time: LengthOrSpan::Span(span.try_into().unwrap()),
        event_data,
    }
}

pub struct BuildTweenEvent<D>
where
    D: Send + Sync + 'static,
{
    time: LengthOrSpan,
    event_data: D,
}

impl<D> BuildAnimation for BuildTweenEvent<D>
where
    D: Send + Sync + 'static,
{
    fn build(self, commands: &mut AnimationCommands, position: &mut Duration) {
        let span = match self.time {
            LengthOrSpan::Length(length) => {
                TimeSpan::try_from(*position..(*position + length)).unwrap()
            }
            LengthOrSpan::Span(span) => span,
        };
        commands.spawn((span, TweenEventData::with_data(self.event_data)));
    }
}

enum LengthOrSpan {
    Length(Duration),
    Span(TimeSpan),
}
