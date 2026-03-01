use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    prelude::*,
};
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded, combinator::*, prelude::*,
    tween::AnimationTarget,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins::default()))
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
    use interpolate::{angle_z_to, translation_to};
    commands.spawn(Camera2d);

    let x_left = -300.;
    let x_right = 300.;

    commands.insert_resource(EffectPos {
        trail: Vec3::new(x_left - 40., 0., 0.),
        boom: Vec3::new(x_right, 0., 0.),
    });

    let start_angle = -90.0_f32.to_radians();
    let mid_angle = start_angle + 540.0_f32.to_radians();
    let end_angle = mid_angle + 180.0_f32.to_radians();

    let triangle = AnimationTarget.into_target();
    let mut triangle_translation = triangle.state(Vec3::new(x_left, 0., 0.));

    let mut triangle_angle_z = triangle.state(start_angle);

    commands
        .spawn((
            Triangle,
            Sprite {
                image: asset_server.load("triangle_filled.png"),
                ..default()
            },
            AnimationTarget,
        ))
        .animation()
        .repeat(Repeat::Infinitely)
        .insert(sequence((
            event("bump"),
            tween(
                secs(1.),
                EaseKind::ExponentialIn,
                (
                    triangle_translation
                        .with(translation_to(Vec3::new(x_right, 0., 0.))),
                    triangle_angle_z.with(angle_z_to(mid_angle)),
                ),
            ),
            backward(secs(0.2)),
            event_for(secs(0.2), "small_boom"),
            event("boom"),
            tween(
                secs(1.),
                EaseKind::CircularOut,
                (
                    triangle_translation
                        .with(translation_to(Vec3::new(x_left, 0., 0.))),
                    triangle_angle_z.with(angle_z_to(end_angle)),
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
    mut event: MessageReader<TweenEvent<&'static str>>,
) {
    use interpolate::{scale, sprite_color, translation};
    event.read().for_each(|event| match event.data {
        "bump" => {
            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Effect,
                    Sprite {
                        custom_size: Some(Vec2::new(20., 100.)),
                        ..Default::default()
                    },
                    Transform::from_translation(effect_pos.trail),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    secs(1.),
                    EaseKind::QuinticOut,
                    (
                        entity.with(translation(
                            effect_pos.trail,
                            effect_pos.trail - Vec3::new(100., 0., 0.),
                        )),
                        entity.with(sprite_color(
                            into_color(WHITE),
                            into_color(DEEP_PINK.with_alpha(0.)),
                        )),
                    ),
                );
        }
        "small_boom" => {
            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Effect,
                    Sprite {
                        image: asset_server.load("circle.png"),
                        ..default()
                    },
                    Transform::from_translation(
                        q_triangle.single().unwrap().translation,
                    ),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    secs(0.1),
                    EaseKind::Linear,
                    (
                        entity.with(scale(
                            Vec3::new(0.5, 0.5, 0.),
                            Vec3::new(3., 3., 0.),
                        )),
                        entity.with(sprite_color(
                            into_color(WHITE.with_alpha(0.5)),
                            into_color(DEEP_PINK.with_alpha(0.)),
                        )),
                    ),
                );
        }
        "boom" => {
            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Effect,
                    Sprite {
                        image: asset_server.load("circle.png"),
                        ..default()
                    },
                    Transform::from_translation(effect_pos.boom),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    secs(0.5),
                    EaseKind::QuadraticOut,
                    (
                        entity.with(scale(
                            Vec3::new(1., 1., 0.),
                            Vec3::new(15., 15., 0.),
                        )),
                        entity.with(sprite_color(
                            into_color(WHITE.with_alpha(1.)),
                            into_color(DEEP_PINK.with_alpha(0.)),
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
    mut ended: MessageReader<TimeRunnerEnded>,
) {
    ended.read().for_each(|ended| {
        if ended.is_completed() && q_effect.contains(ended.entity) {
            commands.entity(ended.entity).despawn();
        }
    });
}
