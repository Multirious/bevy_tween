use std::marker::PhantomData;

use bevy_animation::animatable::Animatable;
use bevy_app::{Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ecs::{
    component::Component,
    schedule::{
        InternedScheduleLabel, IntoSystemConfigs, IntoSystemSetConfigs,
        ScheduleLabel, SystemSet,
    },
    system::Resource,
};
use bevy_math::Curve;

use crate::alter::{Alter, AlterSingle};
use crate::systems;

#[cfg(feature = "bevy_asset")]
use bevy_asset::Asset;

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

pub struct AltererPlugin<A>(PhantomData<A>)
where
    A: Alter;
impl<A> Plugin for AltererPlugin<A>
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
                systems::alterer_system::<A>
                    .in_set(TweenSystemSet::ApplyValues),
            ),
        );
    }
}
impl<A> Default for AltererPlugin<A>
where
    A: Alter,
{
    fn default() -> Self {
        AltererPlugin(PhantomData)
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub enum TweenSystemSet {
    PrepareValues,
    BlendValues,
    ApplyValues,
}

pub fn component_plugin<A>(app: &mut bevy_app::App)
where
    A: AlterSingle,
    A::Item: Component,
{
    use crate::alter::AlterComponent;
    app.add_plugins(AltererPlugin::<AlterComponent<A>>::default());
}

pub fn resource_plugin<A>(app: &mut bevy_app::App)
where
    A: AlterSingle,
    A::Item: Resource,
{
    use crate::alter::AlterResource;
    app.add_plugins(AltererPlugin::<AlterResource<A>>::default());
}

#[cfg(feature = "bevy_asset")]
pub fn asset_plugin<A>(app: &mut bevy_app::App)
where
    A: AlterSingle,
    A::Item: Asset,
{
    use crate::alter::AlterAsset;
    app.add_plugins(AltererPlugin::<AlterAsset<A>>::default());
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
