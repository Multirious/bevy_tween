use crate::prelude::Interpolator;
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

    fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
        item.0 = self.start.mix(&self.end, value)
    }
}

/// delta [`Interpolator`] for Bevy's [`BackgroundColor`](bevy::prelude::BackgroundColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BackgroundColorDelta {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

impl Interpolator for BackgroundColorDelta {
    type Item = bevy::prelude::BackgroundColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear().to_vec4();
        let next_color_as_vec = self.start.mix(&self.end, value).to_linear().to_vec4();
        let color_delta = next_color_as_vec - previous_color_as_vec;
        let updated_color = item.0.to_linear().to_vec4() + color_delta;
        item.0 = Color::srgba(updated_color.x, updated_color.y, updated_color.z, updated_color.w);
    }
}

/// Constructor for [`BackgroundColor`](crate::interpolate::BackgroundColor)
pub fn background_color(start: Color, end: Color) -> BackgroundColor {
    BackgroundColor { start, end }
}

/// Constructor for [`BackgroundColor`](crate::interpolate::BackgroundColor) that's relative to previous value using currying.
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

    fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
        item.0 = self.start.mix(&self.end, value)
    }
}


/// delta [`Interpolator`] for Bevy's [`BorderColor`](bevy::prelude::BorderColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BorderColorDelta {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

impl Interpolator for BorderColorDelta {
    type Item = bevy::prelude::BackgroundColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear().to_vec4();
        let next_color_as_vec = self.start.mix(&self.end, value).to_linear().to_vec4();
        let color_delta = next_color_as_vec - previous_color_as_vec;
        let updated_color = item.0.to_linear().to_vec4() + color_delta;
        item.0 = Color::srgba(updated_color.x, updated_color.y, updated_color.z, updated_color.w);
    }
}


/// Constructor for [`BorderColor`](crate::interpolate::BorderColor)
pub fn border_color(start: Color, end: Color) -> BorderColor {
    BorderColor { start, end }
}

/// Constructor for [`BorderColor`](crate::interpolate::BorderColor) that's relative to previous value using currying.
pub fn border_color_to(to: Color) -> impl Fn(&mut Color) -> BorderColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        border_color(start, end)
    }
}
