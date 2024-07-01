use super::Set;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BackgroundColor;
impl Set<bevy::prelude::BackgroundColor, Color> for BackgroundColor {
    fn set(&self, item: &mut bevy::prelude::BackgroundColor, value: &Color) {
        item.0 = *value;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BorderColor;

impl Set<bevy::prelude::BorderColor, Color> for BorderColor {
    fn set(&self, item: &mut bevy::prelude::BorderColor, value: &Color) {
        item.0 = *value;
    }
}
