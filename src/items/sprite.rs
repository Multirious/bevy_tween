use super::Set;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct SpriteColor;

impl Set for SpriteColor {
    type Item = Sprite;
    type Value = Color;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.color = *value;
    }
}

#[cfg(feature = "bevy_asset")]
#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct ColorMaterial;

#[cfg(feature = "bevy_asset")]
impl Set for ColorMaterial {
    type Item = bevy::prelude::ColorMaterial;
    type Value = Color;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.color = *value;
    }
}
