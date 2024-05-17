mod utils;

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_time_runner::{TimeRunner, TimeRunnerPlugin};
use bevy_tween::{
    combinator::InsertAnimationExt,
    interpolate::{scale, sprite_color, translation},
    prelude::*,
    tween::{TargetComponent, TweenerMarker},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TimeRunnerPlugin::default(),
            DefaultTweenPlugins,
            ResourceInspectorPlugin::<Config>::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (utils::main_cursor_world_coord_system, jeb_follows_cursor),
        )
        .init_resource::<Config>()
        .init_resource::<utils::MainCursorWorldCoord>()
        .register_type::<Config>()
        .run();
}

#[derive(Reflect)]
enum UpdateKind {
    CursorMoved,
    CusorStopped,
    AnimatorCompleted,
}

// Let us change the the tween ease and duration at runtime
#[derive(Resource, Reflect)]
struct Config {
    tween_duration: Duration,
    tween_ease: EaseFunction,
    update_kind: UpdateKind,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            update_kind: UpdateKind::CursorMoved,
            tween_duration: Duration::from_millis(500),
            tween_ease: EaseFunction::ExponentialOut,
        }
    }
}

/// Marker component for the square that will be following the cursor
#[derive(Component)]
struct Jeb;

/// Marker component for the tween entity we will be modifying to make the follow
/// effect
#[derive(Component)]
struct JebTranslationAnimator;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        utils::MainCamera,
    ));

    // Spawning the square
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("square_filled.png"),
                ..Default::default()
            },
            Jeb,
        ))
        .with_children(|c| {
            // Spawning the marker for an animator that will be responsible
            // for the follow effect
            c.spawn((JebTranslationAnimator, TweenerMarker));

            let jeb = TargetComponent::tweener_parent();
            // Spawning an animator that's responsible for a rotating effect
            c.spawn(TweenerMarker)
                .insert_animation()
                .repeat(Repeat::Infinitely)
                .repeat_style(RepeatStyle::PingPong)
                .animate_here(
                    Duration::from_secs(2),
                    EaseFunction::CubicInOut,
                    jeb.with_closure(|transform: &mut Transform, value| {
                        let start = 0.;
                        let end = TAU;
                        transform.rotation =
                            Quat::from_rotation_z(start.lerp(end, value));
                    }),
                );

            // Spawning a Tweener that's responsible for scaling effect
            // when you launch up the demo.
            c.spawn(TweenerMarker).insert_animation().animate_here(
                Duration::from_secs(1),
                EaseFunction::QuinticIn,
                jeb.with(scale(Vec3::ZERO, Vec3::ONE)),
            );
        });
}

fn jeb_follows_cursor(
    mut commands: Commands,
    coord: Res<utils::MainCursorWorldCoord>,
    config: Res<Config>,
    q_jeb: Query<&Transform, With<Jeb>>,
    q_jeb_translation_animator: Query<
        (Entity, Option<&TimeRunner>),
        With<JebTranslationAnimator>,
    >,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    let jeb_transform = q_jeb.single();
    let (jeb_animator_entity, jeb_time_runner) =
        q_jeb_translation_animator.single();
    let Some(coord) = coord.0 else {
        return;
    };
    let update = match config.update_kind {
        UpdateKind::CursorMoved => cursor_moved.read().next().is_some(),
        UpdateKind::CusorStopped => {
            let dx = (coord.x - jeb_transform.translation.x).abs();
            let dy = (coord.x - jeb_transform.translation.x).abs();
            let is_near_coord = dx < 0.05 && dy < 0.05;
            cursor_moved.read().next().is_none() && !is_near_coord
        }
        UpdateKind::AnimatorCompleted => match jeb_time_runner {
            Some(jeb_time_runner) => {
                jeb_time_runner.is_completed()
                    && coord != jeb_transform.translation.xy()
            }
            None => true,
        },
    };
    if update {
        let jeb = TargetComponent::tweener_parent();
        commands
            .entity(jeb_animator_entity)
            .insert_animation()
            .animate_here(
                config.tween_duration,
                config.tween_ease,
                (
                    jeb.with(translation(
                        jeb_transform.translation,
                        Vec3::new(coord.x, coord.y, 0.),
                    )),
                    jeb.with(sprite_color(Color::WHITE, Color::PINK)),
                ),
            );
    }
}
