use crate::interpolate::Interpolator;
use bevy::prelude::*;

/// [`Interpolator`] for [`UiTransform`]'s scale
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct UiScale {
    /// Starting [`UiTransform`]'s scale
    pub start: Vec2,
    /// Ending [`UiTransform`]'s scale
    pub end: Vec2,
    /// whether it increments by delta or sets absolute values
    pub delta: bool,
}

impl Interpolator for UiScale {
    type Item = UiTransform;

    fn interpolate(
        &self,
        item: &mut Self::Item,
        value: f32,
        previous_value: f32,
    ) {
        if self.delta {
            let previous_scale = self.start.lerp(self.end, previous_value);
            let next_scale = self.start.lerp(self.end, value);
            let scale_delta = next_scale - previous_scale;
            item.scale += scale_delta;
        } else {
            item.scale = self.start.lerp(self.end, value);
        }
    }
}

/// Constructor for [`crate::interpolate::UiScale`]
pub fn ui_scale(start: Vec2, end: Vec2) -> UiScale {
    UiScale {
        start,
        end,
        delta: false,
    }
}

/// Constructor for [`crate::interpolate::UiScale`] relative to previous value using currying.
pub fn ui_scale_to(to: Vec2) -> impl Fn(&mut Vec2) -> UiScale {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        ui_scale(start, end)
    }
}

/// Constructor for [`crate::interpolate::UiScale`] relative to previous value using currying.
pub fn ui_scale_by(by: Vec2) -> impl Fn(&mut Vec2) -> UiScale {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        ui_scale(start, end)
    }
}

/// Constructor for [`crate::interpolate::UiScale`] relative to previous value.
/// Since this is a delta tween, it can run alongside other ongoing tweens of that type.
pub fn ui_scale_delta_by(by: Vec2) -> impl Fn(&mut Vec2) -> UiScale {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        UiScale {
            start,
            end,
            delta: true,
        }
    }
}
