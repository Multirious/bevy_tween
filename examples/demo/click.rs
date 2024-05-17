use bevy::prelude::*;
use bevy_time_runner::TimeRunnerEnded;
use bevy_tween::bevy_time_runner::TimeRunnerPlugin;
use bevy_tween::tween::TweenerMarker;
use bevy_tween::{
    combinator::*, interpolate::transform_tween_state, prelude::*,
    tween::TargetComponent,
};
mod utils;

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TimeRunnerPlugin::default(),
            DefaultTweenPlugins,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                utils::main_cursor_world_coord_system,
                click_spawn_circle,
                despawn_finished_circle,
            ),
        )
        .init_resource::<utils::MainCursorWorldCoord>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        utils::MainCamera,
    ));
}

fn click_spawn_circle(
    mut commands: Commands,
    coord: Res<utils::MainCursorWorldCoord>,
    key: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    use interpolate::sprite_color;
    let circle_filled_image = asset_server.load("circle_filled.png");
    if let Some(coord) = coord.0 {
        if key.just_pressed(MouseButton::Left)
            || key.pressed(MouseButton::Right)
        {
            let start = Vec3::new(coord.x, coord.y, 1.);
            let end = Vec3::new(0., 0., 0.);
            let transform = Transform::from_translation(start);
            let mut circle_transform = transform_tween_state(
                TargetComponent::TweenerEntity,
                transform,
            );
            let circle = TargetComponent::tweener_entity();
            commands
                .spawn((
                    SpriteBundle {
                        texture: circle_filled_image,
                        transform,
                        ..Default::default()
                    },
                    TweenerMarker,
                ))
                .animation()
                .insert(|a, pos| {
                    parallel((
                        tween(
                            secs(2.),
                            EaseFunction::ExponentialOut,
                            circle_transform.translation_to(end),
                        ),
                        tween(
                            secs(1.),
                            EaseFunction::BackIn,
                            circle_transform.scale_to(Vec3::ZERO),
                        ),
                        tween(
                            secs(1.),
                            EaseFunction::Linear,
                            circle
                                .with(sprite_color(Color::WHITE, Color::PINK)),
                        ),
                    ))(a, pos)
                });
        }
    }
}

fn despawn_finished_circle(
    mut commands: Commands,
    mut time_runner_ended_reader: EventReader<TimeRunnerEnded>,
) {
    for t in time_runner_ended_reader.read() {
        commands.entity(t.time_runner).despawn();
    }
}
