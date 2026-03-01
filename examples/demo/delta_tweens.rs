use bevy::color::palettes::basic::WHITE;
use bevy::color::palettes::css::{BLUE, RED};
use bevy::prelude::*;
use bevy_tween::interpolate::sprite_color_delta_to;
use bevy_tween::{combinator::*, prelude::*, tween::AnimationTarget};
use std::time::Duration;

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins::default()))
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
    let circle_transform = Transform::from_xyz(400., -200., 0.);
    let vertical_delta = Vec3::new(0.0, 400.0, 0.0);
    let horizontal_delta = Vec3::new(-800.0, 0.0, 0.0);
    let float_duration = secs(4.0);
    let circle = AnimationTarget.into_target();
    let mut circle_transform_state = circle.transform_state(circle_transform);
    let mut circle_sprite_state = circle.state(WHITE.into());

    let mut circle_commands = commands.spawn((
        Sprite {
            image: circle_filled_image,
            ..default()
        },
        circle_transform,
        AnimationTarget,
    ));

    circle_commands
        .animation()
        .repeat(Repeat::Infinitely)
        .repeat_style(RepeatStyle::PingPong)
        .insert(parallel((
            tween(
                float_duration,
                EaseKind::BounceIn,
                circle_transform_state.translation_delta_by(horizontal_delta),
            ),
            tween(
                float_duration,
                EaseKind::SineInOut,
                circle_transform_state.translation_delta_by(vertical_delta),
            ),
            tween(
                float_duration,
                EaseKind::Linear,
                circle_sprite_state.with(sprite_color_delta_to(BLUE.into())),
            ),
            tween(
                float_duration,
                EaseKind::CubicIn,
                circle_sprite_state.with(sprite_color_delta_to(RED.into())),
            ),
        )));
}
