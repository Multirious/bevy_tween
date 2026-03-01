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
//! - [`tween_event_system`]
//!
//! **Events**:
//! - [`TweenEvent<Data>`]
//!
//! Simple event system that fires generic data in a timed manner.
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
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
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
    pub fn in_schedule(schedule: InternedScheduleLabel) -> Self {
        Self {
            schedule,
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
        #[allow(deprecated)]
        let schedule = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .map(|a| a.schedule)
            .unwrap_or(self.schedule);
        app.add_systems(
            schedule,
            (tween_event_system::<Data, TimeCtx>)
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
    pub fn in_schedule(schedule: InternedScheduleLabel) -> Self {
        Self {
            schedule,
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
#[require(EventEmittingTween)]
pub struct TweenEventData<Data = ()>(pub Data)
where
    Data: Send + Sync + 'static;

/// Used to mark event-emitting tweens (tweens with `TweenEventData<Data>` for some registered `Data`)
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

/// Fires whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the same entity
/// by [`tween_event_system`].
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

/// Fires [`TweenEvent`] with optional user data whenever [`TimeSpanProgress`]
/// and [`TweenEventData`] exist in the same entity and data is `Some`,
/// cloning the data.
#[allow(clippy::type_complexity)]
pub fn tween_event_system<Data, TimeCtx>(
    mut commands: Commands,
    q_tween_event_data: Query<
        (
            Entity,
            &TweenEventData<Data>,
            &TimeSpanProgress,
            Option<&TweenInterpolationValue>,
        ),
        (Without<SkipTween>, With<TimeContext<TimeCtx>>),
    >,
    mut event_writer: MessageWriter<TweenEvent<Data>>,
) where
    Data: Clone + Send + Sync + 'static,
    TimeCtx: Default + Send + Sync + 'static,
{
    q_tween_event_data.iter().for_each(
        |(entity, event_data, progress, interpolation_value)| {
            let event = TweenEvent {
                data: event_data.0.clone(),
                progress: *progress,
                interpolation_value: interpolation_value.map(|v| v.0),
                entity,
            };
            commands.trigger(event.clone());
            event_writer.write(event);
        },
    );
}
