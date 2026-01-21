use bevy::prelude::*;
use bevy_tween::interpolate::{AngleZ, Translation, translation};
use bevy_tween::prelude::*;
use bevy_tween::{
    bevy_time_runner::{TimeRunner, TimeSpan},
    tween::{AnimationTarget, TargetComponent},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn sprite(start_x: f32, start_y: f32) -> (Sprite, Transform) {
    (
        Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(start_x, start_y, 0.),
    )
}

/// This show all the possible structure you can use.
/// All of these result in exactly the same animation!
/// Just use what fit for your use case.
///
/// These will be presented in its most rawest form.
/// See other examples for better APIs.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let angle_start = 0.;
    let angle_end = std::f32::consts::PI * 2.;

    let start_x = -300.;
    let end_x = 300.;

    let spacing_y = 100.;
    let offset_y = -(spacing_y * 3.) / 2.;

    // Everything in the same entity
    let y = 0. * spacing_y + offset_y;
    commands.spawn((
        sprite(start_x, y),
        AnimationTarget,
        TimeRunner::<()>::new(Duration::from_secs(5)),
        TimeSpan::try_from(..Duration::from_secs(5)).unwrap(),
        EaseKind::QuadraticInOut,
        ComponentTween::new_target(
            TargetComponent::marker(),
            translation(Vec3::new(start_x, y, 0.), Vec3::new(end_x, y, 0.)),
        ),
        ComponentTween::new_target(
            TargetComponent::marker(),
            AngleZ {
                start: angle_start,
                end: angle_end,
                delta: false,
            },
        ),
    ));
    // equivalent to
    //
    // let target = TargetComponent::marker();
    // commands.spawn((sprite(...), AnimationTarget))
    //     .animation()
    //     .insert_tween_here(
    //         Duration::from_secs(5),
    //         EaseKind::QuadraticOut,
    //         (
    //             target.with(translation(...)),
    //             target.with(angle_z(...)),
    //         ),
    //     );

    // Sprite and time runner as parent, tweens as children.
    let y = 1. * spacing_y + offset_y;
    commands
        .spawn((
            sprite(start_x, y),
            AnimationTarget,
            TimeRunner::<()>::new(Duration::from_secs(5)),
        ))
        .with_children(|c| {
            c.spawn((
                TimeSpan::try_from(..Duration::from_secs(5)).unwrap(),
                EaseKind::QuadraticInOut,
                ComponentTween::new_target(
                    TargetComponent::marker(),
                    translation(
                        Vec3::new(start_x, y, 0.),
                        Vec3::new(end_x, y, 0.),
                    ),
                ),
                ComponentTween::new_target(
                    TargetComponent::marker(),
                    AngleZ {
                        start: angle_start,
                        end: angle_end,
                        delta: false,
                    },
                ),
            ));
        });
    // equivalent to
    //
    // let target = TargetComponent::marker();
    // commands.spawn((sprite(...), AnimationTarget))
    //     .animation()
    //     .insert(tween(
    //         Duration::from_secs(5),
    //         EaseKind::QuadraticOut,
    //         (
    //             target.with(translation(...)),
    //             target.with(angle_z(...)),
    //         ),
    //     ));

    // Only Sprite as parent, time runner and tweens as children.
    let y = 2. * spacing_y + offset_y;
    commands
        .spawn((sprite(start_x, y), AnimationTarget))
        .with_children(|c| {
            c.spawn((
                TimeRunner::<()>::new(Duration::from_secs(5)),
                TimeSpan::try_from(..Duration::from_secs(5)).unwrap(),
                EaseKind::QuadraticInOut,
                ComponentTween::new_target(
                    TargetComponent::marker(),
                    translation(
                        Vec3::new(start_x, y, 0.),
                        Vec3::new(end_x, y, 0.),
                    ),
                ),
                ComponentTween::new_target(
                    TargetComponent::marker(),
                    AngleZ {
                        start: angle_start,
                        end: angle_end,
                        delta: false,
                    },
                ),
            ));
        });
    // equivalent to
    //
    // let target = TargetComponent::marker();
    // commands.spawn((sprite(...), AnimationTarget))
    //     .with_children(|c| {
    //         c.animation().insert_tween_here(
    //             Duration::from_secs(5),
    //             EaseKind::QuadraticOut,
    //             (
    //                 target.with(translation(...)),
    //                 target.with(angle_z(...))
    //             ),
    //         );
    //     });

    // Only Sprite as parent, tweens as children of a time runner.
    let y = 3. * spacing_y + offset_y;
    commands
        .spawn((sprite(start_x, y), AnimationTarget))
        .with_children(|c| {
            c.spawn(TimeRunner::<()>::new(Duration::from_secs(5)))
                .with_children(|c| {
                    c.spawn((
                        TimeSpan::try_from(..Duration::from_secs(5)).unwrap(),
                        EaseKind::QuadraticInOut,
                        ComponentTween::new_target(
                            TargetComponent::marker(),
                            Translation {
                                start: Vec3::new(start_x, y, 0.),
                                end: Vec3::new(end_x, y, 0.),
                                delta: false,
                            },
                        ),
                        ComponentTween::new_target(
                            TargetComponent::marker(),
                            AngleZ {
                                start: angle_start,
                                end: angle_end,
                                delta: false,
                            },
                        ),
                    ));
                });
        });
    // equivalent to
    //
    // let target = TargetComponent::marker();
    // commands.spawn((sprite(...), AnimationTarget))
    //     .with_children(|c| {
    //         c.animation().insert(tween(
    //             Duration::from_secs(5),
    //             EaseKind::QuadraticOut,
    //             (
    //                 target.with(translation(...)),
    //                 target.with(angle_z(...))
    //             ),
    //         ));
    //     });

    // or with this completely detached
    let y = 4. * spacing_y + offset_y;

    let sprite = commands.spawn(sprite(start_x, y)).id();

    commands
        .spawn(TimeRunner::<()>::new(Duration::from_secs(5)))
        .with_children(|c| {
            c.spawn((
                TimeSpan::try_from(..Duration::from_secs(5)).unwrap(),
                EaseKind::QuadraticInOut,
                ComponentTween::new_target(
                    sprite,
                    translation(
                        Vec3::new(start_x, y, 0.),
                        Vec3::new(end_x, y, 0.),
                    ),
                ),
                ComponentTween::new_target(
                    sprite,
                    AngleZ {
                        start: angle_start,
                        end: angle_end,
                        delta: false,
                    },
                ),
            ));
        });
    // equivalent to
    //
    // let target = TargetComponent::entity(sprite);
    // commands.animate().insert(tween(
    //     Duration::from_secs(5),
    //     EaseKind::QuadraticOut,
    //     (
    //         target.with(translation(...)),
    //         target.with(angle_z(...))
    //     ),
    // ));
}
