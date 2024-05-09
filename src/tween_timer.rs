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

#[deprecated(
    since = "0.5.0",
    note = "Use `bevy_time_runner::TimeDirection` instead"
)]
pub use bevy_time_runner::TimeDirection as AnimationDirection;

#[deprecated(
    since = "0.5.0",
    note = "Use `bevy_time_runner::Repeat` instead"
)]
pub use bevy_time_runner::Repeat;
#[deprecated(
    since = "0.5.0",
    note = "Use `bevy_time_runner::RepeatStyle` instead"
)]
pub use bevy_time_runner::RepeatStyle;

/// Contains the current elasped time per tick.
/// Have more informations useful for handling edge cases and retain timing accuracy.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct Elasped {
    /// The current elasped seconds. Always within timer's length.
    pub now: f32,
    /// Value between 0–1 as percentage of the timer period.
    /// Value may goes over or under 0–1 to indicate looping or repeating in
    /// arbitary times.
    pub now_period: f32,
    /// The previous elasped seconds. Always within timer's length.
    pub previous: f32,
    /// Previous value between 0–1 as percentage of the timer period.
    /// Value may goes over or under 0–1 to indicate looping or repeating in
    /// arbitary times.
    pub previous_period: f32,
}

impl Elasped {
    fn update(&mut self, now: f32, now_period: f32) {
        self.previous = self.now;
        self.previous_period = self.now_period;
        self.now = now;
        self.now_period = now_period;
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
                self.elasped.now_period >= 1.0
                    && self.elasped.now_period == self.elasped.previous_period
            }
            AnimationDirection::Backward => {
                self.elasped.now_period <= 0.0
                    && self.elasped.now == self.elasped.previous
            }
        };
        match self.repeat {
            Some((repeat, _)) => repeat.exhausted() && at_edge,
            None => at_edge,
        }
    }

    /// Update  [`Elasped`] for `secs`.
    ///
    /// # Panics
    ///
    /// Panics if `secs` is Nan.
    pub fn tick(&mut self, secs: f32) {
        use AnimationDirection::*;
        use RepeatStyle::*;

        assert!(!secs.is_nan(), "Tick seconds can't be Nan");

        let length = self.length.as_secs_f32();
        let now = self.elasped.now;

        let new_elasped = match self.direction {
            Forward => now + secs,
            Backward => now - secs,
        };

        let p = period_percentage(new_elasped, length);

        let repeat_count = p.floor() as i32;
        let repeat_style = 'a: {
            if let Some(r) = self.repeat.as_mut() {
                if repeat_count != 0 {
                    let repeat_count =
                        if self.direction == AnimationDirection::Forward {
                            repeat_count
                        } else {
                            -repeat_count
                        };
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

        let new_elasped = match repeat_style {
            WrapAround => saw_wave(new_elasped, length),
            PingPong => triangle_wave(new_elasped, length),
        };
        self.elasped.update(new_elasped, p);

        if repeat_style == RepeatStyle::PingPong {
            let new_direction = match self.direction {
                Forward => triangle_wave_direction(repeat_count),
                Backward => backward_triangle_wave_direction(repeat_count),
            };
            self.direction = new_direction;
        }
    }

    /// Set currently elasped now to `duration`.
    pub fn set_tick(&mut self, secs: f32) {
        self.elasped.now = secs;
        self.elasped.now_period =
            period_percentage(secs, self.length.as_secs_f32());
    }

    /// Update the `previous` in [`Elasped`] to `now` and set `repeat_style` to
    /// None. Only call if the current range of elasped has been handled.
    pub fn collaspe_elasped(&mut self) {
        self.elasped.previous = self.elasped.now;
        self.elasped.previous_period = self.elasped.now_period;
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

fn saw_wave(x: f32, period: f32) -> f32 {
    x.rem_euclid(period)
}

fn triangle_wave(x: f32, period: f32) -> f32 {
    ((x + period).rem_euclid(period * 2.) - period).abs()
}

fn triangle_wave_direction(repeats: i32) -> AnimationDirection {
    if repeats.rem_euclid(2) == 0 {
        AnimationDirection::Forward
    } else {
        AnimationDirection::Backward
    }
}

fn backward_triangle_wave_direction(repeats: i32) -> AnimationDirection {
    if repeats.rem_euclid(2) == 0 {
        AnimationDirection::Backward
    } else {
        AnimationDirection::Forward
    }
}

fn period_percentage(x: f32, period: f32) -> f32 {
    x / period
}

#[cfg(test)]
mod test {
    use super::*;

    fn secs(secs: f32) -> Duration {
        Duration::from_secs_f32(secs)
    }

    // fn eq(lhs: f32, rhs: f32) -> bool {
    //     (lhs - rhs).abs() <= f32::EPSILON
    // }

    #[test]
    fn timer() {
        let mut timer = TweenTimer::new(secs(5.));

        timer.tick(2.5);
        assert_eq!(timer.elasped.now, 2.5);
        assert_eq!(timer.elasped.now_period, 0.5);

        timer.tick(2.5);
        assert_eq!(timer.elasped.now, 5.);
        assert_eq!(timer.elasped.now_period, 1.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 5.);
        assert_eq!(timer.elasped.now_period, 1.);

        timer.set_tick(0.);

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 3.);
        assert_eq!(timer.elasped.now_period, 3. / 5.);

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 5.);
        assert_eq!(timer.elasped.now_period, 1.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 5.);
        assert_eq!(timer.elasped.now_period, 1.);
    }

    #[test]
    fn timer_backward() {
        let mut timer = TweenTimer::new(secs(5.));
        timer.set_direction(AnimationDirection::Backward);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 0.);
        assert_eq!(timer.elasped.now_period, 0.);

        timer.set_tick(5.);

        timer.tick(2.5);
        assert_eq!(timer.elasped.now, 2.5);
        assert_eq!(timer.elasped.now_period, 0.5);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 1.5);
        assert_eq!(timer.elasped.now_period, 1.5 / 5.);

        timer.tick(2.);
        assert_eq!(timer.elasped.now, 0.);
        assert_eq!(timer.elasped.now_period, 0.);
    }

    #[test]
    fn timer_wrap_around() {
        let mut timer = TweenTimer::new(secs(5.));
        timer.set_repeat(Some((Repeat::Infinitely, RepeatStyle::WrapAround)));

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 1.);
        assert_eq!(timer.elasped.now_period, 1. / 5.);

        timer.tick(2.5);
        assert_eq!(timer.elasped.now, 3.5);
        assert_eq!(timer.elasped.now_period, 3.5 / 5.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 4.5);
        assert_eq!(timer.elasped.now_period, 4.5 / 5.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 0.5);
        assert_eq!(timer.elasped.now_period, 5.5 / 5.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 1.5);
        assert_eq!(timer.elasped.now_period, 1.5 / 5.);

        timer.tick(3.5);
        assert_eq!(timer.elasped.now, 0.);
        assert_eq!(timer.elasped.now_period, 5. / 5.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 1.);
        assert_eq!(timer.elasped.now_period, 1. / 5.);
    }

    #[test]
    fn timer_backward_wrap_around() {
        let mut timer = TweenTimer::new(secs(5.));
        timer
            .set_repeat(Some((Repeat::Infinitely, RepeatStyle::WrapAround)))
            .set_direction(AnimationDirection::Backward);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 4.);
        assert_eq!(timer.elasped.now_period, -1. / 5.);

        timer.tick(2.5);
        assert_eq!(timer.elasped.now, 1.5);
        assert_eq!(timer.elasped.now_period, 1.5 / 5.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 0.5);
        assert_eq!(timer.elasped.now_period, 0.5 / 5.);

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 4.5);
        assert_eq!(timer.elasped.now_period, -0.5 / 5.);
    }

    #[test]
    fn timer_wrap_around_times() {
        let mut timer = TweenTimer::new(secs(5.));
        timer.set_repeat(Some((Repeat::times(2), RepeatStyle::WrapAround)));

        timer.tick(4.);
        assert_eq!(timer.elasped.now, 4.);
        assert_eq!(timer.elasped.now_period, 4. / 5.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 0
            },
        );

        timer.tick(4.);
        assert_eq!(timer.elasped.now, 3.);
        assert_eq!(timer.elasped.now_period, 8. / 5.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 1
            },
        );

        timer.tick(4.);
        assert_eq!(timer.elasped.now, 2.);
        assert_eq!(timer.elasped.now_period, 7. / 5.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 2
            },
        );

        timer.tick(4.);
        assert_eq!(timer.elasped.now, 5.);
        assert_eq!(timer.elasped.now_period, 1.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 2
            },
        );

        timer.tick(1.);
        assert_eq!(timer.elasped.now, 5.);
        assert_eq!(timer.elasped.now_period, 1.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 2
            },
        );
    }

    #[test]
    fn timer_backward_wrap_around_times() {
        let mut timer = TweenTimer::new(secs(5.));
        timer
            .set_repeat(Some((Repeat::times(2), RepeatStyle::WrapAround)))
            .set_direction(AnimationDirection::Backward);

        timer.tick(4.);
        assert_eq!(timer.elasped.now, 1.);
        assert_eq!(timer.elasped.now_period, -4. / 5.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 1
            },
        );

        timer.tick(4.);
        assert_eq!(timer.elasped.now, 2.);
        assert_eq!(timer.elasped.now_period, -3. / 5.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 2
            },
        );

        timer.tick(4.);
        assert_eq!(timer.elasped.now, 0.);
        assert_eq!(timer.elasped.now_period, 0. / 5.);
        assert_eq!(
            timer.repeat.unwrap().0,
            Repeat::Times {
                times: 2,
                times_repeated: 2
            },
        );
    }

    #[test]
    fn timer_ping_pong() {
        let mut timer = TweenTimer::new(secs(5.));
        timer.set_repeat(Some((Repeat::Infinitely, RepeatStyle::PingPong)));

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 3.);
        assert_eq!(timer.elasped.now_period, 3. / 5.);
        assert_eq!(timer.direction, AnimationDirection::Forward);

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 4.);
        assert_eq!(timer.elasped.now_period, 6. / 5.);
        assert_eq!(timer.direction, AnimationDirection::Backward);

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 1.);
        assert_eq!(timer.elasped.now_period, 1. / 5.);
        assert_eq!(timer.direction, AnimationDirection::Backward);

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 2.);
        assert_eq!(timer.elasped.now_period, -2. / 5.);
        assert_eq!(timer.direction, AnimationDirection::Forward);

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 5.);
        assert_eq!(timer.elasped.now_period, 5. / 5.);
        assert_eq!(timer.direction, AnimationDirection::Backward);

        timer.tick(3.);
        assert_eq!(timer.elasped.now, 2.);
        assert_eq!(timer.elasped.now_period, 2. / 5.);
        assert_eq!(timer.direction, AnimationDirection::Backward);
    }
}
