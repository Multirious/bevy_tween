use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    prelude::*,
};
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded, builder::parallel, items, prelude::*,
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
    let circle_filled_image = asset_server.load("circle_filled.png");
    if let Some(coord) = coord.0 {
        if key.just_pressed(MouseButton::Left)
            || key.pressed(MouseButton::Right)
        {
            let cursor = Vec3::new(coord.x, coord.y, 1.);

            let transform = Transform::from_translation(cursor);
            let mut entity_commands = commands.spawn((SpriteBundle {
                texture: circle_filled_image,
                transform,
                ..Default::default()
            },));
            let circle = entity_commands.id().into_target();
            entity_commands.animation().add(parallel((
                circle.set(items::Translation).tween(
                    cursor,
                    Vec3::ZERO,
                    EaseFunction::ExponentialOut,
                    secs(2.),
                ),
                circle.set(items::Scale).tween(
                    Vec3::ONE,
                    Vec3::ZERO,
                    EaseFunction::BackIn,
                    secs(1.),
                ),
                circle.set(items::SpriteColor).tween(
                    into_color(WHITE),
                    into_color(DEEP_PINK),
                    EaseFunction::Linear,
                    secs(1.),
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
        commands.entity(t.time_runner).despawn_recursive();
    }
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}
