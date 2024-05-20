/// Interpolation types support for [`bevy_lookup_curve`](::bevy_lookup_curve)
use super::*;
use ::bevy_lookup_curve::{LookupCache, LookupCurve};

/// Use [`bevy_lookup_curve`](::bevy_lookup_curve) for interpolation.
pub struct BevyLookupCurveInterpolationPlugin;

impl Plugin for BevyLookupCurveInterpolationPlugin {
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` to be inserted to world");
        app.add_systems(
            app_resource.schedule,
            (
                sample_lookup_curve_system
                    .in_set(TweenSystemSet::UpdateInterpolationValue),
                // sample_interpolations_mut_system::<CurveCached>
                //     .in_set(TweenSystemSet::UpdateInterpolationValue),
            ),
        );
    }
}

/// Wrapper for [`LookupCache`] to make it a component
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LookupCurveCache(pub LookupCache);

/// Interpolation system for [`Handle<LookupCurve>`]
#[allow(clippy::type_complexity)]
pub fn sample_lookup_curve_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Handle<LookupCurve>,
            Option<&mut LookupCurveCache>,
            &TimeSpanProgress,
        ),
        Or<(Changed<Handle<LookupCurve>>, Changed<TimeSpanProgress>)>,
    >,
    mut removed: RemovedComponents<TimeSpanProgress>,
    lookup_curve: Res<Assets<LookupCurve>>,
) {
    query
        .iter_mut()
        .for_each(|(entity, curve, cache, progress)| {
            if progress.now_percentage.is_nan() {
                return;
            }

            let Some(curve) = lookup_curve.get(curve) else {
                error!("Curve handle is not valid");
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
    super::remove_removed(&mut commands, &mut removed);
}
