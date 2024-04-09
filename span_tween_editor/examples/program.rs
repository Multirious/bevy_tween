use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_tween::prelude::*;
use span_tween_editor::SpanTweenEditorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultTweenPlugins,
            EguiPlugin,
            SpanTweenEditorPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let sprite = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20., 20.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    commands
        .spawn((SpanTweenerBundle::new(secs(4.)), Name::new("MyTweener")))
        .with_children(|c| {
            c.span_tweens()
                .tween(
                    secs(1.),
                    EaseFunction::QuadraticOut,
                    ComponentTween::new_target(
                        sprite,
                        interpolate::Translation {
                            start: Vec3::new(-200., -200., 0.),
                            end: Vec3::new(200., -200., 0.),
                        },
                    ),
                )
                .tween(
                    secs(1.),
                    EaseFunction::QuadraticOut,
                    ComponentTween::new_target(
                        sprite,
                        interpolate::Translation {
                            start: Vec3::new(200., -200., 0.),
                            end: Vec3::new(200., 200., 0.),
                        },
                    ),
                )
                .tween(
                    secs(1.),
                    EaseFunction::QuadraticOut,
                    ComponentTween::new_target(
                        sprite,
                        interpolate::Translation {
                            start: Vec3::new(200., 200., 0.),
                            end: Vec3::new(-200., 200., 0.),
                        },
                    ),
                )
                .tween(
                    secs(1.),
                    EaseFunction::QuadraticOut,
                    ComponentTween::new_target(
                        sprite,
                        interpolate::Translation {
                            start: Vec3::new(-200., 200., 0.),
                            end: Vec3::new(-200., -200., 0.),
                        },
                    ),
                );
        });
}
