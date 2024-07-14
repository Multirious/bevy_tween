use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    prelude::*,
};
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded,
    builder::{backward, event, event_for, parallel, sequence},
    items,
    prelude::*,
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

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}

#[derive(Resource)]
struct EffectPos {
    trail: Vec3,
    boom: Vec3,
}

#[derive(Component)]
struct Triangle;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let x_left = -300.;
    let x_right = 300.;

    commands.insert_resource(EffectPos {
        trail: Vec3::new(x_left - 40., 0., 0.),
        boom: Vec3::new(x_right, 0., 0.),
    });

    let start_angle = -90.0_f32.to_radians();
    let mid_angle = start_angle + 540.0_f32.to_radians();
    let end_angle = mid_angle + 180.0_f32.to_radians();

    let mut entity_commands = commands.spawn((
        Triangle,
        SpriteBundle {
            texture: asset_server.load("triangle_filled.png"),
            ..Default::default()
        },
    ));

    let triangle = entity_commands.id().into_target();
    let mut triangle_translation = triangle
        .set_with(items::Translation)
        .with_state(Vec3::new(x_left, 0., 0.));
    let mut triangle_angle_z =
        triangle.set_with(items::AngleZ).with_state(start_angle);

    entity_commands
        .animation()
        .repeat(Repeat::Infinitely)
        .add(sequence((
            event("bump"),
            parallel((
                triangle_translation.tween_to(
                    Vec3::new(x_right, 0., 0.),
                    EaseFunction::ExponentialIn,
                    secs(1.),
                ),
                triangle_angle_z.tween_to(
                    mid_angle,
                    EaseFunction::ExponentialIn,
                    secs(1.),
                ),
            )),
            backward(secs(0.2)),
            event_for(secs(0.2), "small_boom"),
            event("boom"),
            parallel((
                triangle_translation.tween_to(
                    Vec3::new(x_left, 0., 0.),
                    EaseFunction::CircularOut,
                    secs(1.),
                ),
                triangle_angle_z.tween_to(
                    end_angle,
                    EaseFunction::CircularOut,
                    secs(1.),
                ),
            )),
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
    event.read().for_each(|event| match event.data {
        "bump" => {
            let mut entity_commands = commands.spawn((
                Effect,
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(20., 100.)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(effect_pos.trail),
                    ..Default::default()
                },
            ));
            let entity = entity_commands.id().into_target();
            let effect_translation = entity.set_with(items::Translation);
            let effect_color = entity.set_with(items::SpriteColor);
            entity_commands.animation().add(parallel((
                effect_translation.tween(
                    effect_pos.trail,
                    effect_pos.trail - Vec3::new(100., 0., 0.),
                    EaseFunction::QuinticOut,
                    secs(1.),
                ),
                effect_color.tween(
                    into_color(WHITE),
                    into_color(DEEP_PINK.with_alpha(0.)),
                    EaseFunction::QuinticOut,
                    secs(1.),
                ),
            )));
        }
        "small_boom" => {
            let mut entity_commands = commands.spawn((
                Effect,
                SpriteBundle {
                    texture: asset_server.load("circle.png"),
                    transform: Transform::from_translation(
                        q_triangle.single().translation,
                    ),
                    ..Default::default()
                },
            ));
            let entity = entity_commands.id().into_target();
            let effect_scale = entity.set_with(items::Scale);
            let effect_color = entity.set_with(items::SpriteColor);
            entity_commands.animation().add(parallel((
                effect_scale.tween(
                    Vec3::new(0.5, 0.5, 0.),
                    Vec3::new(3., 3., 0.),
                    EaseFunction::Linear,
                    secs(0.1),
                ),
                effect_color.tween(
                    into_color(WHITE.with_alpha(0.5)),
                    into_color(DEEP_PINK.with_alpha(0.)),
                    EaseFunction::Linear,
                    secs(0.1),
                ),
            )));
        }
        "boom" => {
            let mut entity_commands = commands.spawn((
                Effect,
                SpriteBundle {
                    texture: asset_server.load("circle.png"),
                    transform: Transform::from_translation(effect_pos.boom),
                    ..Default::default()
                },
            ));
            let entity = entity_commands.id().into_target();
            let effect_translation = entity.set_with(items::Scale);
            let effect_color = entity.set_with(items::SpriteColor);
            entity_commands.animation().add(parallel((
                effect_translation.tween(
                    Vec3::new(1., 1., 0.),
                    Vec3::new(15., 15., 0.),
                    EaseFunction::QuadraticOut,
                    secs(0.5),
                ),
                effect_color.tween(
                    into_color(WHITE.with_alpha(1.)),
                    into_color(DEEP_PINK.with_alpha(0.)),
                    EaseFunction::QuadraticOut,
                    secs(0.5),
                ),
            )));
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
