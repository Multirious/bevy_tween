use std::time::Duration;

use crate::prelude::*;
use bevy::{prelude::*, time::TimeUpdateStrategy};
use bevy_ecs::system::RunSystemOnce;
use bevy_tween_core::alters::consts::Translation;

fn default_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        bevy_tween_core::DefaultTweenCorePlugins,
        bevy_tween_core::AlterPlugin::<
            bevy_tween_core::alters::types::Translation,
        >::default(),
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
    use crate::timing::{forward, sequence};
    let mut app = default_test_app();
    let target = app.world_mut().spawn(Transform::IDENTITY);
    let target_id = target.id();
    app.world_mut()
        .run_system_once(move |mut commands: Commands| {
            let target = target_id.tween_via(Translation);
            commands.animation().insert(sequence((
                forward(secs(1)),
                target.ease(
                    Vec3::ZERO,
                    Vec3::splat(4.),
                    EaseFunction::Linear,
                    secs(4),
                ),
                forward(secs(2)),
                target.ease(
                    Vec3::splat(8.),
                    Vec3::splat(10.),
                    EaseFunction::Linear,
                    secs(2),
                ),
            )));
        })
        .unwrap();

    app.update();
    assert_eq!(elapsed(&app), secs(0));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(0.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(1));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(0.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(2));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(1.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(3));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(2.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(4));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(3.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(5));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(4.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(6));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(4.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(7));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(8.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(8));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(9.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(9));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(10.)
    );
    app.update();
    assert_eq!(elapsed(&app), secs(10));
    assert_eq!(
        component::<Transform>(&app, target_id).translation,
        Vec3::splat(10.)
    );
    app.update();
}
