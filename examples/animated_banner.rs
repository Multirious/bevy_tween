use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, window};
use bevy_tween::{prelude::*, tween_player::TweenPlayerState};

struct TransformAngleLens {
    start: f32,
    end: f32,
}
impl TweenLens for TransformAngleLens {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        let angle = (self.end - self.start).mul_add(value, self.start);
        item.rotation = Quat::from_rotation_z(angle);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy_tween animated banner".to_string(),
                    resizable: false,
                    resolution: window::WindowResolution::new(550., 100.),
                    enabled_buttons: window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            DefaultTweenPlugins,
        ))
        .add_systems(Startup, (animation, setup_camera))
        .add_systems(
            Update,
            bevy_tween::tween::component_tween_system::<TransformAngleLens>
                .in_set(bevy_tween::TweenSystemSet::ApplyTween),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        ..Default::default()
    },));
}

fn animation(mut commands: Commands, asset_server: Res<AssetServer>) {
    use lenses::*;

    let triangle_image = asset_server.load("triangle.png");
    let square_image = asset_server.load("square.png");
    let bevy_tween_image = asset_server.load("bevy_tween.png");
    let dot_image = asset_server.load("dot.png");

    // ========================================================================

    let blue = Color::rgb(103. / 255., 163. / 255., 217. / 255.);
    let pink = Color::rgb(248. / 255., 183. / 255., 205. / 255.);

    // ========================================================================

    let mut dot_grid_children = vec![];
    let dot_grid = {
        let x_count = 100usize * 2;
        let y_count = 10usize * 2;
        let spacing = 100.;
        let offset_x = -(x_count as f32 * spacing) / 2.;
        let offset_y = -(y_count as f32 * spacing) / 2.;
        commands
            .spawn(SpatialBundle::INHERITED_IDENTITY)
            .with_children(|c| {
                for x in 0..x_count {
                    for y in 0..y_count {
                        let x = x as f32;
                        let y = y as f32;
                        let id = c
                            .spawn(SpriteBundle {
                                texture: dot_image.clone(),
                                transform: Transform::from_xyz(
                                    (x * spacing) + offset_x,
                                    (y * spacing) + offset_y,
                                    0.,
                                ),
                                sprite: Sprite {
                                    color: Color::WHITE.with_a(0.2),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .id();
                        dot_grid_children.push(id);
                    }
                }
            })
            .id()
    };
    let triangle = commands
        .spawn(SpriteBundle {
            texture: triangle_image,
            sprite: Sprite {
                color: pink,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    let square = commands
        .spawn(SpriteBundle {
            texture: square_image,
            sprite: Sprite {
                color: blue,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    let bevy_tween_text = commands
        .spawn(SpriteBundle {
            texture: bevy_tween_image,
            ..Default::default()
        })
        .id();

    // ========================================================================
    fn secs(secs: f32) -> Duration {
        Duration::from_secs_f32(secs)
    }
    commands
        .spawn(SpanTweenPlayerBundle::new(
            TweenPlayerState::new(secs(12.))
                .with_repeat(Some(Repeat::Infinitely)),
        ))
        .with_children(|c| {
            c.spawn((
                SpanTweenBundle::new(secs(0.)..secs(0.), EaseFunction::Linear),
                ComponentTween::new_target(
                    bevy_tween_text,
                    SpriteColorLens {
                        start: Color::WHITE,
                        end: Color::WHITE,
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(secs(0.)..secs(0.), EaseFunction::Linear),
                ComponentTweenBoxed::new_target_map(
                    [triangle, square],
                    |sprite: &mut Sprite, value: f32| {
                        sprite.color = sprite
                            .color
                            .with_a(sprite.color.a().lerp(1., value));
                    },
                ),
            ));

            c.spawn((
                SpanTweenBundle::new(
                    secs(0.00)..secs(12.),
                    EaseFunction::Linear,
                ),
                ComponentTween::new_target(
                    square,
                    TransformAngleLens {
                        start: 0.,
                        end: PI * 10.,
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(0.00)..secs(12.),
                    EaseFunction::Linear,
                ),
                ComponentTween::new_target(
                    triangle,
                    TransformAngleLens {
                        start: 0.,
                        end: -PI * 10.,
                    },
                ),
            ));

            // ================================================================

            c.spawn((
                SpanTweenBundle::new(
                    secs(0.)..secs(9.),
                    EaseFunction::CircularOut,
                ),
                ComponentTween::new_target(
                    [bevy_tween_text, square, triangle],
                    TransformScaleLens {
                        start: Vec3::ZERO,
                        end: Vec3::ONE,
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(0.)..secs(5.),
                    EaseFunction::QuinticOut,
                ),
                ComponentTween::new_target(
                    bevy_tween_text,
                    TransformAngleLens {
                        start: PI,
                        end: PI * 2.,
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(0.)..secs(5.),
                    EaseFunction::QuinticOut,
                ),
                ComponentTween::new_target(
                    dot_grid,
                    TransformScaleLens {
                        start: Vec3::new(0.01, 0.01, 0.),
                        end: Vec3::new(0.4, 0.4, 0.),
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(0.)..secs(4.),
                    EaseFunction::ExponentialOut,
                ),
                ComponentTween::new_target(
                    triangle,
                    TransformTranslationLens {
                        start: Vec3::new(0., 0., 0.),
                        end: Vec3::new(150., -20., 0.),
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(0.)..secs(4.),
                    EaseFunction::ExponentialOut,
                ),
                ComponentTween::new_target(
                    square,
                    TransformTranslationLens {
                        start: Vec3::new(0., 0., 0.),
                        end: Vec3::new(-150., 20., 0.),
                    },
                ),
            ));

            // ================================================================

            c.spawn((
                SpanTweenBundle::new(
                    secs(10.)..secs(12.),
                    EaseFunction::QuinticIn,
                ),
                ComponentTween::new_target(
                    bevy_tween_text,
                    SpriteColorLens {
                        start: Color::WHITE,
                        end: Color::WHITE.with_a(0.0),
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(9.)..secs(12.),
                    EaseFunction::QuinticIn,
                ),
                ComponentTween::new_target(
                    bevy_tween_text,
                    TransformScaleLens {
                        start: Vec3::ONE,
                        end: Vec3::new(0.0, 0.0, 0.),
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(10.)..secs(12.),
                    EaseFunction::QuinticIn,
                ),
                ComponentTween::new_target(
                    bevy_tween_text,
                    TransformAngleLens { start: 0., end: PI },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(10.)..secs(12.),
                    EaseFunction::QuadraticInOut,
                ),
                ComponentTween::new_target(
                    dot_grid,
                    TransformScaleLens {
                        start: Vec3::new(0.4, 0.4, 0.),
                        end: Vec3::new(0.01, 0.01, 0.),
                    },
                ),
            ));
            c.spawn((
                SpanTweenBundle::new(
                    secs(4.)..secs(12.),
                    EaseFunction::ExponentialInOut,
                ),
                ComponentTweenBoxed::new_target_map(
                    [triangle, square],
                    |sprite: &mut Sprite, value: f32| {
                        sprite.color = sprite
                            .color
                            .with_a(sprite.color.a().lerp(0., value));
                    },
                ),
            ));
        });
}
