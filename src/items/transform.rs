use super::Set;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
pub struct Translation;
impl Set for Translation {
    type Item = Transform;
    type Value = Vec3;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.translation = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct Rotation;
impl Set for Rotation {
    type Item = Transform;
    type Value = Quat;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.rotation = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct Scale;
impl Set for Scale {
    type Item = Transform;
    type Value = Vec3;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.scale = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct AngleZ;
impl Set for AngleZ {
    type Item = Transform;
    type Value = f32;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.rotation = Quat::from_rotation_z(*value);
    }
}
