//! Module containing implementation of a tween timer
//!
//! # [`TweenTimer`]
//! [`TweenTimer`] is a more advanced version of [`Timer`]
//!
//! Features:
//! - Backward and forward ticking direction handling.
//! - Customize repeat behavior with [`Repeat`] and [`RepeatStyle`].
//! - Customizable ticking speed.

use std::time::Duration;

use bevy::prelude::*;

/// Contains the current elasped time and other useful information.
/// This is better for handling with edge cases and retain timing accuracy per frame.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct Elasped {
    /// The current elasped seconds
    pub now: f32,
    /// The current percentage of the timer length.
    pub now_percentage: f32,
    /// The previous elasped seconds
    pub previous: f32,
    /// The previous percentage of the timer length.
    pub previous_percentage: f32,
}

impl Elasped {
    fn update(&mut self, now: f32, now_percentage: f32) {
        self.previous = self.now;
        self.previous_percentage = self.now_percentage;
        self.now = now;
        self.now_percentage = now_percentage;
    }
}

/// Tween timer
#[derive(Debug, Component, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct TweenTimer {
    /// Stop the ticking system from updating this timer.
    pub paused: bool,
    /// The current elasped time with other useful information.
    elasped: Elasped,
    /// Maximum amount of duration.
    pub length: Duration,
    /// Ticking direction of the current timer.
    pub direction: AnimationDirection,
    /// Set speed of the playback to `speed_scale` second per second.
    /// This *is not* applied automatically by [Self::tick] but instead by specifc
    /// tweener player implementation
    pub speed_scale: Duration,
    /// Repeat configuration.
    pub repeat: Option<(Repeat, RepeatStyle)>,
}

impl TweenTimer {
    /// Create new [`TweenTimer`] with this duration.
    pub fn new(length: Duration) -> TweenTimer {
        TweenTimer {
            length,
            ..Default::default()
        }
    }

    /// Set the duration limit of this timer
    pub fn set_length(&mut self, duration: Duration) -> &mut Self {
        self.length = duration;
        self
    }

    /// Set paused
    pub fn set_paused(&mut self, paused: bool) -> &mut Self {
        self.paused = paused;
        self
    }

    /// Set direction
    pub fn set_direction(
        &mut self,
        direction: AnimationDirection,
    ) -> &mut Self {
        self.direction = direction;
        self
    }

    /// Set repeat
    pub fn set_repeat(
        &mut self,
        repeat: Option<(Repeat, RepeatStyle)>,
    ) -> &mut Self {
        self.repeat = repeat;
        self
    }

    /// Get current elasped
    pub fn elasped(&self) -> Elasped {
        self.elasped
    }

    /// Returns true if the tween timer completed.
    /// Completed meaning that there will be no more ticking and all
    /// configured repeat is exhausted.
    pub fn is_completed(&self) -> bool {
        let at_edge = match self.direction {
            AnimationDirection::Forward => {
                self.elasped.now_percentage >= 1.0
                    && self.elasped.now_percentage
                        == self.elasped.previous_percentage
            }
            AnimationDirection::Backward => {
                self.elasped.now_percentage <= 0.0
                    && self.elasped.now == self.elasped.previous
            }
        };
        match self.repeat {
            Some((repeat, _)) => repeat.exhausted() && at_edge,
            None => at_edge,
        }
    }

    /// Update  [`Elasped`] for `secs`.
    pub fn tick(&mut self, secs: f32) {
        use AnimationDirection::*;
        use RepeatStyle::*;

        let length = self.length.as_secs_f32();
        let now = self.elasped.now;

        assert!(!secs.is_nan(), "Tick seconds can't be Nan");

        if secs == 0. {
            self.elasped
                .update(self.elasped.now, self.elasped.now_percentage);
            return;
        };

        let new_elasped = match self.direction {
            Forward => now + secs,
            Backward => now - secs,
        };

        let p = period_percentage(new_elasped, length);

        let repeat_count = p.floor() as i32;
        let repeat_style = 'a: {
            if let Some(r) = self.repeat.as_mut() {
                if repeat_count != 0 {
                    let advances = r.0.advance_counter_by(repeat_count);
                    if advances != 0 {
                        break 'a r.1;
                    }
                }
            }
            if new_elasped > length {
                self.elasped.update(length, 1.);
            } else if new_elasped < 0. {
                self.elasped.update(0., 0.);
            } else {
                self.elasped.update(new_elasped, p);
            };
            return;
        };

