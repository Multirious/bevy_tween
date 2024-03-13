use std::time::Duration;

use bevy::prelude::*;
use bevy_tween::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let start = Vec3::new(-300., 0., 0.);
    let end = Vec3::new(300., 0., 0.);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    color: Color::WHITE,
                    ..Default::default()
                },
                transform: Transform::from_translation(start),
                ..Default::default()
            },
            SpanTweenerBundle::new(Duration::from_secs(5)),
        ))
        .with_children(|c| {
            c.build_tweens().tween(
                ..Duration::from_secs(5),
                EaseFunction::QuadraticInOut,
                ComponentTween::new(interpolate::Translation { start, end }),
            );
        });
}
