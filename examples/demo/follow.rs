mod utils;

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_tween::{prelude::*, tween::TargetComponent, tweener::Tweener};

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
    TweenerCompleted,
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
struct JebTranslationTweener;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    use interpolate::scale;
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
            // Spawning the marker for a tweener that will be responsible
            // for the follow effect
            c.spawn((
                JebTranslationTweener,
                Name::new("JebTranslationTweener"),
            ));

            let jeb = TargetComponent::tweener_parent();
            // Spawning a tweener that's responsible for a rotating effect
            c.spawn((
                Name::new("Rotate"),
                TweenerBundle::new(Duration::from_secs(2))
                    .with_repeat(Repeat::Infinitely)
                    .with_repeat_style(RepeatStyle::PingPong)
                    .tween_here(),
                EaseFunction::CubicInOut,
                jeb.tween(interpolate::component_closure(
                    |transform: &mut Transform, value| {
                        let start = 0.;
                        let end = TAU;
                        let angle = (end - start).mul_add(value, start);
                        transform.rotation = Quat::from_rotation_z(angle);
                    },
                )),
            ));

            // Spawning a Tweener that's responsible for scaling effect
            // when you launch up the demo.
            c.spawn((
                Name::new("Scale up"),
                TweenerBundle::new(Duration::from_secs(1)).tween_here(),
                EaseFunction::QuinticIn,
                jeb.tween(scale(Vec3::ZERO, Vec3::ONE)),
            ));
        });
}

fn jeb_follows_cursor(
    mut commands: Commands,
    coord: Res<utils::MainCursorWorldCoord>,
    config: Res<Config>,
    q_jeb: Query<&Transform, With<Jeb>>,
    q_jeb_translation_tweener: Query<
        (Entity, Option<&Tweener>),
        With<JebTranslationTweener>,
    >,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    use interpolate::{sprite_color, translation};
    let jeb_transform = q_jeb.single();
    let (jeb_tweener_entity, jeb_tweener) = q_jeb_translation_tweener.single();
    let Some(coord) = coord.0 else {
        return;
    };
    let update = match config.update_kind {
        UpdateKind::CursorMoved => cursor_moved.read().next().is_some(),
        UpdateKind::CusorStopped => cursor_moved.read().next().is_none(),
        UpdateKind::TweenerCompleted => match jeb_tweener {
            Some(jeb_tweener) => {
                jeb_tweener.timer.is_completed()
                    && coord != jeb_transform.translation.xy()
            }
            None => true,
        },
    };
    if update {
        let jeb = TargetComponent::tweener_parent();
        commands.entity(jeb_tweener_entity).insert((
            TweenerBundle::new(config.tween_duration),
            TimeSpan::try_from(..config.tween_duration).unwrap(),
            config.tween_ease, // don't forget the ease
            // You can have multiple tween in the same Entity as long as their
            // type is differernt.
            //
            // This one for translation
            jeb.tween(translation(
                jeb_transform.translation,
                Vec3::new(coord.x, coord.y, 0.),
            )),
            // This one for color
            jeb.tween(sprite_color(Color::PINK, Color::WHITE)),
        ));
    }
}
