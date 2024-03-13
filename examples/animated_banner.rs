use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    window,
};
use bevy_tween::{component_tween_system, prelude::*};

const SCALE: f32 = 2.0;

mod my_interpolate {
    use bevy::prelude::*;
    use bevy_tween::prelude::*;
    pub struct Angle {
        pub start: f32,
        pub end: f32,
    }
    impl Interpolator for Angle {
        type Item = Transform;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            let angle = (self.end - self.start).mul_add(value, self.start);
            item.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy_tween animated banner".to_string(),
                    resizable: false,
                    resolution: window::WindowResolution::new(
                        550. * SCALE,
                        100. * SCALE,
                    ),
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
        .add_tween_systems(component_tween_system::<my_interpolate::Angle>())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        BloomSettings::default(),
    ));
}

fn animation(mut commands: Commands, asset_server: Res<AssetServer>) {
    use interpolate::*;

    let triangle_image = asset_server.load("triangle.png");
    let square_image = asset_server.load("square.png");
    let square_filled_image = asset_server.load("square_filled.png");
    let bevy_tween_image = asset_server.load("bevy_tween.png");
    let dot_image = asset_server.load("dot.png");

    // ========================================================================

    let dot_color = Color::WHITE.with_a(0.2);
    let white_color = Color::WHITE * 2.;
    let text_pop_scale = 1.2;

    let blue_glow = Color::rgb(103. / 255., 163. / 255., 217. / 255.) * 5.;
    let pink_glow = Color::rgb(248. / 255., 183. / 255., 205. / 255.) * 5.;

    let cornering_tween_offset = 200. * SCALE;
    let destinated_cornering_left = Vec3::new(-300., -100., 0.) * SCALE;
    let destinated_cornering_right = Vec3::new(300., 100., 0.) * SCALE;

    let cornering_left_tween_start = Vec3::new(
        destinated_cornering_left.x + cornering_tween_offset,
        destinated_cornering_left.y - cornering_tween_offset,
        0.,
    );
    let cornering_left_tween_end = Vec3::new(
        destinated_cornering_left.x - cornering_tween_offset,
        destinated_cornering_left.y + cornering_tween_offset,
        0.,
    );
    let cornering_right_tween_start = Vec3::new(
        destinated_cornering_right.x + cornering_tween_offset,
        destinated_cornering_right.y - cornering_tween_offset,
        0.,
    );
    let cornering_right_tween_end = Vec3::new(
        destinated_cornering_right.x - cornering_tween_offset,
        destinated_cornering_right.y + cornering_tween_offset,
        0.,
    );

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
                                    color: dot_color,
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
                color: pink_glow,
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::ONE * SCALE),
            ..Default::default()
        })
        .id();
    let square = commands
        .spawn(SpriteBundle {
            texture: square_image,
            sprite: Sprite {
                color: blue_glow,
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::ONE * SCALE),
            ..Default::default()
        })
        .id();
    let bevy_tween_text = commands
        .spawn(SpriteBundle {
            texture: bevy_tween_image,
            transform: Transform::from_scale(Vec3::ONE * SCALE),
            ..Default::default()
        })
        .id();
    let cornering_left = commands
        .spawn(SpriteBundle {
            texture: square_filled_image.clone(),
            sprite: Sprite {
                color: white_color,
                ..Default::default()
            },
            transform: Transform {
                translation: cornering_left_tween_start,
                rotation: Quat::from_rotation_z(PI / 4.),
                scale: Vec3::ONE * 5. * SCALE,
            },
            ..Default::default()
        })
        .id();
    let cornering_right = commands
        .spawn(SpriteBundle {
            texture: square_filled_image.clone(),
            sprite: Sprite {
                color: white_color,
                ..Default::default()
            },
            transform: Transform {
                translation: cornering_right_tween_start,
                rotation: Quat::from_rotation_z(PI / 4.),
                scale: Vec3::ONE * 5. * SCALE,
            },
            ..Default::default()
        })
        .id();

    // ========================================================================
    fn secs(secs: f32) -> Duration {
        Duration::from_secs_f32(secs)
    }
    commands
        .spawn(
            SpanTweenerBundle::new(secs(12.)).with_repeat(Repeat::Infinitely),
        )
        .with_children(|c| {
            c.child_tweens()
                // [ bevy_tween_text ] ========================================
                .jump(
                    secs(0.),
                    ComponentTween::new_target(
                        bevy_tween_text,
                        SpriteColor {
                            start: white_color,
                            end: white_color,
                        },
                    ),
                )
                .tween(
                    secs(0.)..secs(5.),
                    EaseFunction::QuinticOut,
                    ComponentTween::new_target(
                        bevy_tween_text,
                        my_interpolate::Angle {
                            start: PI,
                            end: PI * 4.,
                        },
                    ),
                )
                .tween(
                    secs(0.)..secs(9.),
                    EaseFunction::CircularOut,
                    ComponentTween::new_target(
                        bevy_tween_text,
                        Scale {
                            start: Vec3::ZERO * SCALE,
                            end: Vec3::ONE * SCALE,
                        },
                    ),
                )
                .tween(
                    secs(11.)..secs(11.5),
                    EaseFunction::SineOut,
                    ComponentTween::new_target(
                        bevy_tween_text,
                        Scale {
                            start: Vec3::ONE * SCALE,
                            end: Vec3::ONE * text_pop_scale * SCALE,
                        },
                    ),
                )
                .tween(
                    secs(11.5)..secs(12.),
                    EaseFunction::SineIn,
                    ComponentTween::new_target(
                        bevy_tween_text,
                        Scale {
                            start: Vec3::ONE * text_pop_scale * SCALE,
                            end: Vec3::ZERO * SCALE,
                        },
                    ),
                )
                .tween(
                    secs(10.)..secs(12.),
                    EaseFunction::QuinticIn,
                    ComponentTween::new_target(
                        bevy_tween_text,
                        SpriteColor {
                            start: white_color,
                            end: white_color.with_a(0.0),
                        },
                    ),
                )
                .tween(
                    secs(11.)..secs(12.),
                    EaseFunction::QuinticIn,
                    ComponentTween::new_target(
                        bevy_tween_text,
                        my_interpolate::Angle {
                            start: PI * 4.,
                            end: PI * 7.,
                        },
                    ),
                )
                // [ square and triangle ] ====================================
                .jump(
                    secs(0.),
                    ComponentDynTween::new_target_boxed(
                        [square, triangle],
                        interpolate::closure(
                            |sprite: &mut Sprite, value: f32| {
                                sprite.color =
                                    sprite.color.with_a(1_f32.lerp(1., value));
                            },
                        ),
                    ),
                )
                .tween(
                    secs(0.)..secs(9.),
                    EaseFunction::CircularOut,
                    ComponentTween::new_target(
                        [square, triangle],
                        Scale {
                            start: Vec3::ZERO * SCALE,
                            end: Vec3::ONE * SCALE,
                        },
                    ),
                )
                .tween(
                    secs(4.)..secs(12.),
                    EaseFunction::ExponentialInOut,
                    ComponentDynTween::new_target_boxed(
                        [triangle, square],
                        interpolate::closure(
                            |sprite: &mut Sprite, value: f32| {
                                sprite.color = sprite
                                    .color
                                    .with_a(sprite.color.a().lerp(0., value));
                            },
                        ),
                    ),
                )
                .tween(
                    secs(0.)..secs(12.),
                    EaseFunction::ExponentialOut,
                    ComponentTween::new_target(
                        square,
                        my_interpolate::Angle {
                            start: 0.,
                            end: PI * 10.,
                        },
                    ),
                )
                .tween(
                    secs(0.)..secs(12.),
                    EaseFunction::ExponentialOut,
                    ComponentTween::new_target(
                        triangle,
                        my_interpolate::Angle {
                            start: 0.,
                            end: -PI * 10.,
                        },
                    ),
                )
                .tween(
                    secs(0.)..secs(4.),
                    EaseFunction::ExponentialOut,
                    ComponentTween::new_target(
                        triangle,
                        Translation {
                            start: Vec3::new(0., 0., 0.) * SCALE,
                            end: Vec3::new(150., -20., 0.) * SCALE,
                        },
                    ),
                )
                .tween(
                    secs(0.)..secs(4.),
                    EaseFunction::ExponentialOut,
                    ComponentTween::new_target(
                        square,
                        Translation {
                            start: Vec3::new(0., 0., 0.) * SCALE,
                            end: Vec3::new(-150., 20., 0.) * SCALE,
                        },
                    ),
                )
                // [ cornering ] ===============================================
                .tween(
                    secs(6.)..secs(6.2),
                    EaseFunction::Linear,
                    ComponentTween::new_target(
                        cornering_left,
                        Translation {
                            start: cornering_left_tween_start,
                            end: destinated_cornering_left,
                        },
                    ),
                )
                .tween(
                    secs(6.)..secs(6.2),
                    EaseFunction::Linear,
                    ComponentTween::new_target(
                        cornering_right,
                        Translation {
                            start: cornering_right_tween_start,
                            end: destinated_cornering_right,
                        },
                    ),
                )
                .tween(
                    secs(9.8)..secs(10.),
                    EaseFunction::Linear,
                    ComponentTween::new_target(
                        cornering_left,
                        Translation {
                            start: destinated_cornering_left,
                            end: cornering_left_tween_end,
                        },
                    ),
                )
                .tween(
                    secs(9.8)..secs(10.),
                    EaseFunction::Linear,
                    ComponentTween::new_target(
                        cornering_right,
                        Translation {
                            start: destinated_cornering_right,
                            end: cornering_right_tween_end,
                        },
                    ),
                )
                // [ dot_grid ] ===============================================
                .tween(
                    secs(0.)..secs(5.),
                    EaseFunction::QuinticOut,
                    ComponentTween::new_target(
                        dot_grid,
                        Scale {
                            start: Vec3::new(0.01, 0.01, 0.) * SCALE,
                            end: Vec3::new(0.4, 0.4, 0.) * SCALE,
                        },
                    ),
                )
                .tween(
                    secs(11.5)..secs(12.),
                    EaseFunction::QuadraticInOut,
                    ComponentTween::new_target(
                        dot_grid,
                        Scale {
                            start: Vec3::new(0.4, 0.4, 0.) * SCALE,
                            end: Vec3::new(0.01, 0.01, 0.) * SCALE,
                        },
                    ),
                );
        });
}
