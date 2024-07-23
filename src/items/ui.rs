use super::{impl_simple_setter, Set};
use bevy::prelude::*;

impl_simple_setter! {
    BackgroundColor,
    |item: &mut bevy::prelude::BackgroundColor, value: &Color| {
        item.0 = *value;
    }
}
impl_simple_setter! {
    BorderColor,
    |item: &mut bevy::prelude::BorderColor, value: &Color| {
        item.0 = *value;
    }
}
