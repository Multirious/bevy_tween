use bevy::prelude::*;
use bevy_tween::{
    bevy_lookup_curve::{
        editor::LookupCurveEditor, Knot, KnotInterpolation, LookupCurve,
        LookupCurvePlugin,
    },
    combinator::tween,
    interpolate::translation,
    prelude::*,
    tween::TargetComponent,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins, LookupCurvePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn default_curve() -> LookupCurve {
    LookupCurve::new(vec![
        Knot {
            position: Vec2::ZERO,
            interpolation: KnotInterpolation::Cubic,
            ..Default::default()
        },
        Knot {
            position: Vec2::ONE,
            interpolation: KnotInterpolation::Linear,
            ..Default::default()
        },
    ])
}

fn setup(mut commands: Commands, mut curves: ResMut<Assets<LookupCurve>>) {
    commands.spawn(Camera2dBundle::default());
    let curve = curves.add(default_curve());
    let sprite = TargetComponent::marker();
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            AnimationTarget,
        ))
        .animation()
        .repeat(Repeat::Infinitely)
        .repeat_style(RepeatStyle::PingPong)
        .insert(tween(
            Duration::from_secs(1),
            (curve.clone(), LookupCurveEditor::new(curve)),
            sprite.with(translation(
                Vec3::new(-300., -300., 0.),
                Vec3::new(300., -300., 0.),
            )),
        ));
}
