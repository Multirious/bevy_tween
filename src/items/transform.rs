use super::Set;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
pub struct Translation;
impl Set<Transform, Vec3> for Translation {
    fn set(&self, item: &mut Transform, value: &Vec3) {
        item.translation = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct Rotation;
impl Set<Transform, Quat> for Rotation {
    fn set(&self, item: &mut Transform, value: &Quat) {
        item.rotation = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct Scale;
impl Set<Transform, Vec3> for Scale {
    fn set(&self, item: &mut Transform, value: &Vec3) {
        item.scale = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct AngleZ;
impl Set<Transform, f32> for AngleZ {
    fn set(&self, item: &mut Transform, value: &f32) {
        item.rotation = Quat::from_rotation_z(*value);
    }
}
