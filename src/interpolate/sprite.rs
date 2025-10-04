use crate::interpolate::Interpolator;
use bevy::prelude::*;

// type ReflectInterpolatorSprite = ReflectInterpolator<Sprite>;

/// [`Interpolator`] for [`Sprite`]'s color
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorSprite)]
pub struct SpriteColor {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
    /// whether it increments by delta or sets absolute values
    pub delta: bool
}

impl Interpolator for SpriteColor {
    type Item = Sprite;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta{
            let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear();
            let next_color_as_vec = self.start.mix(&self.end, value).to_linear();
            let updated_color = item.color.to_linear() + (next_color_as_vec - previous_color_as_vec);
            item.color = updated_color.into();
        }else{
            item.color = self.start.mix(&self.end, value)
        }
    }
}

/// Constructor for [`SpriteColor`]
pub fn sprite_color(start: Color, end: Color) -> SpriteColor {
    SpriteColor { start, end, delta: false }
}

/// Constructor for [`SpriteColor`] that's relative to previous value using currying.
pub fn sprite_color_to(to: Color) -> impl Fn(&mut Color) -> SpriteColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        sprite_color(start, end)
    }
}

/// Constructor for delta [`SpriteColor`]
pub fn sprite_color_delta_to(to: Color) -> impl Fn(&mut Color) -> SpriteColor {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        SpriteColor {start, end, delta: true}
    }
}

// type ReflectInterpolatorColorMaterial =
//     ReflectInterpolator<bevy::sprite::ColorMaterial>;

/// [`Interpolator`] for [`Sprite`]'s ColorMaterial
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorColorMaterial)]
pub struct ColorMaterial {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
    /// whether it increments by delta or sets absolute values
    pub delta: bool
}

impl Interpolator for ColorMaterial {
    type Item = bevy::prelude::ColorMaterial;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        if self.delta {
            let previous_color_as_vec = self.start.mix(&self.end, previous_value).to_linear();
            let next_color_as_vec = self.start.mix(&self.end, value).to_linear();
            let updated_color = item.color.to_linear() + (next_color_as_vec - previous_color_as_vec);
            item.color = updated_color.into();
        }else{
            item.color = self.start.mix(&self.end, value);
        }
    }
}


/// Constructor for [`ColorMaterial`](crate::interpolate::ColorMaterial)
pub fn color_material(start: Color, end: Color) -> ColorMaterial {
    ColorMaterial { start, end, delta: false }
}

/// Constructor for [`ColorMaterial`](crate::interpolate::ColorMaterial) that's relative to previous value using currying.
pub fn color_material_to(to: Color) -> impl Fn(&mut Color) -> ColorMaterial {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        color_material(start, end)
    }
}

/// Constructor for delta [`ColorMaterial`](crate::interpolate::ColorMaterial)
pub fn color_material_delta_to(to: Color) -> impl Fn(&mut Color) -> ColorMaterial {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        ColorMaterial{start, end, delta: true}
    }
}
