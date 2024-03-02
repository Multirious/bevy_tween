use crate::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TransformTranslationLens {
    pub start: Vec3,
    pub end: Vec3,
}
impl TweenLens for TransformTranslationLens {
    type Thing = Transform;

    fn apply(&self, component: &mut Self::Thing, ease_value: f32) {
        component.translation = self.start.lerp(self.end, ease_value);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TransformRotationLens {
    pub start: Quat,
    pub end: Quat,
}
impl TweenLens for TransformRotationLens {
    type Thing = Transform;

    fn apply(&self, component: &mut Self::Thing, ease_value: f32) {
        component.rotation = self.start.slerp(self.end, ease_value);
    }
}
