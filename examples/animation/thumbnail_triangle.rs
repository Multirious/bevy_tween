use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_tween::{
    prelude::*,
    tween::TargetComponent,
    tweener::{EntitySpawner, TweensBuilder},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let bevy_text = asset_server.load("bevy.png");
    let tween_text = asset_server.load("tween.png");
    let square_image = asset_server.load("big_triangle.png");
    let ease = EaseFunction::ExponentialInOut;

    commands.spawn(SpriteBundle {
        texture: bevy_text,
        transform: Transform::from_xyz(-300., 0., 0.),
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: tween_text,
        transform: Transform::from_xyz(340., 10., 0.),
        ..Default::default()
    });

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
            triangle(&mut commands, square_image.clone(), *color, i as f32)
        })
        .collect::<Vec<_>>();

    let secs = 12.;

    commands
        .spawn(
            SpanTweenerBundle::new(Duration::from_secs_f32(secs))
                .with_repeat(Repeat::Infinitely),
        )
        .with_children(|c| {
            c.tweens()
                .add(snap_rotate(triangles[4], secs, 7, 4., ease))
                .add(snap_rotate(triangles[3], secs, 7, 6., ease))
                .add(snap_rotate(triangles[2], secs, 7, 8., ease))
                .add(snap_rotate(triangles[1], secs, 7, 10., ease))
                .add(snap_rotate(triangles[0], secs, 7, 12., ease));
        });
}

fn snap_rotate<S: EntitySpawner>(
    target: impl Into<TargetComponent>,
    secs: f32,
    max: usize,
    rev: f32,
    ease: EaseFunction,
) -> impl FnOnce(&mut TweensBuilder<S>) {
    move |b| {
        let target = target.into();
        for i in 0..max {
            let max = max as f32;
            let i = i as f32;
            b.tween_exact(
                Duration::from_secs_f32(i / max * secs)
                    ..Duration::from_secs_f32((i + 1.) / max * secs),
                ease,
                ComponentTween::new_target_boxed(
                    target.clone(),
                    interpolate::AngleZ {
                        start: rev * TAU * (max - i) / max,
                        end: rev * TAU * (max - i - 1.) / max,
                    },
                ),
            );
        }
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
