use std::{any::TypeId, str::FromStr};

use bevy::{prelude::*, reflect::ParsedPath};
use bevy_tween::{
    builder::WorldSetterMarker,
    prelude::*,
    set::{DynamicSetter, SetterValue},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let mut ec = commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..Default::default()
        },
        ..Default::default()
    });
    let sprite = ec.id().into_target();
    let sprite_color = sprite.set(WorldSetterMarker::<Color>::new(
        DynamicSetter::component_path(
            ParsedPath::parse(".color").unwrap(),
            TypeId::of::<Sprite>(),
            TypeId::of::<SetterValue<Color>>(),
        ),
    ));
    ec.animation().add(sprite_color.tween(
        Color::from(bevy::color::palettes::css::WHITE),
        Color::from(bevy::color::palettes::css::DEEP_PINK),
        Duration::from_secs(5),
        EaseFunction::QuadraticOut,
    ));
}
