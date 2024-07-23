use super::{impl_simple_setter, Set};
use bevy::prelude::*;

impl_simple_setter! {
    SpriteColor,
    |item: &mut Sprite, value: &Color| {
        item.color = *value;
    }
}
impl_simple_setter! {
    ColorMaterial,
    |item: &mut bevy::prelude::ColorMaterial, value: &Color| {
        item.color = *value;
    }
}
