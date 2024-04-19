use bevy::prelude::*;
use bevy_tween::{prelude::*, tweener::combinator::*};
mod utils;

mod interpolate {
    use std::sync::OnceLock;

    use bevy::prelude::*;
    pub use bevy_tween::interpolate::*;
    use bevy_tween::{prelude::*, tween::ComponentTween};

    pub fn translate_to(end: Vec3) -> ComponentTween<TranslateTo> {
        ComponentTween::new(TranslateTo::end(end))
    }

    /// Automatically figure out the current position of this entity and then
    /// tween from there.
    pub struct TranslateTo {
        pub start: OnceLock<Vec3>,
        pub end: Vec3,
    }

    impl TranslateTo {
        pub fn end(end: Vec3) -> TranslateTo {
            TranslateTo {
                start: OnceLock::new(),
                end,
            }
        }
    }

    impl Interpolator for TranslateTo {
        type Item = Transform;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let start = self.start.get_or_init(|| item.translation);
            let end = self.end;
            item.translation = start.lerp(end, value);
        }
    }

    pub fn scale_to(end: Vec3) -> ComponentTween<ScaleTo> {
        ComponentTween::new(ScaleTo::end(end))
    }

    /// Automatically figure out the current scale of this entity and then
    /// tween from there.
    pub struct ScaleTo {
        pub start: OnceLock<Vec3>,
        pub end: Vec3,
    }

    impl ScaleTo {
        pub fn end(end: Vec3) -> ScaleTo {
            ScaleTo {
                start: OnceLock::new(),
                end,
            }
        }
    }

    impl Interpolator for ScaleTo {
        type Item = Transform;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let start = self.start.get_or_init(|| item.scale);
            let end = self.end;
            item.scale = start.lerp(end, value);
        }
    }

    pub fn sprite_color_to(end: Color) -> ComponentTween<SpriteColorTo> {
        ComponentTween::new(SpriteColorTo::end(end))
    }

    /// Automatically figure out the current color of this entity and then
    /// tween from there.
    pub struct SpriteColorTo {
        pub start: OnceLock<Color>,
        pub end: Color,
    }

    impl SpriteColorTo {
        pub fn end(end: Color) -> SpriteColorTo {
            SpriteColorTo {
                start: OnceLock::new(),
                end,
            }
        }
    }

    impl Interpolator for SpriteColorTo {
        type Item = Sprite;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let start = *self.start.get_or_init(|| item.color);
            let end = self.end;
            interpolate::SpriteColor { start, end }.interpolate(item, value);
        }
    }
}

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
    let spawn =
        key.just_pressed(MouseButton::Left) || key.pressed(MouseButton::Right);
    if let Some(coord) = coord.0 {
        if spawn {
            let start = Vec3::new(coord.x, coord.y, 1.);
            let end = Vec3::new(0., 0., 0.);
            commands
                .spawn((
                    SpriteBundle {
                        texture: circle_filled_image,
                        transform: Transform::from_translation(start),
                        ..Default::default()
                    },
                    TweenerBundle::new(Duration::from_secs(2)),
                ))
                .tweens()
                .add(parallel((
                    tween(
                        secs(2.),
                        EaseFunction::ExponentialOut,
                        interpolate::translate_to(end)
                            .with_interpolator_boxed(),
                    ),
                    tween(
                        secs(1.),
                        EaseFunction::BackIn,
                        interpolate::scale_to(Vec3::ZERO)
                            .with_interpolator_boxed(),
                    ),
                    tween(
                        secs(1.),
                        EaseFunction::Linear,
                        interpolate::sprite_color_to(Color::PINK)
                            .with_interpolator_boxed(),
                    ),
                )));
        }
    }
}

fn despawn_finished_circle(
    mut commands: Commands,
    mut tweener_ended_reader: EventReader<TweenerEnded>,
) {
    for t in tweener_ended_reader.read() {
        commands.entity(t.tweener).despawn();
    }
}
