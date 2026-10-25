//! Module containing implementations for simple event
//!
//! # Tween event
//!
//! **Plugins**:
//! - [`DefaultTweenEventPlugins`]
//! - [`TweenEventPlugin<Data>`]
//!
//! **Components**:
//! - [`TweenEventData`]
//!
//! **Systems**
//! - [`tween_message_system`]
//! - [`tween_event_system`]
//!
//! **Events**:
//! - [`TweenEvent<Data>`]
//!
//! Simple event systems that fires generic data in a timed manner.
//! Your data is stored in [`TweenEventData`] and the event type is [`TweenEvent`].
//! Tween event is not necessarily related to tweening.
//!
//! Add this plugin for your custom data.
//! - [`TweenEventPlugin<Data>`]
//!
//! See [`DefaultTweenEventPlugins`] for default events which is also added in
//! [`DefaultTweenPlugins`](crate::DefaultTweenPlugins)

use bevy_time_runner::TimeContext;
use std::marker::PhantomData;

use bevy::{
    app::PluginGroupBuilder,
    ecs::{
        query::QueryData,
        schedule::{InternedScheduleLabel, ScheduleLabel},
    },
    prelude::*,
};

use bevy_time_runner::TimeSpanProgress;

use crate::tween::{SkipTween, TweenInterpolationValue};

/// A plugin for registering the tween event system for tween of type Data for the specified schedule
pub struct TweenEventPlugin<Data, TimeCtx = ()>
where
    Data: Send + Sync + 'static + Clone,
    TimeCtx: Default + Send + Sync + 'static,
{
    /// Register all systems from this plugin to the specified schedule.
    pub schedule: InternedScheduleLabel,
    marker: PhantomData<(Data, TimeCtx)>,
}
impl<Data, TimeCtx> TweenEventPlugin<Data, TimeCtx>
where
    Data: Send + Sync + 'static + Clone,
    TimeCtx: Default + Send + Sync + 'static,
{
    /// Register all systems from this plugin to the specified schedule.
    pub fn in_schedule(schedule: impl ScheduleLabel) -> Self {
        Self {
            schedule: schedule.intern(),
            marker: PhantomData,
        }
    }
}
impl<Data, TimeCtx> Plugin for TweenEventPlugin<Data, TimeCtx>
where
    Data: Send + Sync + 'static + Clone,
    TimeCtx: Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            self.schedule,
            (
                tween_message_system::<Data, TimeCtx>,
                tween_event_system::<Data, TimeCtx>,
            )
                .in_set(crate::TweenSystemSet::ApplyTween),
        )
        .add_message::<TweenEvent<Data>>();
    }
}

/// Default tween event plugins:
/// - `TweenEventPlugin::<()>::default()`,
/// - `TweenEventPlugin::<&'static str>::default()`
pub struct DefaultTweenEventPlugins<TimeCtx = ()>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    /// Register all systems from this plugin to the specified schedule.
    pub schedule: InternedScheduleLabel,
    marker: PhantomData<TimeCtx>,
}

impl<TimeCtx> PluginGroup for DefaultTweenEventPlugins<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    #[allow(unused)]
    #[allow(clippy::let_and_return)]
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<DefaultTweenEventPlugins>()
            .add(TweenEventPlugin::<(), TimeCtx>::in_schedule(self.schedule))
            .add(TweenEventPlugin::<&'static str, TimeCtx>::in_schedule(
                self.schedule,
            ))
    }
}

impl<TimeCtx> DefaultTweenEventPlugins<TimeCtx>
where
    TimeCtx: Default + Send + Sync + 'static,
{
    /// Register all systems from this plugin to the specified schedule.
    pub fn in_schedule(schedule: impl ScheduleLabel) -> Self {
        Self {
            schedule: schedule.intern(),
            marker: PhantomData,
        }
    }
}

impl Default for DefaultTweenEventPlugins<()> {
    fn default() -> Self {
        Self {
            schedule: PostUpdate.intern(),
            marker: Default::default(),
        }
    }
}

/// Fires [`TweenEvent`] whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the same entity.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct TweenEventData<Data = ()>(pub Data)
where
    Data: Send + Sync + 'static;

