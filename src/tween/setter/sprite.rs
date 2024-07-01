use super::Setter;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct SpriteColor;

impl Setter<Sprite, Color> for SpriteColor {
    fn set(&self, item: &mut Sprite, value: &Color) {
        item.color = *value;
    }
}

#[cfg(feature = "bevy_asset")]
#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct ColorMaterial;

#[cfg(feature = "bevy_asset")]
impl Setter<bevy::prelude::ColorMaterial, Color> for ColorMaterial {
    fn set(&self, item: &mut bevy::prelude::ColorMaterial, value: &Color) {
        item.color = *value;
    }
}
