use bevy::prelude::*;
use bevy_tween::prelude::*;
mod utils;

mod my_interpolate {
    use std::sync::OnceLock;

    use bevy::prelude::*;
    use bevy_tween::prelude::*;

    pub struct JustTranslateTo {
        pub start: OnceLock<Vec3>,
        pub end: Vec3,
    }

    impl JustTranslateTo {
        pub fn end(end: Vec3) -> JustTranslateTo {
            JustTranslateTo {
                start: OnceLock::new(),
                end,
            }
        }
    }

    impl Interpolator for JustTranslateTo {
        type Item = Transform;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let start = self.start.get_or_init(|| item.translation);
            let end = self.end;
            item.translation = start.lerp(end, value);
        }
    }

    pub struct JustScaleTo {
        pub start: OnceLock<Vec3>,
        pub end: Vec3,
    }

    impl JustScaleTo {
        pub fn end(end: Vec3) -> JustScaleTo {
            JustScaleTo {
                start: OnceLock::new(),
                end,
            }
        }
    }

    impl Interpolator for JustScaleTo {
        type Item = Transform;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let start = self.start.get_or_init(|| item.scale);
            let end = self.end;
            item.scale = start.lerp(end, value);
        }
    }
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

#[derive(Component)]
struct CircleCountText;

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
    let spawn =
        key.just_pressed(MouseButton::Left) || key.pressed(MouseButton::Right);
    if let Some(coord) = coord.0 {
        if spawn {
            let start = Vec3::new(coord.x, coord.y, 0.);
            let end = Vec3::new(0., 0., 0.);
            commands
                .spawn((
                    SpriteBundle {
                        texture: circle_filled_image,
                        transform: Transform::from_translation(start),
                        ..Default::default()
                    },
                    SpanTweenerBundle::new(Duration::from_secs(2)),
                ))
                .with_children(|c| {
                    c.child_tweens()
                        .tween(
                            ..Duration::from_secs(2),
                            EaseFunction::ExponentialOut,
                            ComponentDynTween::tweener_entity(Box::new(
                                my_interpolate::JustTranslateTo::end(end),
                            )),
                        )
                        .tween(
                            ..Duration::from_secs(1),
                            EaseFunction::BackIn,
                            ComponentDynTween::tweener_entity(Box::new(
                                my_interpolate::JustScaleTo::end(Vec3::ZERO),
                            )),
                        );
                });
        }
    }
}

fn despawn_finished_circle(
    mut commands: Commands,
    mut tweener_ended_reader: EventReader<SpanTweenerEnded>,
) {
    for t in tweener_ended_reader.read() {
        commands.entity(t.tweener).despawn();
    }
}

// fn display_circle_count(
//     q_circle: Query<(), With<Sprite>>,
// ) {

// }
