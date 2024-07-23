use super::{impl_simple_setter, Set};
use bevy::prelude::*;

impl_simple_setter! {
    Translation,
    |item: &mut Transform, value: &Vec3| {
        item.translation = *value;
    }
}
impl_simple_setter! {
    Rotation,
    |item: &mut Transform, value: &Quat| {
        item.rotation = *value;
    }
}
impl_simple_setter! {
    Scale,
    |item: &mut Transform, value: &Vec3| {
        item.scale = *value;
    }
}
impl_simple_setter! {
    AngleZ,
    |item: &mut Transform, value: &f32| {
        item.rotation = Quat::from_rotation_z(*value);
    }
}
