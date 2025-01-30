use bevy_math::prelude::*;
use bevy_transform::prelude::*;

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::Reflect;

use crate::{AlterComponent, AlterSingle};

pub mod types {
    use super::*;
    pub type Translation = AlterComponent<AlterTranslation>;
    pub type Rotation = AlterComponent<AlterRotation>;
    pub type Scale = AlterComponent<AlterScale>;
    pub type AngleZ = AlterComponent<AlterAngleZ>;
}

#[allow(non_upper_case_globals)]
pub mod consts {
    use super::*;
    pub const Translation: types::Translation =
        AlterComponent(AlterTranslation);
    pub const Rotation: types::Rotation = AlterComponent(AlterRotation);
    pub const Scale: types::Scale = AlterComponent(AlterScale);
    pub const AngleZ: types::AngleZ = AlterComponent(AlterAngleZ);
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct AlterTranslation;

impl AlterSingle for AlterTranslation {
    type Value = Vec3;
    type Item = Transform;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.translation = *value;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct AlterRotation;

impl AlterSingle for AlterRotation {
    type Value = Quat;
    type Item = Transform;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.rotation = *value;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct AlterScale;

impl AlterSingle for AlterScale {
    type Value = Vec3;
    type Item = Transform;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.scale = *value;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct AlterAngleZ;

impl AlterSingle for AlterAngleZ {
    type Value = f32;
    type Item = Transform;

    fn alter_single(item: &mut Self::Item, value: &Self::Value) {
        item.rotation = Quat::from_rotation_z(*value);
    }
}
