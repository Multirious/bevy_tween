use std::f32::consts::PI;

use bevy::render::view::Hdr;
use bevy::{
    color::{Srgba, palettes::css::WHITE},
    core_pipeline::tonemapping::Tonemapping,
    ecs::schedule::ScheduleLabel,
    post_process::bloom::Bloom,
    prelude::*,
    window,
};
use bevy_tween::{
    combinator::{AnimationCommands, go, parallel, sequence, tween_exact},
    prelude::*,
};

const SCALE: u32 = 2;
const SCALE_AS_F32: f32 = SCALE as f32;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy_tween animated banner".to_string(),
                    resizable: false,
                    resolution: window::WindowResolution::new(
                        550 * SCALE,
                        100 * SCALE,
                    ),
                    enabled_buttons: window::EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
            DefaultTweenPluginsOnDefaultTime::default(),
        ))
        .add_systems(Startup, (animation, setup_camera))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
        Tonemapping::TonyMcMapface,
        Hdr,
        Bloom::default(),
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
    let white_color: Color = (WHITE * 2.).with_alpha(1.).into();
    let text_pop_scale = 1.2;

    let blue_normal = Srgba::rgb_u8(103, 163, 217);
    let pink_normal = Srgba::rgb_u8(248, 183, 205);
    let blue_glow = (blue_normal * 5.).with_alpha(1.);
    let pink_glow = (pink_normal * 5.).with_alpha(1.);
    let blue_normal = Color::from(blue_normal);
    let pink_normal = Color::from(pink_normal);
    let blue_glow = Color::from(blue_glow);
    let pink_glow = Color::from(pink_glow);

    let cornering_tween_offset = 200. * SCALE_AS_F32;
    let destinated_cornering_left = Vec3::new(-300., -100., 0.) * SCALE_AS_F32;
    let destinated_cornering_right = Vec3::new(300., 100., 0.) * SCALE_AS_F32;

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
            .spawn((Transform::IDENTITY, Visibility::Visible))
            .with_children(|c| {
                for x in 0..x_count {
                    for y in 0..y_count {
                        let x = x as f32;
                        let y = y as f32;
                        let id = c
                            .spawn((
                                Sprite {
                                    image: dot_image.clone(),
                                    color: dot_color,
                                    ..default()
                                },
                                Transform::from_xyz(
                                    (x * spacing) + offset_x,
                                    (y * spacing) + offset_y,
                                    0.,
                                ),
                            ))
                            .id();
                        dot_grid_children.push(id);
                    }
                }
            })
            .id()
    };
    let dot_grid = dot_grid.into_target();
    let triangle_id = commands
        .spawn((
            Sprite {
                image: triangle_image,
                color: pink_glow,
                ..Default::default()
            },
            Transform::from_scale(Vec3::ONE * SCALE_AS_F32),
        ))
        .id();
    let triangle = triangle_id.into_target();
    let square_id = commands
        .spawn((
            Sprite {
                image: square_image,
                color: blue_glow,
                ..Default::default()
            },
            Transform::from_scale(Vec3::ONE * SCALE_AS_F32),
        ))
        .id();
    let square = square_id.into_target();
    let bevy_tween_text = commands
        .spawn((
            Sprite {
                image: bevy_tween_image,
                ..default()
            },
            Transform::from_scale(Vec3::ONE * SCALE_AS_F32),
        ))
        .id();
    let bevy_tween_text = bevy_tween_text.into_target();
    let cornering_left = commands
        .spawn((
            Sprite {
                image: square_filled_image.clone(),
                color: white_color,
                ..default()
            },
            Transform {
                translation: cornering_left_tween_start,
                rotation: Quat::from_rotation_z(PI / 4.),
                scale: Vec3::ONE * 5. * SCALE_AS_F32,
            },
        ))
        .id();
    let cornering_left = cornering_left.into_target();
    let cornering_right = commands
        .spawn((
            Sprite {
                image: square_filled_image.clone(),
                color: white_color,
                ..default()
            },
            Transform {
                translation: cornering_right_tween_start,
                rotation: Quat::from_rotation_z(PI / 4.),
                scale: Vec3::ONE * 5. * SCALE_AS_F32,
            },
        ))
        .id();
    let cornering_right = cornering_right.into_target();

    let square_and_triangle = [triangle_id, square_id].into_target();

    // ========================================================================
    let mut bevy_tween_text_color = bevy_tween_text.state(white_color);
    let mut bevy_tween_text_angle_z = bevy_tween_text.state(PI);
    let mut bevy_tween_text_scale =
        bevy_tween_text.state(Vec3::ZERO * SCALE_AS_F32);
    let mut square_and_triangle_scale =
        square_and_triangle.state(Vec3::ZERO * SCALE_AS_F32);
    let mut square_and_triangle_alpha = square_and_triangle.state(1.);
    let mut square_angle_z = square.state(0.);
    let mut square_color = square.state(blue_glow.with_alpha(0.));
    let mut triangle_angle_z = triangle.state(0.);
    let mut triangle_translation = triangle.state(Vec3::ZERO);
    let mut triangle_color = triangle.state(pink_glow.with_alpha(0.));
    let mut square_translation = square.state(Vec3::ZERO);
    let mut cornering_right_translation =
        cornering_right.state(cornering_right_tween_start);
    let mut cornering_left_translation =
        cornering_left.state(cornering_left_tween_start);
    let mut dot_grid_scale =
        dot_grid.state(Vec3::new(0.01, 0.01, 0.) * SCALE_AS_F32);

    fn secs(secs: f32) -> Duration {
        Duration::from_secs_f32(secs)
    }
    commands
        .animation()
        .repeat(Repeat::Infinitely)
        .insert(parallel((
            (
                set_value(
                    bevy_tween_text_color.with(sprite_color_to(white_color)),
                ),
                tween_exact(
                    secs(0.)..secs(5.),
                    EaseKind::QuinticOut,
                    bevy_tween_text_angle_z.with(angle_z_to(PI * 4.)),
                ),
                tween_exact(
                    secs(0.)..secs(9.),
                    EaseKind::CircularOut,
                    bevy_tween_text_scale
                        .with(scale_to(Vec3::ONE * SCALE_AS_F32)),
                ),
                tween_exact(
                    secs(11.)..secs(11.5),
                    EaseKind::SineOut,
                    bevy_tween_text_scale.with(scale_to(
                        Vec3::ONE * text_pop_scale * SCALE_AS_F32,
                    )),
                ),
                tween_exact(
                    secs(11.5)..secs(12.),
                    EaseKind::SineIn,
                    bevy_tween_text_scale
                        .with(scale_to(Vec3::ZERO * SCALE_AS_F32)),
                ),
                tween_exact(
                    secs(10.)..secs(12.),
                    EaseKind::QuinticIn,
                    bevy_tween_text_color
                        .with(sprite_color_to(white_color.with_alpha(0.0))),
                ),
                tween_exact(
                    secs(11.)..secs(12.),
                    EaseKind::QuinticIn,
                    bevy_tween_text_angle_z.with(angle_z_to(PI * 7.)),
                ),
            ),
            (
                sequence((
                    // the objects is visible for a split second without this
                    go(secs(0.1)),
                    set_value(square_color.with(sprite_color_to(blue_glow))),
                    set_value(triangle_color.with(sprite_color_to(pink_glow))),
                )),
                tween_exact(
                    secs(0.)..secs(9.),
                    EaseKind::CircularOut,
                    square_and_triangle_scale
                        .with(scale_to(Vec3::ONE * SCALE_AS_F32)),
                ),
                tween_exact(
                    secs(4.)..secs(10.),
                    EaseKind::ExponentialInOut,
                    square_color.with(sprite_color_to(blue_normal)),
                ),
                tween_exact(
                    secs(4.)..secs(10.),
                    EaseKind::ExponentialInOut,
                    triangle_color.with(sprite_color_to(pink_normal)),
                ),
                tween_exact(
                    secs(4.)..secs(10.),
                    EaseKind::ExponentialInOut,
                    square_and_triangle_alpha.with(sprite_alpha_to(0.)),
                ),
                tween_exact(
                    secs(0.)..secs(12.),
                    EaseKind::ExponentialOut,
                    triangle_angle_z.with(angle_z_to(-PI * 10.)),
                ),
                tween_exact(
                    secs(0.)..secs(12.),
                    EaseKind::ExponentialOut,
                    square_angle_z.with(angle_z_to(PI * 10.)),
                ),
                tween_exact(
                    secs(0.)..secs(4.),
                    EaseKind::ExponentialOut,
                    triangle_translation.with(translation_to(
                        Vec3::new(150., -20., 0.) * SCALE_AS_F32,
                    )),
                ),
                tween_exact(
                    secs(0.)..secs(4.),
                    EaseKind::ExponentialOut,
                    square_translation.with(translation_to(
                        Vec3::new(-150., 20., 0.) * SCALE_AS_F32,
                    )),
                ),
            ),
            (
                tween_exact(
                    secs(6.)..secs(6.2),
                    EaseKind::Linear,
                    cornering_left_translation
                        .with(translation_to(destinated_cornering_left)),
                ),
                tween_exact(
                    secs(6.)..secs(6.2),
                    EaseKind::Linear,
                    cornering_right_translation
                        .with(translation_to(destinated_cornering_right)),
                ),
                tween_exact(
                    secs(9.8)..secs(10.),
                    EaseKind::Linear,
                    cornering_left_translation
                        .with(translation_to(cornering_left_tween_end)),
                ),
                tween_exact(
                    secs(9.8)..secs(10.),
                    EaseKind::Linear,
                    cornering_right_translation
                        .with(translation_to(cornering_right_tween_end)),
                ),
            ),
            (
                tween_exact(
                    secs(0.)..secs(5.),
                    EaseKind::QuinticOut,
                    dot_grid_scale
                        .with(scale_to(Vec3::new(0.4, 0.4, 0.) * SCALE_AS_F32)),
                ),
                tween_exact(
                    secs(11.5)..secs(12.),
                    EaseKind::QuadraticInOut,
                    dot_grid_scale.with(scale_to(
                        Vec3::new(0.01, 0.01, 0.) * SCALE_AS_F32,
                    )),
                ),
            ),
            go(secs(12.)),
        )));
}

type InterpolateSpriteAlpha = Box<dyn Interpolator<Item = Sprite>>;
fn sprite_alpha(start: f32, end: f32) -> InterpolateSpriteAlpha {
    Box::new(interpolate::closure(
        move |sprite: &mut Sprite, value, _| {
            sprite.color = sprite.color.with_alpha(start.lerp(end, value));
        },
    ))
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
        tween_exact(*pos..=*pos, EaseKind::Linear, interpolator)(a, pos)
    }
}