/// Used to mark message-emitting tweens (tweens that emit a tween event message)
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct MessageEmittingTween;

/// Used to mark root event-emitting tween (tweens that emit an entity event to the animation root)
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct EventEmittingTween;

impl<Data: Send + Sync + 'static> TweenEventData<Data> {
    /// Create new [`TweenEventData`] with custom user data.
    pub fn with_data(data: Data) -> Self {
        TweenEventData(data)
    }
}

impl TweenEventData<()> {
    /// Create new [`TweenEventData`] with no custom user data, simply `()`.
    pub fn new() -> Self {
        TweenEventData(())
    }
}

/// Fires whenever [`TimeSpanProgress`], [`TweenEventData`], and [`MessageEmittingTween`]/[`EventEmittingTween`] exist in the same entity
/// by [`tween_message_system`] and [`tween_event_system`].
#[derive(Debug, Clone, PartialEq, Message, Reflect, EntityEvent)]
pub struct TweenEvent<Data = ()> {
    /// Custom user data
    pub data: Data,
    /// Progress of the event
    pub progress: TimeSpanProgress,
    /// Sampled value of an interpolation.
    pub interpolation_value: Option<f32>,
    /// The entity that emitted the event
    pub entity: Entity,
}

/// QueryData used to construct a [`TweenEvent`] for a given [`TweenEventData`]
#[derive(QueryData)]
pub struct TweenSystemData<Data: Send + Sync + 'static> {
    /// The tween entity the event combinator created
    pub entity: Entity,
    /// The data passed to the event combinator
    pub event_data: &'static TweenEventData<Data>,
    /// The progress for the event
    pub progress: &'static TimeSpanProgress,
    /// The current interpolation value
    pub interpolation_value: Option<&'static TweenInterpolationValue>,
}

// Note we need to implement event for the data item,
// due to how the [`QueryData`] derive is implemented.
impl<'w, 's, Data> TweenSystemDataItem<'w, 's, Data>
where
    Data: Clone + Send + Sync + 'static,
{
    /// Constructs a tween event for the given tween data entity
    pub fn event(&self) -> TweenEvent<Data> {
        TweenEvent {
            data: self.event_data.0.clone(),
            progress: *self.progress,
            interpolation_value: self.interpolation_value.map(|v| v.0),
            entity: self.entity,
        }
    }

    /// Constructs a tween event for a different tween data entity
    pub fn event_on(&self, entity: Entity) -> TweenEvent<Data> {
        TweenEvent {
            entity,
            ..self.event()
        }
    }
}

/// Fires a [`TweenEvent`] message with optional user data
/// whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the
/// same entity and data is `Some`, cloning the data.
#[allow(clippy::type_complexity)]
pub fn tween_message_system<Data, TimeCtx>(
    q_tween_event_data: Query<
        TweenSystemData<Data>,
        (
            Without<SkipTween>,
            With<TimeContext<TimeCtx>>,
            With<MessageEmittingTween>,
        ),
    >,
    mut event_writer: MessageWriter<TweenEvent<Data>>,
) where
    Data: Clone + Send + Sync + 'static,
    TimeCtx: Default + Send + Sync + 'static,
{
    q_tween_event_data.iter().for_each(|data| {
        event_writer.write(data.event());
    });
}

/// Fires a [`TweenEvent`] to the animation root with optional user data
/// whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the
/// same entity and data is `Some`, cloning the data.
#[allow(clippy::type_complexity)]
pub fn tween_event_system<Data, TimeCtx>(
    mut commands: Commands,
    q_tween_event_data: Query<
        (&ChildOf, TweenSystemData<Data>),
        (
            Without<SkipTween>,
            With<TimeContext<TimeCtx>>,
            With<EventEmittingTween>,
        ),
    >,
) where
    Data: Clone + Send + Sync + 'static,
    TimeCtx: Default + Send + Sync + 'static,
{
    q_tween_event_data.iter().for_each(|(child_of, data)| {
        commands.trigger(data.event_on(child_of.parent()));
    });
}
