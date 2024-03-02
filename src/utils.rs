#![allow(missing_docs)]
#![allow(unused)]
use bevy::prelude::*;

#[cfg(feature = "bevy_render")]
pub fn color_lerp(start: Color, end: Color, v: f32) -> Color {
    let Color::Rgba {
        red: start_red,
        green: start_green,
        blue: start_blue,
        alpha: start_alpha,
    } = start.as_rgba()
    else {
        unreachable!()
    };
    let Color::Rgba {
        red: end_red,
        green: end_green,
        blue: end_blue,
        alpha: end_alpha,
    } = end.as_rgba()
    else {
        unreachable!()
    };
    Color::Rgba {
        red: start_red.lerp(end_red, v),
        green: start_green.lerp(end_green, v),
        blue: start_blue.lerp(end_blue, v),
        alpha: start_alpha.lerp(end_alpha, v),
    }
}
