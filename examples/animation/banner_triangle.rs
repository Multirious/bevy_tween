use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_tween::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Banner triangle".to_string(),
                    resizable: false,
                    resolution: Vec2::new(1100., 250.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            DefaultTweenPlugins,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let triangle_image = asset_server.load("big_triangle.png");
    let ease = EaseFunction::ExponentialInOut;

    // colors by https://color-hex.org/color-palettes/189
    let colors = [
        Color::rgb_u8(0, 128, 191),
        Color::rgb_u8(0, 172, 223),
        Color::rgb_u8(85, 208, 255),
        Color::rgb_u8(124, 232, 255),
        Color::rgb_u8(204, 249, 255),
    ];
    let triangles = colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            triangle(
                &mut commands,
                triangle_image.clone(),
                *color,
                (i + 2) as f32,
            )
        })
        .collect::<Vec<_>>();

    let secs = 12.;

    commands
        .spawn(
            SpanTweenerBundle::new(Duration::from_secs_f32(secs))
                .with_repeat(Repeat::Infinitely),
        )
        .with_children(|c| {
            snap_rotate(c, triangles[4], secs, 7, 4., ease);
            snap_rotate(c, triangles[3], secs, 7, 6., ease);
            snap_rotate(c, triangles[2], secs, 7, 8., ease);
            snap_rotate(c, triangles[1], secs, 7, 10., ease);
            snap_rotate(c, triangles[0], secs, 7, 12., ease);
        });

    commands
        .spawn((
            SpatialBundle::default(),
            SpanTweenerBundle::new(Duration::from_secs_f32(12. / 7.))
                .with_repeat(Repeat::Infinitely)
                .tween_here(),
            EaseFunction::ExponentialInOut,
            ComponentTween::new(interpolate::Translation {
                start: Vec3::ZERO,
                end: Vec3::new(30. * 10., 0., 0.),
            }),
        ))
        .with_children(|c| {
            let color = Color::WHITE;
            let count = 70;
            let height = 5.;
            let width = 20.;
            let spacing = 30.;
            let x_offset =
                -(width * count as f32 + (spacing - width) * count as f32) / 2.;
            for i in 0..count {
                let i = i as f32;
                c.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(width, height)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        i * spacing + x_offset,
                        0.,
                        0.,
                    ),
                    ..Default::default()
                });
            }
        });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(250., 250.)),
            color: Color::rgb_u8(43, 44, 47),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    });
}

fn snap_rotate(
    c: &mut ChildBuilder<'_>,
    entity: Entity,
    secs: f32,
    max: usize,
    rev: f32,
    ease: EaseFunction,
) {
    for i in 0..max {
        let max = max as f32;
        let i = i as f32;
        c.span_tweens().tween(
            Duration::from_secs_f32(i / max * secs)
                ..Duration::from_secs_f32((i + 1.) / max * secs),
            ease,
            ComponentTween::new_target_boxed(
                entity,
                interpolate::AngleZ {
                    start: rev * TAU * (max - i) / max,
                    end: rev * TAU * (max - i - 1.) / max,
                },
            ),
        );
    }
}

fn triangle(
    commands: &mut Commands,
    texture: Handle<Image>,
    color: Color,
    z: f32,
) -> Entity {
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., z),
            texture,
            ..Default::default()
        },))
        .id()
}
