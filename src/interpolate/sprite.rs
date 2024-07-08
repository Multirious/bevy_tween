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
}

impl Interpolator for SpriteColor {
    type Item = Sprite;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.color = self.start.mix(&self.end, value)
    }
}

/// Constructor for [`SpriteColor`]
pub fn sprite_color(start: Color, end: Color) -> SpriteColor {
    SpriteColor { start, end }
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

// type ReflectInterpolatorColorMaterial =
//     ReflectInterpolator<bevy::sprite::ColorMaterial>;

/// [`Interpolator`] for [`Sprite`]'s [`ColorMaterial`]
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorColorMaterial)]
pub struct ColorMaterial {
    #[allow(missing_docs)]
    pub start: Color,
    #[allow(missing_docs)]
    pub end: Color,
}

impl Interpolator for ColorMaterial {
    type Item = bevy::sprite::ColorMaterial;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.color = self.start.mix(&self.end, value);
    }
}

/// Constructor for [`ColorMaterial`](crate::interpolate::ColorMaterial)
pub fn color_material(start: Color, end: Color) -> ColorMaterial {
    ColorMaterial { start, end }
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
