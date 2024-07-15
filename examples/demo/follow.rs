mod utils;

use std::f32::consts::TAU;

use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    prelude::*,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_tween::{
    bevy_time_runner::TimeRunner, builder::parallel, items, prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
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
                sprite: Sprite {
                    color: into_color(DEEP_PINK),
                    ..Default::default()
                },
                ..Default::default()
            },
            Jeb,
        ))
        .with_children(|c| {
            // Spawning the marker for an animator that will be responsible
            // for the follow effect
            c.spawn(JebTranslationAnimator);

            let jeb = c.parent_entity().into_target();
            // Spawning an animator that's responsible for rotating effect
            c.animation()
                .repeat(Repeat::Infinitely)
                .repeat_style(RepeatStyle::PingPong)
                .insert_tween_here(jeb.set(items::AngleZ).tween(
                    0.,
                    TAU,
                    Duration::from_secs(2),
                    EaseFunction::CubicInOut,
                ));

            // Spawning an animator that's responsible for scaling effect
            // when you launch up the demo.
            c.animation().insert_tween_here(jeb.set(items::Scale).tween(
                Vec3::ZERO,
                Vec3::ONE,
                Duration::from_secs(1),
                EaseFunction::QuinticIn,
            ));
        });
}

fn jeb_follows_cursor(
    mut commands: Commands,
    coord: Res<utils::MainCursorWorldCoord>,
    config: Res<Config>,
    q_jeb: Query<(Entity, &Transform), With<Jeb>>,
    q_jeb_translation_animator: Query<
        (Entity, Option<&TimeRunner>),
        With<JebTranslationAnimator>,
    >,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_prev_stopped: Local<bool>,
) {
    let (jeb, jeb_transform) = q_jeb.single();
    let (jeb_translation_animator, jeb_time_runner) =
        q_jeb_translation_animator.single();
    let Some(coord) = coord.0 else {
        return;
    };
    let update = match config.update_kind {
        UpdateKind::CursorMoved => cursor_moved.read().last().is_some(),
        UpdateKind::CusorStopped => {
            let cursor_stopped = cursor_moved.read().last().is_none();
            let update = cursor_stopped && !*cursor_prev_stopped;
            *cursor_prev_stopped = cursor_stopped;
            update
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
        let jeb = jeb.into_target();
        commands
            .entity(jeb_translation_animator)
            .despawn_descendants() // clear previous animation
            .animation()
            .add(parallel((
                jeb.set(items::Translation).tween(
                    jeb_transform.translation,
                    Vec3::new(coord.x, coord.y, 0.),
                    config.tween_duration,
                    config.tween_ease,
                ),
                jeb.set(items::SpriteColor).tween(
                    into_color(WHITE),
                    into_color(DEEP_PINK),
                    config.tween_duration,
                    config.tween_ease,
                ),
            )));
    }
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}
