// type ReflectInterpolatorTransform = ReflectInterpolator<Transform>;

use crate::interpolate::Interpolator;
use bevy::prelude::*;

/// [`Interpolator`] for [`Transform`]'s translation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct Translation {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl Interpolator for Translation {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.translation = self.start.lerp(self.end, value);
    }
}

/// Constructor for [`Translation`]
pub fn translation(start: Vec3, end: Vec3) -> Translation {
    Translation { start, end }
}

/// Constructor for [`Translation`] that's relative to previous value using currying.
pub fn translation_to(to: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        translation(start, end)
    }
}

/// Constructor for [`Translation`] that's relative to previous value using currying.
pub fn translation_by(by: Vec3) -> impl Fn(&mut Vec3) -> Translation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        translation(start, end)
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation using the [`Quat::slerp`] function.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct Rotation {
    #[allow(missing_docs)]
    pub start: Quat,
    #[allow(missing_docs)]
    pub end: Quat,
}
impl Interpolator for Rotation {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.rotation = self.start.slerp(self.end, value);
    }
}

/// Constructor for [`Rotation`]
pub fn rotation(start: Quat, end: Quat) -> Rotation {
    Rotation { start, end }
}

/// Constructor for [`Rotation`] that's relative to previous value using currying.
pub fn rotation_to(to: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        rotation(start, end)
    }
}

/// Constructor for [`Rotation`] that's relative to previous value using currying.
pub fn rotation_by(by: Quat) -> impl Fn(&mut Quat) -> Rotation {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state = state.mul_quat(by);
        rotation(start, end)
    }
}

/// [`Interpolator`] for [`Transform`]'s scale
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct Scale {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl Interpolator for Scale {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        item.scale = self.start.lerp(self.end, value);
    }
}

/// Constructor for [`Scale`]
pub fn scale(start: Vec3, end: Vec3) -> Scale {
    Scale { start, end }
}

/// Constructor for [`Scale`] that's relative to previous value using currying.
pub fn scale_to(to: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        scale(start, end)
    }
}

/// Constructor for [`Scale`] that's relative to previous value using currying.
pub fn scale_by(by: Vec3) -> impl Fn(&mut Vec3) -> Scale {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        scale(start, end)
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation at Z axis.
/// Usually used for 2D rotation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
// #[reflect(InterpolatorTransform)]
pub struct AngleZ {
    #[allow(missing_docs)]
    pub start: f32,
    #[allow(missing_docs)]
    pub end: f32,
}
impl Interpolator for AngleZ {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        let angle = (self.end - self.start).mul_add(value, self.start);
        item.rotation = Quat::from_rotation_z(angle);
    }
}

/// Constructor for [`AngleZ`]
pub fn angle_z(start: f32, end: f32) -> AngleZ {
    AngleZ { start, end }
}

/// Constructor for [`AngleZ`] that's relative to previous value using currying.
pub fn angle_z_to(to: f32) -> impl Fn(&mut f32) -> AngleZ {
    move |state| {
        let start = *state;
        let end = to;
        *state = to;
        angle_z(start, end)
    }
}

/// Constructor for [`AngleZ`] that's relative to previous value using currying.
pub fn angle_z_by(by: f32) -> impl Fn(&mut f32) -> AngleZ {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        angle_z(start, end)
    }
}
