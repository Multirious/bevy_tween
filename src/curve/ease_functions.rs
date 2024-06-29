use std::f32::consts::PI;
fn clamp(p: f32) -> f32 {
    match () {
        _ if p > 1.0 => 1.0,
        _ if p < 0.0 => 0.0,
        _ => p,
    }
}

pub fn linear(v: f32) -> f32 {
    clamp(v)
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
    1. - (p * PI * 0.5).cos()
}

pub fn sine_out(v: f32) -> f32 {
    let p = clamp(v);
    (p * PI * 0.5).sin()
}

pub fn sine_in_out(v: f32) -> f32 {
    let p = clamp(v);
    -((p * PI).cos() - 1.) * 0.5
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
    let p = clamp(v);
    p * p * p - p * (p * PI).sin()
}

pub fn back_out(v: f32) -> f32 {
    let p = clamp(v);
    let f = 1.0 - p;
    1.0 - (f * f * f - f * (f * PI).sin())
}

pub fn back_in_out(v: f32) -> f32 {
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
