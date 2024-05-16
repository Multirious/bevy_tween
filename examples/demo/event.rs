use bevy::prelude::*;
use bevy_tween::{
    interpolate::tween_state, prelude::*, tween::TargetComponent,
    tweener::combinator::*,
};

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

#[derive(Component)]
struct Triangle;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    use interpolate::{angle_z_to, translation_to};
    commands.spawn(Camera2dBundle::default());

    let start_x = -300.;
    let end_x = 300.;

    commands.insert_resource(EffectPos {
        trail: Vec3::new(start_x - 40., 0., 0.),
        boom: Vec3::new(end_x, 0., 0.),
    });

    let start_angle = -90.0_f32.to_radians();
    let mid_angle = start_angle + 540.0_f32.to_radians();
    let end_angle = mid_angle + 180.0_f32.to_radians();

    let triangle = TargetComponent::tweener_entity();
    let mut triangle_translation =
        tween_state(triangle.clone(), Vec3::new(start_x, 0., 0.));
    let mut triangle_angle_z = tween_state(triangle, start_angle);

    commands
        .spawn((
            Triangle,
            SpriteBundle {
                texture: asset_server.load("triangle_filled.png"),
                ..Default::default()
            },
            SpanTweenerBundle::new(secs(2.)).with_repeat(Repeat::Infinitely),
        ))
        .tweens()
        .add(sequence((
            tween_event(TweenEventData::with_data("bump")),
            tween(
                secs(1.),
                EaseFunction::ExponentialIn,
                (
                    triangle_translation
                        .tween(translation_to(Vec3::new(end_x, 0., 0.))),
                    triangle_angle_z.tween(angle_z_to(mid_angle)),
                ),
            ),
            backward(secs(0.2)),
            tween_event_for(secs(0.2), TweenEventData::with_data("small_boom")),
            tween_event(TweenEventData::with_data("boom")),
            tween(
                secs(1.),
                EaseFunction::CircularOut,
                (
                    triangle_translation
                        .tween(translation_to(Vec3::new(start_x, 0., 0.))),
                    triangle_angle_z.tween(angle_z_to(end_angle)),
                ),
            ),
        )));
}

#[derive(Component)]
struct Effect;

fn effect_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    effect_pos: Res<EffectPos>,
    q_triangle: Query<&Transform, With<Triangle>>,
    mut event: EventReader<TweenEvent<&'static str>>,
) {
    use interpolate::{scale, sprite_color, translation};
    event.read().for_each(|event| match event.data {
        "bump" => {
            let entity = TargetComponent::TweenerEntity;
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
                entity.with(translation(
                    effect_pos.trail,
                    effect_pos.trail - Vec3::new(100., 0., 0.),
                )),
                entity.with(sprite_color(Color::WHITE, Color::PINK.with_a(0.))),
            ));
        }
        "small_boom" => {
            let entity = TargetComponent::TweenerEntity;
            commands.spawn((
                Effect,
                SpriteBundle {
                    texture: asset_server.load("circle.png"),
                    transform: Transform::from_translation(
                        q_triangle.single().translation,
                    ),
                    ..Default::default()
                },
                SpanTweenerBundle::new(secs(0.1)).tween_here(),
                EaseFunction::Linear,
                entity.with(scale(
                    Vec3::new(0.5, 0.5, 0.),
                    Vec3::new(3., 3., 0.),
                )),
                entity.with(sprite_color(
                    Color::WHITE.with_a(0.5),
                    Color::PINK.with_a(0.),
                )),
            ));
        }
        "boom" => {
            let entity = TargetComponent::TweenerEntity;
            commands.spawn((
                Effect,
                SpriteBundle {
                    texture: asset_server.load("circle.png"),
                    transform: Transform::from_translation(effect_pos.boom),
                    ..Default::default()
                },
                SpanTweenerBundle::new(secs(0.5)).tween_here(),
                EaseFunction::QuadraticOut,
                entity.with(scale(
                    Vec3::new(1., 1., 0.),
                    Vec3::new(15., 15., 0.),
                )),
                entity.with(sprite_color(
                    Color::WHITE.with_a(1.),
                    Color::PINK.with_a(0.),
                )),
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
