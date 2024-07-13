//! Module containing ease functions and related systems.
//!
//! # [`Interpolation`]
//!
//! **Built-in interpolations**:
//! - [`EaseFunction`]
//! - [`EaseClosure`]
//!
//! **Systems**:
//! - [`sample_interpolations_system`]

use bevy::prelude::*;

use crate::{set::SetterValue, TweenSystemSet};
use bevy_time_runner::TimeSpanProgress;

// #[cfg(feature = "bevy_lookup_curve")]
// pub mod bevy_lookup_curve;
mod ease_function;
pub use ease_function::*;

/// Plugin for [`EaseClosure`]. In case you want to use custom an ease
/// function. Since most people likely wouldn't use this type, this plugin is
/// not with [`DefaultTweenPlugins`] to reduce unused system.
///
/// [`DefaultTweenPlugins`]: crate::DefaultTweenPlugins
pub struct EaseClosurePlugin;
impl Plugin for EaseClosurePlugin {
    /// # Panics
    ///
    /// Panics if [`TweenAppResource`] does not exist in world.
    ///
    /// [`TweenAppResource`]: crate::TweenAppResource
    fn build(&self, app: &mut App) {
        let app_resource = app
            .world()
            .get_resource::<crate::TweenAppResource>()
            .expect("`TweenAppResource` resource doesn't exist");
        app.add_systems(
            app_resource.schedule,
            ease_closure_system.in_set(TweenSystemSet::UpdateSetterValue),
        );
    }
}

/// Use a custom easing function via a closure.
///
/// See [`EaseFunction`].
#[derive(Component)]
pub struct EaseClosure(pub Box<dyn Fn(f32) -> f32 + Send + Sync + 'static>);

impl EaseClosure {
    /// Create new [`EaseClosure`]
    pub fn new<F: Fn(f32) -> f32 + Send + Sync + 'static>(f: F) -> EaseClosure {
        EaseClosure(Box::new(f))
    }
}

impl Default for EaseClosure {
    fn default() -> Self {
        EaseClosure::new(|v| v)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AToB<V, C> {
    pub a: V,
    pub b: V,
    pub curve: C,
}

#[allow(clippy::type_complexity)]
pub fn ease_closure_system(
    mut commands: Commands,
    query: Query<
        (Entity, &EaseClosure, &TimeSpanProgress),
        Or<(Changed<EaseClosure>, Changed<TimeSpanProgress>)>,
    >,
    mut removed: RemovedComponents<TimeSpanProgress>,
) {
    query.iter().for_each(|(entity, ease_closure, progress)| {
        if progress.now_percentage.is_nan() {
            return;
        }
        let value = ease_closure.0(progress.now_percentage.clamp(0., 1.));

        commands.entity(entity).insert(SetterValue(value));
    });
    removed.read().for_each(|entity| {
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<SetterValue>();
        }
    });
}
