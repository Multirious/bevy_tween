use bevy::{
    color::{
        Color, Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba,
        Xyza,
    },
    math::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4},
    reflect::{FromType, Reflect},
};

/// Bevy don't have general lerp trait.
pub trait Lerp {
    fn lerp(&self, to: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::math::FloatExt::lerp(*self, *to, t)
    }
}

impl Lerp for f64 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::math::FloatExt::lerp(*self, *to, t as f64)
    }
}

impl Lerp for Vec2 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::math::VectorSpace::lerp(self, *to, t)
    }
}

impl Lerp for Vec3 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::math::VectorSpace::lerp(self, *to, t)
    }
}

impl Lerp for Vec4 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::math::VectorSpace::lerp(self, *to, t)
    }
}

impl Lerp for DVec2 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        let t = t as f64;
        *self * (1. - t) + *to * t
    }
}

impl Lerp for DVec3 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        let t = t as f64;
        *self * (1. - t) + *to * t
    }
}

impl Lerp for DVec4 {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        let t = t as f64;
        *self * (1. - t) + *to * t
    }
}

impl Lerp for Srgba {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for LinearRgba {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Hsla {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Hsva {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Hwba {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Laba {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Lcha {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Oklaba {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Oklcha {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Xyza {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}
impl Lerp for Color {
    #[inline]
    fn lerp(&self, to: &Self, t: f32) -> Self {
        bevy::color::Mix::mix(self, to, t)
    }
}

macro_rules! impl_lerp_tuple {
    ($($i:tt $t:ident)+) => {
        impl<$($t: Lerp,)+> Lerp for ($($t,)*) {
            #[inline]
            fn lerp(&self, to: &Self, t: f32) -> Self {
                (
                    $(
                        self.$i.lerp(&to.$i, t),
                    )+
                )
            }
        }
    };
}

impl_lerp_tuple! { 0 T0 }
impl_lerp_tuple! { 0 T0 1 T1 }
impl_lerp_tuple! { 0 T0 1 T1 2 T2 }
impl_lerp_tuple! { 0 T0 1 T1 2 T2 3 T3 }
impl_lerp_tuple! { 0 T0 1 T1 2 T2 3 T3 4 T4 }
impl_lerp_tuple! { 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 }

#[derive(Clone)]
pub struct ReflectLerp {
    lerp: fn(&dyn Reflect, &dyn Reflect, f32) -> Option<Box<dyn Reflect>>,
}

impl ReflectLerp {
    pub fn lerp(
        &self,
        from: &dyn Reflect,
        to: &dyn Reflect,
        t: f32,
    ) -> Option<Box<dyn Reflect>> {
        (self.lerp)(from, to, t)
    }
}

impl<T: Reflect + Lerp> FromType<T> for ReflectLerp {
    fn from_type() -> Self {
        ReflectLerp {
            lerp: |from, to, t| {
                let from = from.downcast_ref::<T>()?;
                let to = to.downcast_ref::<T>()?;
                Some(Box::new(from.lerp(to, t)))
            },
        }
    }
}
