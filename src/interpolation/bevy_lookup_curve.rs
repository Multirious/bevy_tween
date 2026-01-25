//! Interpolation types support for [`bevy_lookup_curve`](::bevy_lookup_curve)
//!
//! **Plugins**:
//! - [`BevyLookupCurveInterpolationPlugin`]
//!
//! **Components**:
//! - [`LookupCurveCache`]
//!
//! **Systems**:
//! - [`sample_lookup_curve_system`]

use super::*;
use crate::InternedScheduleLabel;
use ::bevy_lookup_curve::{LookupCache, LookupCurve};
use bevy::platform::collections::HashSet;
use tracing::error;

/// Use [`bevy_lookup_curve`](::bevy_lookup_curve) for interpolation.
pub struct BevyLookupCurveInterpolationPlugin;

impl Plugin for BevyLookupCurveInterpolationPlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be inserted to world");
        app.add_plugins(
            BevyLookupCurveInterpolationForSchedulePlugin::<()>::on_schedule(
                app_resource.default_schedule,
            ),
        );
    }
}

/// Use [`bevy_lookup_curve`](::bevy_lookup_curve) for interpolation on the specified schedule
pub struct BevyLookupCurveInterpolationForSchedulePlugin<TimeStep>
where
    TimeStep: Default + Send + Sync + 'static,
{
    /// The systems' schedules
    pub schedule: InternedScheduleLabel,
    /// time step marker
    time_step_marker: PhantomData<TimeStep>,
}
impl<TimeStep> BevyLookupCurveInterpolationForSchedulePlugin<TimeStep>
where
    TimeStep: Default + Send + Sync + 'static,
{
    /// Constructor for that schedule
    pub fn on_schedule(schedule: InternedScheduleLabel) -> Self {
        Self {
            schedule,
            time_step_marker: PhantomData::default(),
        }
    }
}
impl<TimeStep> Plugin
    for BevyLookupCurveInterpolationForSchedulePlugin<TimeStep>
where
    TimeStep: Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            self.schedule.clone(),
            (
                sample_lookup_curve_system::<TimeStep>
                    .in_set(TweenSystemSet::UpdateInterpolationValue),
                // sample_interpolations_mut_system::<CurveCached>
                //     .in_set(TweenSystemSet::UpdateInterpolationValue),
            ),
        );
    }
}

/// Wrapper for [`LookupCache`] to make it a component
#[derive(Clone, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LookupCurveCache(pub LookupCache);

/// [`LookupCurve`] handle
#[derive(Clone, Debug, Component, Reflect)]
pub struct LookupCurveHandle(pub Handle<LookupCurve>);

/// Interpolation system for [`LookupCurveHandle`]
#[allow(clippy::type_complexity)]
pub fn sample_lookup_curve_system<TimeStep>(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &LookupCurveHandle,
            Option<&mut LookupCurveCache>,
            &TimeSpanProgress,
        ),
        (
            Or<(Changed<LookupCurveHandle>, Changed<TimeSpanProgress>)>,
            With<TimeStepMarker<TimeStep>>,
        ),
    >,
    mut removed: RemovedComponents<TimeSpanProgress>,
    lookup_curve: Res<Assets<LookupCurve>>,
    mut last_handle_error: Local<HashSet<AssetId<LookupCurve>>>,
) where
    TimeStep: Default + Send + Sync + 'static,
{
    let mut handle_error = HashSet::new();
    query
        .iter_mut()
        .for_each(|(entity, curve, cache, progress)| {
            if progress.now_percentage.is_nan() {
                return;
            }

            let curve = &curve.0;

            let Some(curve) = lookup_curve.get(curve) else {
                if !last_handle_error.contains(&curve.id())
                    && !handle_error.contains(&curve.id())
                {
                    error!(
                        "LookupCurve handle {} is not valid for interpolation",
                        curve.id()
                    );
                }
                handle_error.insert(curve.id());
                return;
            };
            let value = match cache {
                Some(mut cache) => curve.lookup_cached(
                    progress.now_percentage.clamp(0., 1.),
                    &mut cache.0,
                ),
                None => curve.lookup(progress.now_percentage.clamp(0., 1.)),
            };

            commands
                .entity(entity)
                .insert(TweenInterpolationValue(value));
        });

    removed.read().for_each(|entity| {
        if let Ok(mut entity) = commands.get_entity(entity) {
            entity.remove::<TweenInterpolationValue>();
        }
    });
    *last_handle_error = handle_error;
}
