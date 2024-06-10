//! Module containing implementations for simple event
//!
//! # Tween event
//!
//! **Plugins**:
//! - [`DefaultTweenEventPlugins`]
//! - [`TweenEventPlugin<Data>`]
//! - [`TweenEventTakingPlugin<Data>`]
//!
//! **Components**:
//! - [`TweenEventData`]
//!
//! **Systems**
//! - [`tween_event_system`]
//! - [`tween_event_taking_system`]
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
//! - [`TweenEventTakingPlugin<Data>`]
//!
//! See [`DefaultTweenEventPlugins`] for default events which is also added in
//! [`DefaultTweenPlugins`](crate::DefaultTweenPlugins)

use std::marker::PhantomData;

use bevy::{app::PluginGroupBuilder, prelude::*};

#[cfg(feature = "bevy_eventlistener")]
use bevy_eventlistener::prelude::*;
use bevy_time_runner::TimeSpanProgress;

use crate::tween::{SkipTween, TweenInterpolationValue};

/// Plugin for simple generic event that fires at a specific time span
/// See [`TweenEventTakingPlugin`] if your custom data is not [`Clone`].
#[derive(Default)]
pub struct TweenEventPlugin<Data>
where
    Data: Send + Sync + 'static + Clone,
{
    marker: PhantomData<Data>,
}

impl<Data> Plugin for TweenEventPlugin<Data>
where
    Data: Send + Sync + 'static + Clone,
{
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            (tween_event_system::<Data>)
                .in_set(crate::TweenSystemSet::ApplyTween),
        )
        .add_event::<TweenEvent<Data>>();
    }
}

/// Plugin for simple generic event that fires at a specific time span
/// See [`TweenEventPlugin`] if your custom data is [`Clone`].
pub struct TweenEventTakingPlugin<Data>
where
    Data: Send + Sync + 'static,
{
    marker: PhantomData<Data>,
}

impl<Data> Plugin for TweenEventTakingPlugin<Data>
where
    Data: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            (tween_event_taking_system::<Data>)
                .in_set(crate::TweenSystemSet::ApplyTween),
        )
        .add_event::<TweenEvent<Data>>();
    }
}

/// Default tween event plugins:
/// - `TweenEventPlugin::<()>::default()`,
/// - `TweenEventPlugin::<&'static str>::default()`
pub struct DefaultTweenEventPlugins;

impl DefaultTweenEventPlugins {
    pub(crate) fn plugins(
    ) -> (TweenEventPlugin<()>, TweenEventPlugin<&'static str>) {
        (
            TweenEventPlugin::<()>::default(),
            TweenEventPlugin::<&'static str>::default(),
        )
    }
}

impl PluginGroup for DefaultTweenEventPlugins {
    fn build(self) -> PluginGroupBuilder {
        let plugins = Self::plugins();
        PluginGroupBuilder::start::<DefaultTweenEventPlugins>()
            .add(plugins.0)
            .add(plugins.1)
    }
}

/// Fires [`TweenEvent`] whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the same entity.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct TweenEventData<Data = ()>(pub Option<Data>)
where
    Data: Send + Sync + 'static;

impl<Data: Send + Sync + 'static> TweenEventData<Data> {
    /// Create new [`TweenEventData`] with custom user data.
    pub fn with_data(data: Data) -> Self {
        TweenEventData(Some(data))
    }
}

impl TweenEventData<()> {
    /// Create new [`TweenEventData`] with no custom user data, simply `Some(())`.
    pub fn new() -> Self {
        TweenEventData(Some(()))
    }
}

impl<Data> TweenEventData<Data>
where
    Data: Send + Sync + 'static,
{
    /// Create new [`TweenEventData`] with `None` value.
    pub fn none() -> Self {
        TweenEventData(None)
    }
}

/// Fires whenever [`TimeSpanProgress`] and [`TweenEventData`] exist in the same entity
/// by [`tween_event_system`] or [`tween_event_taking_system`].
#[derive(Debug, Clone, PartialEq, Event, Reflect)]
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

#[cfg(feature = "bevy_eventlistener")]
impl<Data> EntityEvent for TweenEvent<Data>
where Data: Clone + Send + Sync + 'static
{
    fn target(&self) -> Entity {
        self.entity
    }
    fn can_bubble(&self) -> bool {
        true
    }
}

/// Fires [`TweenEvent`] with optional user data whenever [`TimeSpanProgress`]
/// and [`TweenEventData`] exist in the same entity and data is `Some`,
/// cloning the data.
#[allow(clippy::type_complexity)]
pub fn tween_event_system<Data>(
    q_tween_event_data: Query<
        (
            Entity,
            &TweenEventData<Data>,
            &TimeSpanProgress,
            Option<&TweenInterpolationValue>,
        ),
        Without<SkipTween>,
    >,
    mut event_writer: EventWriter<TweenEvent<Data>>,
) where
    Data: Clone + Send + Sync + 'static,
{
    q_tween_event_data.iter().for_each(
        |(entity, event_data, progress, interpolation_value)| {
            if let Some(data) = event_data.0.as_ref() {
                event_writer.send(TweenEvent {
                    data: data.clone(),
                    progress: *progress,
                    interpolation_value: interpolation_value.map(|v| v.0),
                    entity,
                });
            }
        },
    );
}

/// Fires [`TweenEvent`] with optional user data whenever [`TimeSpanProgress`]
/// and [`TweenEventData`] exist in the same entity and data is `Some`,
/// taking the data and leaves the value `None`.
#[allow(clippy::type_complexity)]
pub fn tween_event_taking_system<Data>(
    mut q_tween_event_data: Query<
        (
            Entity,
            &mut TweenEventData<Data>,
            &TimeSpanProgress,
            Option<&TweenInterpolationValue>,
        ),
        Without<SkipTween>,
    >,
    mut event_writer: EventWriter<TweenEvent<Data>>,
) where
    Data: Send + Sync + 'static,
{
    q_tween_event_data.iter_mut().for_each(
        |(entity, mut event_data, progress, interpolation_value)| {
            if let Some(data) = event_data.0.take() {
                event_writer.send(TweenEvent {
                    data,
                    progress: *progress,
                    interpolation_value: interpolation_value.map(|v| v.0),
                    entity,
                });
            }
        },
    );
}
