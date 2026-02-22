use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_tween::{
    bevy_lookup_curve::{
        Knot, KnotInterpolation, LookupCurve, LookupCurvePlugin,
        editor::LookupCurveEditor,
    },
    combinator::tween,
    interpolate::translation,
    interpolation::bevy_lookup_curve::LookupCurveHandle,
    prelude::*,
    tween::{AnimationTarget, TargetComponent},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultTweenPluginsOnDefaultTime::default(),
            EguiPlugin::default(),
            LookupCurvePlugin,
        ))
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
    commands.spawn(Camera2d);
    let curve = curves.add(default_curve());
    let sprite = TargetComponent::marker();
    commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            AnimationTarget,
        ))
        .animation()
        .repeat(Repeat::Infinitely)
        .repeat_style(RepeatStyle::WrapAround)
        .insert(tween(
            Duration::from_secs(5),
            (
                LookupCurveHandle(curve.clone()),
                LookupCurveEditor::new(curve),
            ),
            sprite.with(translation(
                Vec3::new(-300., -300., 0.),
                Vec3::new(300., -300., 0.),
            )),
        ));
}