        let (new_elasped, new_direction) = match (self.direction, repeat_style)
        {
            (Forward, WrapAround) => {
                (slope_up_saw_wave(new_elasped, length), Forward)
            }
            (Backward, WrapAround) => {
                (slope_down_saw_wave(new_elasped, length), Backward)
            }
            (Forward, PingPong) => (
                slope_up_triangle_wave(new_elasped, length),
                slope_up_triangle_wave_direction(repeat_count),
            ),
            (Backward, PingPong) => (
                // ?? why in the hell this works
                slope_up_triangle_wave(new_elasped, length),
                slope_down_triangle_wave_direction(repeat_count),
            ),
        };
        self.elasped.update(new_elasped, p);
        self.direction = new_direction;
    }

    /// Set currently elasped now to `duration`.
    pub fn set_tick(&mut self, secs: f32) {
        self.elasped.now = secs;
        self.elasped.now_percentage =
            period_percentage(secs, self.length.as_secs_f32());
    }

    /// Update the `previous` in [`Elasped`] to `now` and set `repeat_style` to
    /// None. Only call if the currently elasped has been correctly apply
    /// and doesn't need to be accounted for anymore.
    pub fn collaspe_elasped(&mut self) {
        self.elasped.previous = self.elasped.now;
        self.elasped.previous_percentage = self.elasped.now_percentage;
    }
}

impl Default for TweenTimer {
    fn default() -> Self {
        TweenTimer {
            paused: Default::default(),
            elasped: Default::default(),
            length: Default::default(),
            direction: Default::default(),
            speed_scale: Duration::from_secs(1),
            repeat: Default::default(),
        }
    }
}

/// Repeat the tween
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Repeat {
    /// Repeat infinitely
    Infinitely,
    /// Repeat infinitely and count the times this timer has repeated
    InfinitelyCounted {
        /// The times this timer has repeated
        times_repeated: i32,
    },
    /// Repeat for this amount of times
    Times {
        /// Times to repeat for
        #[allow(missing_docs)]
        times: i32,
        /// Times this timer has repeated.
        #[allow(missing_docs)]
        times_repeated: i32,
    },
}

impl Repeat {
    /// Repeat infinitely
    pub fn infinitely() -> Repeat {
        Repeat::Infinitely
    }

    /// Repeat infinitely and count the times this timer has repeated
    pub fn infinitely_counted() -> Repeat {
        Repeat::InfinitelyCounted { times_repeated: 0 }
    }

    /// Repeat for this amount of times
    pub fn times(times: i32) -> Repeat {
        Repeat::Times {
            times,
            times_repeated: 0,
        }
    }

    /// Returns if all repeat has been exhausted.
    /// Infinite repeat always returns false.
    pub fn exhausted(&self) -> bool {
        match self {
            Repeat::Infinitely => false,
            Repeat::InfinitelyCounted { .. } => false,
            Repeat::Times {
                times,
                times_repeated,
            } => times_repeated >= times,
        }
    }

    /// true if still can repeat, false otherwise.
    #[deprecated(
        since = "0.3.0",
        note = "Use `advance_counter_by(1) == 1` instead"
    )]
    pub fn try_advance_counter(&mut self) -> bool {
        self.advance_counter_by(1) == 1
    }

    /// Returns actual advanced count.
    pub fn advance_counter_by(&mut self, by: i32) -> i32 {
        match self {
            Repeat::Infinitely => by,
            Repeat::InfinitelyCounted { times_repeated } => {
                *times_repeated += by;
                by
            }
            Repeat::Times {
                times,
                times_repeated,
            } => {
                let times_left = *times - *times_repeated;
                if times_left == 0 {
                    return 0;
                }
                let times_to_advance =
                    if times_left > by { by } else { times_left };
                *times_repeated += times_to_advance;
                times_to_advance
            }
        }
    }
}

/// Tween timer repeat behavior
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RepeatStyle {
    /// Timer will wrap around.
    #[default]
    WrapAround,
    /// Timer will flip its direction.
    PingPong,
}

/// Specfy which way the tween timer is ticking
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AnimationDirection {
    /// Playing forward
    #[default]
    Forward,
    /// Playing backward
    Backward,
}

fn slope_up_saw_wave(x: f32, period: f32) -> f32 {
    x.rem_euclid(period)
}

fn slope_down_saw_wave(x: f32, period: f32) -> f32 {
    (-x).rem_euclid(period)
}

fn slope_up_triangle_wave(x: f32, period: f32) -> f32 {
    ((x + period).rem_euclid(period * 2.) - period).abs()
}

// fn slope_down_triangle_wave(x: f32, period: f32) -> f32 {
//     (x.rem_euclid(period * 2.) - period).abs()
// }

fn slope_up_triangle_wave_direction(repeats: i32) -> AnimationDirection {
    if repeats.rem_euclid(2) == 0 {
        AnimationDirection::Forward
    } else {
        AnimationDirection::Backward
    }
}

fn slope_down_triangle_wave_direction(repeats: i32) -> AnimationDirection {
    if repeats.rem_euclid(2) == 0 {
        AnimationDirection::Backward
    } else {
        AnimationDirection::Forward
    }
}

fn period_percentage(x: f32, period: f32) -> f32 {
    x / period
}
