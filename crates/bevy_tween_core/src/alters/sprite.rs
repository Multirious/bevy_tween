#![allow(non_upper_case_globals)]

use std::marker::PhantomData;

use bevy_color::Color;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;
use bevy_sprite::{ColorMaterial, Sprite};

use crate::{AlterAsset, AlterComponent, AlterSingle};

pub const SpriteColorLaba: AlterComponent<AlterSpriteColor<bevy_color::Laba>> =
    AlterComponent(AlterSpriteColor(PhantomData));
pub const SpriteColorLinearRgba: AlterComponent<
    AlterSpriteColor<bevy_color::LinearRgba>,
> = AlterComponent(AlterSpriteColor(PhantomData));
pub const SpriteColorOklaba: AlterComponent<
    AlterSpriteColor<bevy_color::Oklaba>,
> = AlterComponent(AlterSpriteColor(PhantomData));
pub const SpriteColorSrgba: AlterComponent<
    AlterSpriteColor<bevy_color::Srgba>,
> = AlterComponent(AlterSpriteColor(PhantomData));
pub const SpriteColorXyza: AlterComponent<AlterSpriteColor<bevy_color::Xyza>> =
    AlterComponent(AlterSpriteColor(PhantomData));

pub const ColorMaterialLaba: AlterAsset<AlterColorMaterial<bevy_color::Laba>> =
    AlterAsset(AlterColorMaterial(PhantomData));
pub const ColorMaterialLinearRgba: AlterAsset<
    AlterColorMaterial<bevy_color::LinearRgba>,
> = AlterAsset(AlterColorMaterial(PhantomData));
pub const ColorMaterialOklaba: AlterAsset<
    AlterColorMaterial<bevy_color::Oklaba>,
> = AlterAsset(AlterColorMaterial(PhantomData));
pub const ColorMaterialSrgba: AlterAsset<
    AlterColorMaterial<bevy_color::Srgba>,
> = AlterAsset(AlterColorMaterial(PhantomData));
pub const ColorMaterialXyza: AlterAsset<AlterColorMaterial<bevy_color::Xyza>> =
    AlterAsset(AlterColorMaterial(PhantomData));

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct AlterSpriteColor<C>(PhantomData<C>);

impl AlterSingle for AlterSpriteColor<bevy_color::Laba> {
    type Value = bevy_color::Laba;
    type Item = Sprite;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Laba(*value);
    }
}

impl AlterSingle for AlterSpriteColor<bevy_color::LinearRgba> {
    type Value = bevy_color::LinearRgba;
    type Item = Sprite;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::LinearRgba(*value);
    }
}

impl AlterSingle for AlterSpriteColor<bevy_color::Oklaba> {
    type Value = bevy_color::Oklaba;
    type Item = Sprite;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Oklaba(*value);
    }
}

impl AlterSingle for AlterSpriteColor<bevy_color::Srgba> {
    type Value = bevy_color::Srgba;
    type Item = Sprite;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Srgba(*value);
    }
}

impl AlterSingle for AlterSpriteColor<bevy_color::Xyza> {
    type Value = bevy_color::Xyza;
    type Item = Sprite;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Xyza(*value);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct AlterColorMaterial<C>(PhantomData<C>);

impl AlterSingle for AlterColorMaterial<bevy_color::Laba> {
    type Value = bevy_color::Laba;
    type Item = ColorMaterial;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Laba(*value);
    }
}

impl AlterSingle for AlterColorMaterial<bevy_color::LinearRgba> {
    type Value = bevy_color::LinearRgba;
    type Item = ColorMaterial;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::LinearRgba(*value);
    }
}

impl AlterSingle for AlterColorMaterial<bevy_color::Oklaba> {
    type Value = bevy_color::Oklaba;
    type Item = ColorMaterial;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Oklaba(*value);
    }
}

impl AlterSingle for AlterColorMaterial<bevy_color::Srgba> {
    type Value = bevy_color::Srgba;
    type Item = ColorMaterial;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Srgba(*value);
    }
}

impl AlterSingle for AlterColorMaterial<bevy_color::Xyza> {
    type Value = bevy_color::Xyza;
    type Item = ColorMaterial;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.color = Color::Xyza(*value);
    }
}
