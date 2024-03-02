use bevy::reflect::Reflect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum EaseFunctionEnum {
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SineIn,
    SineOut,
    SineInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}
impl EaseFunctionEnum {
    pub fn function(&self) -> fn(f32) -> f32 {
        match self {
            EaseFunctionEnum::Linear => linear,
            EaseFunctionEnum::QuadraticIn => quadratic_in,
            EaseFunctionEnum::QuadraticOut => quadratic_out,
            EaseFunctionEnum::QuadraticInOut => quadratic_in_out,
            EaseFunctionEnum::CubicIn => cubic_in,
            EaseFunctionEnum::CubicOut => cubic_out,
            EaseFunctionEnum::CubicInOut => cubic_in_out,
            EaseFunctionEnum::QuarticIn => quartic_in,
            EaseFunctionEnum::QuarticOut => quartic_out,
            EaseFunctionEnum::QuarticInOut => quartic_in_out,
            EaseFunctionEnum::QuinticIn => quintic_in,
            EaseFunctionEnum::QuinticOut => quintic_out,
            EaseFunctionEnum::QuinticInOut => quintic_in_out,
            EaseFunctionEnum::SineIn => sine_in,
            EaseFunctionEnum::SineOut => sine_out,
            EaseFunctionEnum::SineInOut => sine_in_out,
            EaseFunctionEnum::CircularIn => circular_in,
            EaseFunctionEnum::CircularOut => circular_out,
            EaseFunctionEnum::CircularInOut => circular_in_out,
            EaseFunctionEnum::ExponentialIn => exponential_in,
            EaseFunctionEnum::ExponentialOut => exponential_out,
            EaseFunctionEnum::ExponentialInOut => exponential_in_out,
            EaseFunctionEnum::ElasticIn => elastic_in,
            EaseFunctionEnum::ElasticOut => elastic_out,
            EaseFunctionEnum::ElasticInOut => elastic_in_out,
            EaseFunctionEnum::BackIn => back_in,
            EaseFunctionEnum::BackOut => back_out,
            EaseFunctionEnum::BackInOut => back_in_out,
            EaseFunctionEnum::BounceIn => bounce_in,
            EaseFunctionEnum::BounceOut => bounce_out,
            EaseFunctionEnum::BounceInOut => bounce_in_out,
        }
    }
}

pub trait EaseFunction: Send + Sync + 'static {
    fn value(&self, ratio: f32) -> f32;
}

impl<F: Fn(f32) -> f32 + Send + Sync + 'static> EaseFunction for F {
    fn value(&self, ratio: f32) -> f32 {
        self(ratio)
    }
}

pub enum EaseMethod {
    EaseFunction(EaseFunctionEnum),
    Custom(fn(f32) -> f32),
    CustomBoxed(Box<dyn EaseFunction>),
}
impl std::fmt::Debug for EaseMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EaseMethod::EaseFunction(ef) => {
                f.debug_tuple("EaseFunction").field(ef).finish()
            }
            EaseMethod::Custom(c) => f.debug_tuple("Custom").field(c).finish(),
            EaseMethod::CustomBoxed(_) => {
                f.debug_struct("CustomBoxed(_)").finish()
            }
        }
    }
}
impl EaseMethod {
    pub fn value(&self, ratio: f32) -> f32 {
        match self {
            EaseMethod::EaseFunction(r) => r.function()(ratio),
            EaseMethod::Custom(c) => c(ratio),
            EaseMethod::CustomBoxed(c) => c.value(ratio),
        }
    }
}

impl From<fn(f32) -> f32> for EaseMethod {
    fn from(value: fn(f32) -> f32) -> Self {
        EaseMethod::Custom(value)
    }
}
impl From<EaseFunctionEnum> for EaseMethod {
    fn from(value: EaseFunctionEnum) -> Self {
        EaseMethod::EaseFunction(value)
    }
}

fn clamp(p: f32) -> f32 {
    match () {
        _ if p > 1.0 => 1.0,
        _ if p < 0.0 => 0.0,
        _ => p,
    }
}

pub fn linear(v: f32) -> f32 {
    v
}
pub fn quadratic_in(v: f32) -> f32 {
    let p = clamp(v);
    p * p
}

pub fn quadratic_out(v: f32) -> f32 {
    let p = clamp(v);
    -(p * (p - 2.0))
}

pub fn quadratic_in_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 0.5 {
        2.0 * p * p
    } else {
        (-2.0 * p * p) + (4.0 * p) - 1.0
    }
}

pub fn cubic_in(v: f32) -> f32 {
    let p = clamp(v);
    p * p * p
}

pub fn cubic_out(v: f32) -> f32 {
    let p = clamp(v);
    let f = p - 1.0;
    f * f * f + 1.0
}

pub fn cubic_in_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 0.5 {
        4.0 * p * p * p
    } else {
        let f = (2.0 * p) - 2.0;
        0.5 * f * f * f + 1.0
    }
}

pub fn quartic_in(v: f32) -> f32 {
    let p = clamp(v);
    p * p * p * p
}

