use super::AnimationCommands;
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
pub fn event<Data>(
    event_data: Data,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        a.spawn((
            TimeSpan::try_from(*pos..=*pos).unwrap(),
            TweenEventData::with_data(event_data),
        ));
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
pub fn event_at<Data>(
    at: Duration,
    event_data: Data,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    Data: Send + Sync + 'static,
{
    move |a, _pos| {
        a.spawn((
            TimeSpan::try_from(at..=at).unwrap(),
            TweenEventData::with_data(event_data),
        ));
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
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    Data: Send + Sync + 'static,
{
    move |a, pos| {
        let start = *pos;
        let end = start + length;
        a.spawn((
            TimeSpan::try_from(start..end).unwrap(),
            TweenEventData::with_data(event_data),
        ));
        *pos = end;
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
pub fn event_exact<S, Data>(
    span: S,
    event_data: Data,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration)
where
    S: TryInto<TimeSpan>,
    S::Error: std::fmt::Debug,
    Data: Send + Sync + 'static,
{
    move |a, _pos| {
        a.spawn((
            span.try_into().unwrap(),
            TweenEventData::with_data(event_data),
        ));
    }
}
