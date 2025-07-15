use crate::prelude::Interpolator;
use bevy::prelude::*;

/// [`Interpolator`] for Bevy's [`BackgroundColor`](bevy::prelude::BackgroundColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BackgroundColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
    /// whether it increments by delta or sets absolute values
    pub delta: bool
}

impl Interpolator for BackgroundColor {
    type Item = bevy::prelude::BackgroundColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta{
            let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear();
            let next_color_as_vec = self.start.mix(&self.end, value).to_linear();
            let updated_color = item.0.to_linear() + (next_color_as_vec - previous_color_as_vec);
            item.0 = updated_color.into();
        }else{
            item.0 = self.start.mix(&self.end, value)
        }
    }
}

/// Constructor for [`BackgroundColor`](crate::interpolate::BackgroundColor)
pub fn background_color(start: Color, end: Color) -> BackgroundColor {
    BackgroundColor { start, end, delta: false }
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

/// Constructor for delta [`BackgroundColor`]
pub fn background_color_delta_to(
    to: Color,
) -> impl Fn(&mut Color) -> BackgroundColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        BackgroundColor {start, end, delta: true}
    }
}

/// [`Interpolator`] for Bevy's [`BorderColor`](bevy::prelude::BorderColor) used in UIs.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct BorderColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
    /// whether it increments by delta or sets absolute values
    pub delta: bool
}

impl Interpolator for BorderColor {
    type Item = bevy::prelude::BorderColor;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear();
            let next_color_as_vec = self.start.mix(&self.end, value).to_linear();
            let updated_color = item.0.to_linear() + (next_color_as_vec - previous_color_as_vec);
            item.0 = updated_color.into();
        }else{
            item.0 = self.start.mix(&self.end, value)
        }
    }
}

/// Constructor for [`BorderColor`](crate::interpolate::BorderColor)
pub fn border_color(start: Color, end: Color) -> BorderColor {
    BorderColor { start, end, delta: false }
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

/// Constructor for [`BorderColor`] that's relative to previous value using currying.
pub fn border_color_delta_to(to: Color) -> impl Fn(&mut Color) -> BorderColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        BorderColor {start, end, delta: true}
    }
}