pub fn quartic_out(v: f32) -> f32 {
    let p = clamp(v);
    let f = p - 1.0;
    f * f * f * (1.0 - p) + 1.0
}

pub fn quartic_in_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 0.5 {
        8.0 * p * p * p * p
    } else {
        let f = p - 1.0;
        -8.0 * f * f * f * f + 1.0
    }
}

pub fn quintic_in(v: f32) -> f32 {
    let p = clamp(v);
    p * p * p * p * p
}

pub fn quintic_out(v: f32) -> f32 {
    let p = clamp(v);
    let f = p - 1.0;
    f * f * f * f * f + 1.0
}

pub fn quintic_in_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 0.5 {
        16.0 * p * p * p * p * p
    } else {
        let f = (2.0 * p) - 2.0;
        0.5 * f * f * f * f * f + 1.0
    }
}

pub fn sine_in(v: f32) -> f32 {
    let p = clamp(v);
    ((p - 1.0) * std::f32::consts::TAU).sin() + 1.0
}

pub fn sine_out(v: f32) -> f32 {
    let p = clamp(v);
    (p * std::f32::consts::TAU).sin()
}

pub fn sine_in_out(v: f32) -> f32 {
    let p = clamp(v);
    0.5 * (1.0 - (p * std::f32::consts::PI).cos())
}

pub fn circular_in(v: f32) -> f32 {
    let p = clamp(v);
    1.0 - (1.0 - (p * p)).sqrt()
}

pub fn circular_out(v: f32) -> f32 {
    let p = clamp(v);
    ((2.0 - p) * p).sqrt()
}

pub fn circular_in_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 0.5 {
        0.5 * (1.0 - (1.0 - 4.0 * (p * p)).sqrt())
    } else {
        0.5 * ((-((2.0 * p) - 3.0) * ((2.0 * p) - 1.0)).sqrt() + 1.0)
    }
}

pub fn exponential_in(v: f32) -> f32 {
    if v <= 0.0 {
        0.0
    } else {
        (2.0_f32).powf(10.0 * (v.min(1.0) - 1.0))
    }
}

pub fn exponential_out(v: f32) -> f32 {
    if v >= 1.0 {
        1.0
    } else {
        1.0 - (2.0_f32).powf(-10.0 * v.max(0.0))
    }
}

pub fn exponential_in_out(v: f32) -> f32 {
    if v <= 0.0 {
        return 0.0;
    }
    if v >= 1.0 {
        return 1.0;
    }

    if v < 0.5 {
        0.5 * (2.0_f32).powf((20.0 * v) - 10.0)
    } else {
        -0.5 * (2.0_f32).powf((-20.0 * v) + 10.0) + 1.0
    }
}

pub fn elastic_in(v: f32) -> f32 {
    let p = clamp(v);
    (13.0 * std::f32::consts::TAU * p).sin() * (2.0_f32).powf(10.0 * (p - 1.0))
}

pub fn elastic_out(v: f32) -> f32 {
    let p = clamp(v);
    (-13.0 * std::f32::consts::TAU * (p + 1.0)).sin()
        * (2.0_f32).powf(-10.0 * p)
        + 1.0
}

pub fn elastic_in_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 0.5 {
        0.5 * (13.0 * std::f32::consts::TAU * (2.0 * p)).sin()
            * (2.0_f32).powf(10.0 * ((2.0 * p) - 1.0))
    } else {
        0.5 * ((-13.0 * std::f32::consts::TAU * ((2.0 * p - 1.0) + 1.0)).sin()
            * (2.0_f32).powf(-10.0 * (2.0 * p - 1.0))
            + 2.0)
    }
}

pub fn back_in(v: f32) -> f32 {
    use std::f32::consts::PI;
    let p = clamp(v);
    p * p * p - p * (p * PI).sin()
}

pub fn back_out(v: f32) -> f32 {
    use std::f32::consts::PI;
    let p = clamp(v);
    let f = 1.0 - p;
    1.0 - (f * f * f - f * (f * PI).sin())
}

pub fn back_in_out(v: f32) -> f32 {
    use std::f32::consts::PI;
    let p = clamp(v);
    if p < 0.5 {
        let f = 2.0 * p;
        0.5 * (f * f * f - f * (f * PI).sin())
    } else {
        let f = 1.0 - (2.0 * p - 1.0);
        0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
    }
}

pub fn bounce_in(v: f32) -> f32 {
    let p = clamp(v);
    1.0 - bounce_out(1.0 - p)
}

pub fn bounce_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 4.0 / 11.0 {
        (121.0 * p * p) / 16.0
    } else if p < 8.0 / 11.0 {
        (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + 17.0 / 5.0
    } else if p < 9.0 / 10.0 {
        (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + 16061.0 / 1805.0
    } else {
        (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + 268.0 / 25.0
    }
}

pub fn bounce_in_out(v: f32) -> f32 {
    let p = clamp(v);
    if p < 0.5 {
        0.5 * bounce_in(p * 2.0)
    } else {
        0.5 * bounce_out(p * 2.0 - 1.0) + 0.5
    }
}
