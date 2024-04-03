use bevy::prelude::*;
use bevy_tween::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (effect_system, despawn_effect_system))
        .run();
}

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

#[derive(Resource)]
struct EffectPos {
    trail: Vec3,
    boom: Vec3,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let start_x = -200.;
    let end_x = 200.;

    commands.insert_resource(EffectPos {
        trail: Vec3::new(start_x - 40., 0., 0.),
        boom: Vec3::new(end_x, 0., 0.),
    });

    let start_angle = -90.0_f32.to_radians();
    let mid_angle = start_angle + 540.0_f32.to_radians();
    let end_angle = mid_angle + 180.0_f32.to_radians();

    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("triangle_filled.png"),
                ..Default::default()
            },
            SpanTweenerBundle::new(secs(2.)).with_repeat(Repeat::Infinitely),
        ))
        .with_children(|c| {
            // &'static str is available as an even data but recommended to use
            // dedicated custom type instead to leverage the rust type system.
            c.span_tweens()
                .tween_event(TweenEventData::with_data("trail"))
                .tween(
                    secs(1.),
                    EaseFunction::ExponentialIn,
                    (
                        ComponentTween::new(interpolate::Translation {
                            start: Vec3::new(start_x, 0., 0.),
                            end: Vec3::new(end_x, 0., 0.),
                        }),
                        ComponentTween::new(interpolate::AngleZ {
                            start: start_angle,
                            end: mid_angle,
                        }),
                    ),
                )
                .tween_event(TweenEventData::with_data("boom"))
                .tween(
                    secs(1.),
                    EaseFunction::ExponentialOut,
                    (
                        ComponentTween::new(interpolate::Translation {
                            start: Vec3::new(end_x, 0., 0.),
                            end: Vec3::new(start_x, 0., 0.),
                        }),
                        ComponentTween::new(interpolate::AngleZ {
                            start: mid_angle,
                            end: end_angle,
                        }),
                    ),
                );
        });
}

#[derive(Component)]
struct Effect;

fn effect_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    effect_pos: Res<EffectPos>,
    mut event: EventReader<TweenEvent<&'static str>>,
) {
    event.read().for_each(|event| match event.data {
        "trail" => {
            commands.spawn((
                Effect,
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(20., 100.)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(effect_pos.trail),
                    ..Default::default()
                },
                SpanTweenerBundle::new(secs(1.)).tween_here(),
                EaseFunction::QuinticOut,
                ComponentTween::new(interpolate::Translation {
                    start: effect_pos.trail,
                    end: effect_pos.trail - Vec3::new(100., 0., 0.),
                }),
                ComponentTween::new(interpolate::SpriteColor {
                    start: Color::WHITE,
                    end: Color::WHITE.with_a(0.),
                }),
            ));
        }
        "boom" => {
            commands.spawn((
                Effect,
                SpriteBundle {
                    texture: asset_server.load("circle.png"),
                    transform: Transform::from_translation(effect_pos.boom),
                    ..Default::default()
                },
                SpanTweenerBundle::new(secs(0.5)).tween_here(),
                EaseFunction::ExponentialOut,
                ComponentTween::new(interpolate::Scale {
                    start: Vec3::new(1., 1., 0.),
                    end: Vec3::new(15., 15., 0.),
                }),
                ComponentTween::new(interpolate::SpriteColor {
                    start: Color::WHITE,
                    end: Color::WHITE.with_a(0.),
                }),
            ));
        }
        _ => {}
    });
}

fn despawn_effect_system(
    mut commands: Commands,
    q_effect: Query<(), With<Effect>>,
    mut ended: EventReader<SpanTweenerEnded>,
) {
    ended.read().for_each(|ended| {
        if ended.is_completed() && q_effect.contains(ended.tweener) {
            commands.entity(ended.tweener).despawn_recursive();
        }
    });
}
