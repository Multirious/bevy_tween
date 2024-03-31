use bevy::prelude::*;
use bevy_tween::prelude::*;
// This import isn't needed if you're using shortcuts.
use bevy_tween::tween::TargetComponent;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn sprite(start_x: f32, start_y: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            color: Color::WHITE,
            ..Default::default()
        },
        transform: Transform::from_xyz(start_x, start_y, 0.),
        ..Default::default()
    }
}

/// This show all the possible structure you can use.
/// All of these result in exactly the same animation!
/// Just use what fit for your use case.
///
/// These will be presented using the more barebone APIs for clarity.
/// You might want to use shortcuts under "----- or -----" comment.
/// `ComponentTween`, `ResourceTween`, and `AssetTween` is all type alias
/// of `Tween`
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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
        SpanTweenerBundle::new(Duration::from_secs(5)),
        SpanTweenBundle::new(..Duration::from_secs(5)),
        EaseFunction::QuadraticInOut,
        ComponentTween::new_target(
            TargetComponent::tweener_entity(),
            interpolate::Translation {
                start: Vec3::new(start_x, y, 0.),
                end: Vec3::new(end_x, y, 0.),
            },
        ),
        ComponentTween::new_target(
            TargetComponent::tweener_entity(),
            interpolate::AngleZ {
                start: angle_start,
                end: angle_end,
            },
        ),
        // ----- or -----
        // ComponentTween::tweener_entity( ... ),
        // ----- or -----
        // ComponentTween::new( ... ),
    ));

    // Sprite and tweener as parent, tweens as children.
    let y = 1. * spacing_y + offset_y;
    commands
        .spawn((
            sprite(start_x, y),
            SpanTweenerBundle::new(Duration::from_secs(5)),
        ))
        .with_children(|c| {
            c.spawn((
                SpanTweenBundle::new(..Duration::from_secs(5)),
                EaseFunction::QuadraticInOut,
                ComponentTween::new_target(
                    TargetComponent::tweener_entity(),
                    interpolate::Translation {
                        start: Vec3::new(start_x, y, 0.),
                        end: Vec3::new(end_x, y, 0.),
                    },
                ),
                ComponentTween::new_target(
                    TargetComponent::tweener_entity(),
                    interpolate::AngleZ {
                        start: angle_start,
                        end: angle_end,
                    },
                ),
                // ----- or -----
                // ComponentTween::tweener_entity( ... ),
                // ----- or -----
                // ComponentTween::new( ... ),
            ));
        });

    // Only Sprite as parent, tweener and tweens as children.
    let y = 2. * spacing_y + offset_y;
    commands.spawn(sprite(start_x, y)).with_children(|c| {
        c.spawn((
            SpanTweenerBundle::new(Duration::from_secs(5)),
            SpanTweenBundle::new(..Duration::from_secs(5)),
            EaseFunction::QuadraticInOut,
            ComponentTween::new_target(
                TargetComponent::tweener_parent(),
                interpolate::Translation {
                    start: Vec3::new(start_x, y, 0.),
                    end: Vec3::new(end_x, y, 0.),
                },
            ),
            ComponentTween::new_target(
                TargetComponent::tweener_parent(),
                interpolate::AngleZ {
                    start: angle_start,
                    end: angle_end,
                },
            ),
            // ----- or -----
            // ComponentTween::tweener_parent( ... ),
        ));
    });

    // Only Sprite as parent, tweens as children of a tweener.
    let y = 3. * spacing_y + offset_y;
    commands.spawn(sprite(start_x, y)).with_children(|c| {
        c.spawn(SpanTweenerBundle::new(Duration::from_secs(5)))
            .with_children(|c| {
                c.spawn((
                    SpanTweenBundle::new(..Duration::from_secs(5)),
                    EaseFunction::QuadraticInOut,
                    ComponentTween::new_target(
                        TargetComponent::tweener_parent(),
                        interpolate::Translation {
                            start: Vec3::new(start_x, y, 0.),
                            end: Vec3::new(end_x, y, 0.),
                        },
                    ),
                    ComponentTween::new_target(
                        TargetComponent::tweener_parent(),
                        interpolate::AngleZ {
                            start: angle_start,
                            end: angle_end,
                        },
                    ),
                    // ----- or -----
                    // ComponentTween::tweener_parent( ... ),
                ));
            });
    });

    // or with this completely detached
    let y = 4. * spacing_y + offset_y;

    let sprite = commands.spawn(sprite(start_x, y)).id();

    commands
        .spawn(SpanTweenerBundle::new(Duration::from_secs(5)))
        .with_children(|c| {
            c.spawn((
                SpanTweenBundle::new(..Duration::from_secs(5)),
                EaseFunction::QuadraticInOut,
                ComponentTween::new_target(
                    sprite,
                    interpolate::Translation {
                        start: Vec3::new(start_x, y, 0.),
                        end: Vec3::new(end_x, y, 0.),
                    },
                ),
                ComponentTween::new_target(
                    sprite,
                    interpolate::AngleZ {
                        start: angle_start,
                        end: angle_end,
                    },
                ),
            ));
        });
}
