use std::f32::consts::PI;

use bevy::{
    color::{palettes::css::WHITE, Srgba},
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    window,
};
use bevy_tween::{
    builder::{go, parallel, tween_exact, AnimationCommands},
    prelude::*,
};

const SCALE: f32 = 2.0;

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

    let dot_color: Color = WHITE.with_alpha(0.2).into();
    let white_color: Color = (WHITE * 2.).into();
    let text_pop_scale = 1.2;

    let blue_glow: Color = (Srgba::rgb_u8(103, 163, 217) * 5.).into();
    let pink_glow: Color = (Srgba::rgb_u8(248, 183, 205) * 5.).into();

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
    let dot_grid = dot_grid.into_target();
    let triangle_id = commands
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
    let triangle = triangle_id.into_target();
    let square_id = commands
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
    let square = square_id.into_target();
    let bevy_tween_text = commands
        .spawn(SpriteBundle {
            texture: bevy_tween_image,
            transform: Transform::from_scale(Vec3::ONE * SCALE),
            ..Default::default()
        })
        .id();
    let bevy_tween_text = bevy_tween_text.into_target();
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
    let cornering_left = cornering_left.into_target();
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
    let cornering_right = cornering_right.into_target();

    let square_and_triangle = [triangle_id, square_id].into_target();

    // ========================================================================
    let mut bevy_tween_text_color = bevy_tween_text.state(white_color);
    let mut bevy_tween_text_angle_z = bevy_tween_text.state(PI);
    let mut bevy_tween_text_scale = bevy_tween_text.state(Vec3::ZERO * SCALE);
    let mut square_and_triangle_scale =
        square_and_triangle.state(Vec3::ZERO * SCALE);
    let mut square_and_triangle_alpha = square_and_triangle.state(1.);
    let mut square_angle_z = square.state(0.);
    let mut triangle_angle_z = triangle.state(0.);
    let mut triangle_translation = triangle.state(Vec3::ZERO);
    let mut square_translation = square.state(Vec3::ZERO);
    let mut cornering_right_translation =
        cornering_right.state(cornering_right_tween_start);
    let mut cornering_left_translation =
        cornering_left.state(cornering_left_tween_start);
    let mut dot_grid_scale = dot_grid.state(Vec3::new(0.01, 0.01, 0.) * SCALE);

    fn secs(secs: f32) -> Duration {
        Duration::from_secs_f32(secs)
    }
    commands
        .animation()
        .repeat(Repeat::Infinitely)
        .add(parallel((
            (
                set_value(
                    bevy_tween_text_color.with(sprite_color_to(white_color)),
                ),
                tween_exact(
                    secs(0.)..secs(5.),
                    EaseFunction::QuinticOut,
                    bevy_tween_text_angle_z.with(angle_z_to(PI * 4.)),
                ),
                tween_exact(
                    secs(0.)..secs(9.),
                    EaseFunction::CircularOut,
                    bevy_tween_text_scale.with(scale_to(Vec3::ONE * SCALE)),
                ),
                tween_exact(
                    secs(11.)..secs(11.5),
                    EaseFunction::SineOut,
                    bevy_tween_text_scale
                        .with(scale_to(Vec3::ONE * text_pop_scale * SCALE)),
                ),
                tween_exact(
                    secs(11.5)..secs(12.),
                    EaseFunction::SineIn,
                    bevy_tween_text_scale.with(scale_to(Vec3::ZERO * SCALE)),
                ),
                tween_exact(
                    secs(10.)..secs(12.),
                    EaseFunction::QuinticIn,
                    bevy_tween_text_color
                        .with(sprite_color_to(white_color.with_alpha(0.0))),
                ),
                tween_exact(
                    secs(11.)..secs(12.),
                    EaseFunction::QuinticIn,
                    bevy_tween_text_angle_z.with(angle_z_to(PI * 7.)),
                ),
            ),
            (
                set_value(square_and_triangle_alpha.with(sprite_alpha_to(1.))),
                tween_exact(
                    secs(0.)..secs(9.),
                    EaseFunction::CircularOut,
                    square_and_triangle_scale.with(scale_to(Vec3::ONE * SCALE)),
                ),
                tween_exact(
                    secs(4.)..secs(10.),
                    EaseFunction::ExponentialInOut,
                    square_and_triangle_alpha.with(sprite_alpha_to(0.)),
                ),
                tween_exact(
                    secs(0.)..secs(12.),
                    EaseFunction::ExponentialOut,
                    triangle_angle_z.with(angle_z_to(-PI * 10.)),
                ),
                tween_exact(
                    secs(0.)..secs(12.),
                    EaseFunction::ExponentialOut,
                    square_angle_z.with(angle_z_to(PI * 10.)),
                ),
                tween_exact(
                    secs(0.)..secs(4.),
                    EaseFunction::ExponentialOut,
                    triangle_translation.with(translation_to(
                        Vec3::new(150., -20., 0.) * SCALE,
                    )),
                ),
                tween_exact(
                    secs(0.)..secs(4.),
                    EaseFunction::ExponentialOut,
                    square_translation.with(translation_to(
                        Vec3::new(-150., 20., 0.) * SCALE,
                    )),
                ),
            ),
            (
                tween_exact(
                    secs(6.)..secs(6.2),
                    EaseFunction::Linear,
                    cornering_left_translation
                        .with(translation_to(destinated_cornering_left)),
                ),
                tween_exact(
                    secs(6.)..secs(6.2),
                    EaseFunction::Linear,
                    cornering_right_translation
                        .with(translation_to(destinated_cornering_right)),
                ),
                tween_exact(
                    secs(9.8)..secs(10.),
                    EaseFunction::Linear,
                    cornering_left_translation
                        .with(translation_to(cornering_left_tween_end)),
                ),
                tween_exact(
                    secs(9.8)..secs(10.),
                    EaseFunction::Linear,
                    cornering_right_translation
                        .with(translation_to(cornering_right_tween_end)),
                ),
            ),
            (
                tween_exact(
                    secs(0.)..secs(5.),
                    EaseFunction::QuinticOut,
                    dot_grid_scale
                        .with(scale_to(Vec3::new(0.4, 0.4, 0.) * SCALE)),
                ),
                tween_exact(
                    secs(11.5)..secs(12.),
                    EaseFunction::QuadraticInOut,
                    dot_grid_scale
                        .with(scale_to(Vec3::new(0.01, 0.01, 0.) * SCALE)),
                ),
            ),
            go(secs(12.)),
        )));
}

type InterpolateSpriteAlpha = Box<dyn Interpolator<Item = Sprite>>;
fn sprite_alpha(start: f32, end: f32) -> InterpolateSpriteAlpha {
    Box::new(interpolate::closure(move |sprite: &mut Sprite, value| {
        sprite.color = sprite.color.with_alpha(start.lerp(end, value));
    }))
}

fn sprite_alpha_to(to: f32) -> impl Fn(&mut f32) -> InterpolateSpriteAlpha {
    move |alpha| {
        let a = sprite_alpha(*alpha, to);
        *alpha = to;
        a
    }
}

fn set_value<B: Bundle>(
    interpolator: B,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |a, pos| {
        tween_exact(*pos..=*pos, EaseFunction::Linear, interpolator)(a, pos)
    }
}
