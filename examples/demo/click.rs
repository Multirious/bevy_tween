use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    prelude::*,
};
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded, combinator::*, prelude::*,
    tween::AnimationTarget,
};
mod utils;

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
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
    commands.spawn((Camera2d, utils::MainCamera));
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
            let circle = AnimationTarget.into_target();
            let mut circle_transform = circle.transform_state(transform);
            commands
                .spawn((
                    Sprite {
                        image: circle_filled_image,
                        ..default()
                    },
                    transform,
                    AnimationTarget,
                ))
                .animation()
                .insert(parallel((
                    tween(
                        secs(2.),
                        EaseKind::ExponentialOut,
                        circle_transform.translation_to(end),
                    ),
                    tween(
                        secs(1.),
                        EaseKind::BackIn,
                        circle_transform.scale_to(Vec3::ZERO),
                    ),
                    tween(
                        secs(1.),
                        EaseKind::Linear,
                        circle.with(sprite_color(
                            into_color(WHITE),
                            into_color(DEEP_PINK),
                        )),
                    ),
                )));
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

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}
