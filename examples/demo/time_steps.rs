use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_time_runner::TimeRunnerRegistrationPlugin;
use bevy_tween::{combinator::*, prelude::*, tween::AnimationTarget};
use std::time::Duration;

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_plugins(
            TimeRunnerRegistrationPlugin::<Fixed>::from_schedule_intern(
                FixedLast.intern(),
            ),
        )
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .add_systems(Startup, (setup, spawn_circle_with_tweens))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_circle_with_tweens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let circle_filled_image = asset_server.load("circle_filled.png");
    let float_duration = secs(4.0);
    let circle_transform = Transform::from_translation(Vec3::Y * -200.0);
    let circle = AnimationTarget.into_target();
    let mut circle_transform_state = circle.transform_state(circle_transform);

    let mut circle_commands = commands.spawn((
        Sprite {
            image: circle_filled_image,
            ..default()
        },
        circle_transform,
        AnimationTarget,
    ));

    circle_commands
        .animation_for_timestep::<Fixed>()
        .repeat(Repeat::Infinitely)
        .repeat_style(RepeatStyle::PingPong)
        .insert(tween(
            float_duration,
            EaseKind::CircularIn,
            circle_transform_state.translation_delta_by(Vec3::Y * 400.0),
        ));
}
