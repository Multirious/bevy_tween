use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_tween::{
    combinator::{parallel, tween_exact, AnimationCommands},
    interpolate::{angle_z, translation},
    prelude::*,
    tween::{AnimationTarget, TargetComponent},
};

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
    // colors by https://color-hex.org/color-palettes/189
    let colors = [
        Color::srgb_u8(0, 128, 191),
        Color::srgb_u8(0, 172, 223),
        Color::srgb_u8(85, 208, 255),
        Color::srgb_u8(124, 232, 255),
        Color::srgb_u8(204, 249, 255),
    ];

    let mut spawn_triangle = |color, z| {
        commands
            .spawn((SpriteBundle {
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., z),
                texture: triangle_image.clone(),
                ..Default::default()
            },))
            .id()
    };
    let triangles = colors
        .iter()
        .enumerate()
        .map(|(i, color)| spawn_triangle(*color, (i + 2) as f32))
        .map(|t| t.into_target())
        .collect::<Vec<_>>();

    let secs = 12.;
    let ease = EaseKind::ExponentialInOut;

    commands
        .animation()
        .repeat(Repeat::Infinitely)
        .insert(parallel((
            snap_rotate(triangles[4].clone(), secs, 7, 4., ease),
            snap_rotate(triangles[3].clone(), secs, 7, 6., ease),
            snap_rotate(triangles[2].clone(), secs, 7, 8., ease),
            snap_rotate(triangles[1].clone(), secs, 7, 10., ease),
            snap_rotate(triangles[0].clone(), secs, 7, 12., ease),
        )));

    let dotted_line_target = AnimationTarget.into_target();
    commands
        .spawn((SpatialBundle::default(), AnimationTarget))
        .with_children(dotted_line)
        .animation()
        .repeat(Repeat::Infinitely)
        .insert_tween_here(
            Duration::from_secs_f32(12. / 7.),
            EaseKind::ExponentialInOut,
            dotted_line_target
                .with(translation(Vec3::ZERO, Vec3::new(30. * 10., 0., 0.))),
        );

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(250., 250.)),
            color: Color::srgb_u8(43, 44, 47),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    });
}

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

fn snap_rotate(
    target: TargetComponent,
    dur: f32,
    max: usize,
    rev: f32,
    ease: EaseKind,
) -> impl FnOnce(&mut AnimationCommands, &mut Duration) {
    move |a, pos| {
        for i in 0..max {
            let max = max as f32;
            let i = i as f32;
            tween_exact(
                secs(i / max * dur)..secs((i + 1.) / max * dur),
                ease,
                target.with(angle_z(
                    rev * TAU * (max - i) / max,
                    rev * TAU * (max - i - 1.) / max,
                )),
            )(a, pos);
        }
        *pos += secs(dur)
    }
}

fn dotted_line(c: &mut ChildBuilder) {
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
            transform: Transform::from_xyz(i * spacing + x_offset, 0., 0.),
            ..Default::default()
        });
    }
}
