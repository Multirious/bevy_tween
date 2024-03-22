use bevy::prelude::*;
use bevy_tween::prelude::*;

use bevy_tween::{interpolate::W, splines::{Spline, Key, Interpolation}};

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
            c.spawn((
                SpanTweenBundle::new(..Duration::from_secs(5)),
                EaseSpline(Spline::from_vec(vec![
                    Key::new(0.   , 0.   , Interpolation::Linear),
                    Key::new(0.25 , 0.75 , Interpolation::Linear),
                    Key::new(0.26 , 0.   , Interpolation::Linear),
                    Key::new(0.27 , 1.   , Interpolation::Linear),
                    Key::new(0.28 , 0.   , Interpolation::Linear),
                    Key::new(0.5  , 1.   , Interpolation::Linear),
                    Key::new(0.75 , 0.   , Interpolation::Linear),
                    Key::new(1.   , 0.25 , Interpolation::Linear),
                ])),
                ComponentTween::new(interpolate::Translation { start, end }),
            ));
        });

    commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    color: Color::YELLOW,
                    ..Default::default()
                },
                transform: Transform::from_translation(start),
                ..Default::default()
            },
            SpanTweenerBundle::new(Duration::from_secs(5)),
    )).with_children(|c| {
        c.spawn((
            SpanTweenBundle::new(..Duration::from_secs(5)),
            EaseFunction::QuadraticOut,
            ComponentDynTween::new_boxed(interpolate::TranslationSpline(Spline::from_vec(vec![
                Key::new(0., W(start), Interpolation::Bezier(W(Vec3::new(-1000., 100., 0.)))),
                Key::new(0.5, W(Vec3::new(0., 200., 0.)), Interpolation::Linear),
                Key::new(1., W(end), Interpolation::Linear),
            ]))),
        ));
    });
}
