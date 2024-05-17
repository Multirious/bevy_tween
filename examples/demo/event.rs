use bevy::prelude::*;
use bevy_time_runner::{TimeRunnerEnded, TimeRunnerPlugin};
use bevy_tween::{
    combinator::*,
    prelude::*,
    tween::{TargetComponent, TweenerMarker},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TimeRunnerPlugin::default(),
            DefaultTweenPlugins,
        ))
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
    let mut triangle_translation = triangle.state(Vec3::new(start_x, 0., 0.));
    let mut triangle_angle_z = triangle.state(start_angle);

    commands
        .spawn((
            Triangle,
            SpriteBundle {
                texture: asset_server.load("triangle_filled.png"),
                ..Default::default()
            },
            TweenerMarker,
        ))
        .animation()
        .repeat(Repeat::Infinitely)
        .insert(|a, pos| {
            sequence((
                event(TweenEventData::with_data("bump")),
                tween(
                    secs(1.),
                    EaseFunction::ExponentialIn,
                    (
                        triangle_translation
                            .with(translation_to(Vec3::new(end_x, 0., 0.))),
                        triangle_angle_z.with(angle_z_to(mid_angle)),
                    ),
                ),
                backward(secs(0.2)),
                event_for(secs(0.2), TweenEventData::with_data("small_boom")),
                event(TweenEventData::with_data("boom")),
                tween(
                    secs(1.),
                    EaseFunction::CircularOut,
                    (
                        triangle_translation
                            .with(translation_to(Vec3::new(start_x, 0., 0.))),
                        triangle_angle_z.with(angle_z_to(end_angle)),
                    ),
                ),
            ))(a, pos)
        });
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
            commands
                .spawn((
                    Effect,
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(20., 100.)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(
                            effect_pos.trail,
                        ),
                        ..Default::default()
                    },
                    TweenerMarker,
                ))
                .animation()
                .insert_here(
                    secs(1.),
                    EaseFunction::QuinticOut,
                    (
                        entity.with(translation(
                            effect_pos.trail,
                            effect_pos.trail - Vec3::new(100., 0., 0.),
                        )),
                        entity.with(sprite_color(
                            Color::WHITE,
                            Color::PINK.with_a(0.),
                        )),
                    ),
                );
        }
        "small_boom" => {
            let entity = TargetComponent::TweenerEntity;
            commands
                .spawn((
                    Effect,
                    SpriteBundle {
                        texture: asset_server.load("circle.png"),
                        transform: Transform::from_translation(
                            q_triangle.single().translation,
                        ),
                        ..Default::default()
                    },
                    TweenerMarker,
                ))
                .animation()
                .insert_here(
                    secs(0.1),
                    EaseFunction::Linear,
                    (
                        entity.with(scale(
                            Vec3::new(0.5, 0.5, 0.),
                            Vec3::new(3., 3., 0.),
                        )),
                        entity.with(sprite_color(
                            Color::WHITE.with_a(0.5),
                            Color::PINK.with_a(0.),
                        )),
                    ),
                );
        }
        "boom" => {
            let entity = TargetComponent::TweenerEntity;
            commands
                .spawn((
                    Effect,
                    SpriteBundle {
                        texture: asset_server.load("circle.png"),
                        transform: Transform::from_translation(effect_pos.boom),
                        ..Default::default()
                    },
                    TweenerMarker,
                ))
                .animation()
                .insert_here(
                    secs(0.5),
                    EaseFunction::QuadraticOut,
                    (
                        entity.with(scale(
                            Vec3::new(1., 1., 0.),
                            Vec3::new(15., 15., 0.),
                        )),
                        entity.with(sprite_color(
                            Color::WHITE.with_a(1.),
                            Color::PINK.with_a(0.),
                        )),
                    ),
                );
        }
        _ => {}
    });
}

fn despawn_effect_system(
    mut commands: Commands,
    q_effect: Query<(), With<Effect>>,
    mut ended: EventReader<TimeRunnerEnded>,
) {
    ended.read().for_each(|ended| {
        if ended.is_completed() && q_effect.contains(ended.time_runner) {
            commands.entity(ended.time_runner).despawn_recursive();
        }
    });
}
