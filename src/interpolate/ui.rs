use crate::prelude::Interpolator;
use crate::utils::color_lerp;
use bevy::prelude::*;

/// [`Interpolator`] for Bevy's [`BackgroundColor`](bevy::prelude::BackgroundColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BackgroundColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

impl Interpolator for BackgroundColor {
    type Item = bevy::prelude::BackgroundColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.0 = color_lerp(self.start, self.end, value)
    }
}

/// Constructor for [`BackgroundColor`]
pub fn background_color(start: Color, end: Color) -> BackgroundColor {
    BackgroundColor { start, end }
}

/// Constructor for [`BackgroundColor`] that's relative to previous value using currying.
pub fn background_color_to(
    to: Color,
) -> impl Fn(&mut Color) -> BackgroundColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        background_color(start, end)
    }
}

/// [`Interpolator`] for Bevy's [`BorderColor`](bevy::prelude::BorderColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BorderColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

impl Interpolator for BorderColor {
    type Item = bevy::prelude::BorderColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.0 = color_lerp(self.start, self.end, value)
    }
}

/// Constructor for [`BorderColor`]
pub fn border_color(start: Color, end: Color) -> BorderColor {
    BorderColor { start, end }
}

/// Constructor for [`BorderColor`] that's relative to previous value using currying.
pub fn border_color_to(to: Color) -> impl Fn(&mut Color) -> BorderColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        border_color(start, end)
    }
}
