use std::marker::PhantomData;

use bevy_animation::animatable::Animatable;
use bevy_app::{Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ecs::schedule::{
    InternedScheduleLabel, IntoSystemConfigs, IntoSystemSetConfigs,
    ScheduleLabel, SystemSet,
};
use bevy_math::Curve;

use crate::alter::Alter;
use crate::systems;

#[derive(bevy_ecs::system::Resource, Clone)]
#[non_exhaustive]
pub struct TweenCoreAppResource {
    pub schedule: InternedScheduleLabel,
}

impl Default for TweenCoreAppResource {
    fn default() -> Self {
        TweenCoreAppResource {
            schedule: bevy_app::PostUpdate.intern(),
        }
    }
}

#[derive(Default)]
#[non_exhaustive]
pub struct TweenCorePlugin {
    pub app_resource: TweenCoreAppResource,
}
impl Plugin for TweenCorePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        if !app.is_plugin_added::<bevy_time_runner::TimeRunnerPlugin>() {
            app.add_plugins(bevy_time_runner::TimeRunnerPlugin {
                schedule: self.app_resource.schedule,
            });
        }
        app.insert_resource(self.app_resource.clone());
        app.configure_sets(
            self.app_resource.schedule,
            (
                TweenSystemSet::PrepareValues,
                TweenSystemSet::BlendValues,
                TweenSystemSet::ApplyValues,
            )
                .chain()
                .after(bevy_time_runner::TimeRunnerSet::Progress),
        );
    }
}

pub struct DefaultTweenCorePlugins;
impl PluginGroup for DefaultTweenCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        #[cfg(feature = "bevy_color")]
        use bevy_color::*;
        use bevy_math::*;

        let pg = PluginGroupBuilder::start::<DefaultTweenCorePlugins>()
            .add(TweenCorePlugin::default());

        #[cfg(feature = "debug")]
        let pg = pg.add(TweenDebugPlugin);

        type EasingCurvePlugin<V> =
            CurvePlugin<bevy_math::curve::EasingCurve<V>, V>;

        let pg = pg
            .add(EasingCurvePlugin::<f32>::default())
            .add(EasingCurvePlugin::<Vec2>::default())
            .add(EasingCurvePlugin::<Vec3>::default())
            .add(EasingCurvePlugin::<Vec3A>::default())
            .add(EasingCurvePlugin::<Quat>::default());

        #[cfg(feature = "bevy_color")]
        let pg = pg
            .add(EasingCurvePlugin::<Laba>::default())
            .add(EasingCurvePlugin::<LinearRgba>::default())
            .add(EasingCurvePlugin::<Oklaba>::default())
            .add(EasingCurvePlugin::<Srgba>::default())
            .add(EasingCurvePlugin::<Xyza>::default());

        pg
    }
}

pub struct AlterPlugin<A>(PhantomData<A>)
where
    A: Alter;
impl<A> Plugin for AlterPlugin<A>
where
    A: Alter,
{
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<crate::TweenBlend<A>>();
        let res = app
            .world()
            .get_resource::<TweenCoreAppResource>()
            .expect("TweenCoreAppResource exists");
        app.add_systems(
            res.schedule,
            (
                systems::update_blend_system::<A>
                    .in_set(TweenSystemSet::BlendValues),
                A::alter_system.in_set(TweenSystemSet::ApplyValues),
            ),
        );

        #[cfg(feature = "debug")]
        {
            use crate::debug::WillTweenList;
            if let Some(mut list) =
                app.world_mut().get_resource_mut::<WillTweenList>()
            {
                list.will_be_applied::<A>();
            }
        }
    }
}
impl<A> Default for AlterPlugin<A>
where
    A: Alter,
{
    fn default() -> Self {
        AlterPlugin(PhantomData)
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub enum TweenSystemSet {
    PrepareValues,
    BlendValues,
    ApplyValues,
}

pub struct CurvePlugin<C, V>
where
    C: Curve<V>,
{
    __marker: PhantomData<(C, V)>,
}

impl<C, V> Plugin for CurvePlugin<C, V>
where
    C: Curve<V> + Send + Sync + 'static,
    V: Animatable,
{
    fn build(&self, app: &mut bevy_app::App) {
        let res = app
            .world()
            .get_resource::<TweenCoreAppResource>()
            .expect("TweenCoreAppResource exists");
        app.add_systems(
            res.schedule,
            systems::progress_curve_system::<C, V>
                .in_set(TweenSystemSet::PrepareValues),
        );

        #[cfg(feature = "debug")]
        {
            use crate::debug::WillTweenList;
            if let Some(mut list) =
                app.world_mut().get_resource_mut::<WillTweenList>()
            {
                list.will_be_prepared::<C>();
            }
        }
    }
}

impl<C, V> Default for CurvePlugin<C, V>
where
    C: Curve<V>,
{
    fn default() -> Self {
        CurvePlugin {
            __marker: PhantomData,
        }
    }
}

#[cfg(feature = "debug")]
pub struct TweenDebugPlugin;

#[cfg(feature = "debug")]
impl Plugin for TweenDebugPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        use crate::debug::WillTweenList;
        app.init_resource::<WillTweenList>();
    }
}
