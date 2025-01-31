use std::marker::PhantomData;

use bevy_color::Color;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;
use bevy_sprite::{ColorMaterial, Sprite};

use crate::{AlterAsset, AlterComponent, AlterSingle};

pub type SpriteColorLaba = AlterComponent<AlterSpriteColor<bevy_color::Laba>>;
pub type SpriteColorLinearRgba =
    AlterComponent<AlterSpriteColor<bevy_color::LinearRgba>>;
pub type SpriteColorOklaba =
    AlterComponent<AlterSpriteColor<bevy_color::Oklaba>>;
pub type SpriteColorSrgba = AlterComponent<AlterSpriteColor<bevy_color::Srgba>>;
pub type SpriteColorXyza = AlterComponent<AlterSpriteColor<bevy_color::Xyza>>;

pub type ColorMaterialLaba = AlterAsset<AlterColorMaterial<bevy_color::Laba>>;
pub type ColorMaterialLinearRgba =
    AlterAsset<AlterColorMaterial<bevy_color::LinearRgba>>;
pub type ColorMaterialOklaba =
    AlterAsset<AlterColorMaterial<bevy_color::Oklaba>>;
pub type ColorMaterialSrgba = AlterAsset<AlterColorMaterial<bevy_color::Srgba>>;
pub type ColorMaterialXyza = AlterAsset<AlterColorMaterial<bevy_color::Xyza>>;

pub fn sprite_color_laba() -> SpriteColorLaba {
    SpriteColorLaba::default()
}
pub fn sprite_color_linear_rgba() -> SpriteColorLinearRgba {
    SpriteColorLinearRgba::default()
}
pub fn sprite_color_oklaba() -> SpriteColorOklaba {
    SpriteColorOklaba::default()
}
pub fn sprite_color_srgba() -> SpriteColorSrgba {
    SpriteColorSrgba::default()
}
pub fn sprite_color_xyza() -> SpriteColorXyza {
    SpriteColorXyza::default()
}

pub fn color_material_laba() -> ColorMaterialLaba {
    ColorMaterialLaba::default()
}
pub fn color_material_linear_rgba() -> ColorMaterialLinearRgba {
    ColorMaterialLinearRgba::default()
}
pub fn color_material_oklaba() -> ColorMaterialOklaba {
    ColorMaterialOklaba::default()
}
pub fn color_material_srgba() -> ColorMaterialSrgba {
    ColorMaterialSrgba::default()
}
pub fn color_material_xyza() -> ColorMaterialXyza {
    ColorMaterialXyza::default()
}

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
