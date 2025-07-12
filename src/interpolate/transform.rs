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

    fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
        item.translation = self.start.lerp(self.end, value);
    }
}

/// delta [`Interpolator`] for [`Transform`]'s translation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct TranslationDelta {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl Interpolator for TranslationDelta {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        let previous_translation = self.start.lerp(self.end, previous_value);
        let next_translation = self.start.lerp(self.end, value);
        let translation_delta = next_translation - previous_translation;
        item.translation += translation_delta;
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

/// Constructor for [`TranslationDelta`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn translation_delta_by(by: Vec3) -> impl Fn(&mut Vec3) -> TranslationDelta {
    move |state| {
        let start = *state;
        let end = *state + by;
        TranslationDelta { start, end }
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

    fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
        item.rotation = self.start.slerp(self.end, value);
    }
}

/// delta [`Interpolator`] for [`Transform`]'s rotation using the [`Quat::slerp`] function.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct RotationDelta {
    #[allow(missing_docs)]
    pub start: Quat,
    #[allow(missing_docs)]
    pub end: Quat,
}
impl Interpolator for RotationDelta {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        let value_delta = value - previous_value;
        item.rotation = item.rotation.slerp(self.end, value_delta);
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


/// Constructor for [`RotationDelta`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn rotation_delta_by(by: Quat) -> impl Fn(&mut Quat) -> RotationDelta {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state = state.mul_quat(by);
        RotationDelta { start, end }
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

    fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
        item.scale = self.start.lerp(self.end, value);
    }
}


/// delta [`Interpolator`] for [`Transform`]'s scale
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct ScaleDelta {
    #[allow(missing_docs)]
    pub start: Vec3,
    #[allow(missing_docs)]
    pub end: Vec3,
}
impl Interpolator for ScaleDelta {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        let value_delta = value - previous_value;
        item.scale += item.scale.lerp(self.end, value_delta);
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

/// Constructor for [`ScaleDelta`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn scale_delta_by(by: Vec3) -> impl Fn(&mut Vec3) -> ScaleDelta {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        ScaleDelta { start, end }
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

    fn interpolate(&self, item: &mut Self::Item, value: f32, _previous_value: f32) {
        let angle = (self.end - self.start).mul_add(value, self.start);
        item.rotation = Quat::from_rotation_z(angle);
    }
}

/// [`Interpolator`] for [`Transform`]'s rotation at Z axis.
/// Usually used for 2D rotation.
#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub struct AngleZDelta {
    #[allow(missing_docs)]
    pub start: f32,
    #[allow(missing_docs)]
    pub end: f32,
}
impl Interpolator for AngleZDelta {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32, previous_value: f32) {
        let previous_angle = (self.end - self.start).mul_add(previous_value, self.start);
        let update_angle = (self.end - self.start).mul_add(value, self.start);
        let angle_delta_as_quat = Quat::from_rotation_z(update_angle - previous_angle);
        item.rotation = item.rotation.mul_quat(angle_delta_as_quat);
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

/// Constructor for [`AngleZDelta`] that's relative to previous value
/// Since this is a delta tween, it can happen with other ongoing tweens of that type
pub fn angle_z_delta_by(by: f32) -> impl Fn(&mut f32) -> AngleZDelta {
    move |state| {
        let start = *state;
        let end = *state + by;
        *state += by;
        AngleZDelta {start, end}
    }
}
