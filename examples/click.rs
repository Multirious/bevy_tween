use bevy::prelude::*;
use bevy_tween::prelude::*;
mod utils;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (utils::main_cursor_world_coord_system, click_spawn_square),
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

fn click_spawn_square(
    mut commands: Commands,
    coord: Res<utils::MainCursorWorldCoord>,
    key: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(coord) = coord.0 {
        if key.just_pressed(MouseButton::Left) {
            // let start = Vec3::new(coord.x, coord.y, 0.);
            // let end = Vec3::new(coord.x, coord.y - 500., 0.);
            // commands.spawn(
            //     SpriteBundle {
            //         texture: asset_server.load("circle.png"),
            //         transform: Transform::from_translation(start),
            //         ..Default::default()
            //     }
            //     .tween(
            //         SpanTweenPlayer,
            //         lenses::TransformTranslationLens { start, end },
            //         Duration::from_secs(5),
            //         EaseFunction::ExponentialIn,
            //     ),
            // );
        }
    }
}
