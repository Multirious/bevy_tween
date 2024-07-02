use super::Set;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BackgroundColor;
impl Set for BackgroundColor {
    type Item = bevy::prelude::BackgroundColor;
    type Value = Color;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.0 = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BorderColor;

impl Set for BorderColor {
    type Item = bevy::prelude::BorderColor;
    type Value = Color;

    fn set(&self, item: &mut Self::Item, value: &Self::Value) {
        item.0 = *value;
    }
}
