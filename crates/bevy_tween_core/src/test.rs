use std::time::Duration;

use crate::{
    alters::{self, Translation},
    argument, DefaultTweenCorePlugins,
};
use bevy::{prelude::*, time::TimeUpdateStrategy};
use bevy_time_runner::{TimeRunner, TimeSpan};

fn default_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        DefaultTweenCorePlugins,
        crate::AltererPlugin::<Translation>::default(),
    ));
    *app.world_mut().resource_mut::<TimeUpdateStrategy>() =
        TimeUpdateStrategy::ManualDuration(secs(1));
    app.world_mut()
        .resource_mut::<Time<Virtual>>()
        .set_max_delta(secs(1));
    app
}

fn secs(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

fn component<T: Component>(app: &App, id: Entity) -> &T {
    app.world().get::<T>(id).unwrap()
}

fn elapsed(app: &App) -> Duration {
    app.world().resource::<Time>().elapsed()
}

#[test]
fn simple() {
    let mut app = default_test_app();

    let target = app.world_mut().spawn(Transform::default());
    let target_id = target.id();
    let _animator = app
        .world_mut()
        .spawn(TimeRunner::new(secs(5)))
        .with_children(|c| {
            c.spawn((
                argument::Target(target_id),
                TimeSpan::try_from(secs(2)..secs(4)).unwrap(),
                argument::Alterer(alters::translation()),
                argument::Curve::new(EasingCurve::new(
                    Vec3::new(0., 0., 0.),
                    Vec3::new(2., 2., 2.),
                    EaseFunction::Linear,
                )),
            ));
        });

    app.update();
    assert_eq!(elapsed(&app), secs(0));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::new(0., 0., 0.),
    );

    app.update();
    assert_eq!(elapsed(&app), secs(1));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::new(0., 0., 0.),
    );

    app.update();
    assert_eq!(elapsed(&app), secs(2));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::new(0., 0., 0.),
    );

    app.update();
    assert_eq!(elapsed(&app), secs(3));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::new(1., 1., 1.),
    );

    app.update();
    assert_eq!(elapsed(&app), secs(4));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::new(2., 2., 2.),
    );

    app.update();
    assert_eq!(elapsed(&app), secs(5));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::new(2., 2., 2.),
    );

    app.update();
    assert_eq!(elapsed(&app), secs(6));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::new(2., 2., 2.),
    );
}
