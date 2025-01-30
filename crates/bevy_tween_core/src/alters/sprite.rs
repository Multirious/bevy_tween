use std::marker::PhantomData;

use bevy_color::Color;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;
use bevy_sprite::{ColorMaterial, Sprite};

use crate::{AlterAsset, AlterComponent, AlterSingle};

pub mod types {
    use super::*;
    pub type SpriteColorLaba =
        AlterComponent<AlterSpriteColor<bevy_color::Laba>>;
    pub type SpriteColorLinearRgba =
        AlterComponent<AlterSpriteColor<bevy_color::LinearRgba>>;
    pub type SpriteColorOklaba =
        AlterComponent<AlterSpriteColor<bevy_color::Oklaba>>;
    pub type SpriteColorSrgba =
        AlterComponent<AlterSpriteColor<bevy_color::Srgba>>;
    pub type SpriteColorXyza =
        AlterComponent<AlterSpriteColor<bevy_color::Xyza>>;

    pub type ColorMaterialLaba =
        AlterAsset<AlterColorMaterial<bevy_color::Laba>>;
    pub type ColorMaterialLinearRgba =
        AlterAsset<AlterColorMaterial<bevy_color::LinearRgba>>;
    pub type ColorMaterialOklaba =
        AlterAsset<AlterColorMaterial<bevy_color::Oklaba>>;
    pub type ColorMaterialSrgba =
        AlterAsset<AlterColorMaterial<bevy_color::Srgba>>;
    pub type ColorMaterialXyza =
        AlterAsset<AlterColorMaterial<bevy_color::Xyza>>;
}

#[allow(non_upper_case_globals)]
pub mod consts {
    use super::*;
    pub const SpriteColorLaba: types::SpriteColorLaba =
        AlterComponent(AlterSpriteColor(PhantomData));
    pub const SpriteColorLinearRgba: types::SpriteColorLinearRgba =
        AlterComponent(AlterSpriteColor(PhantomData));
    pub const SpriteColorOklaba: types::SpriteColorOklaba =
        AlterComponent(AlterSpriteColor(PhantomData));
    pub const SpriteColorSrgba: types::SpriteColorSrgba =
        AlterComponent(AlterSpriteColor(PhantomData));
    pub const SpriteColorXyza: types::SpriteColorXyza =
        AlterComponent(AlterSpriteColor(PhantomData));

    pub const ColorMaterialLaba: types::ColorMaterialLaba =
        AlterAsset(AlterColorMaterial(PhantomData));
    pub const ColorMaterialLinearRgba: types::ColorMaterialLinearRgba =
        AlterAsset(AlterColorMaterial(PhantomData));
    pub const ColorMaterialOklaba: types::ColorMaterialOklaba =
        AlterAsset(AlterColorMaterial(PhantomData));
    pub const ColorMaterialSrgba: types::ColorMaterialSrgba =
        AlterAsset(AlterColorMaterial(PhantomData));
    pub const ColorMaterialXyza: types::ColorMaterialXyza =
        AlterAsset(AlterColorMaterial(PhantomData));
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
