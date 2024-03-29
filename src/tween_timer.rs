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
/// This is better for handling with edge cases and retain timing accuracy.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Elasped {
    /// The current elasped time
    pub now: Duration,
    /// The previous elasped time
    pub previous: Duration,
    /// Some if the tween timer just ended and repeated in some way.
    pub repeat_style: Option<RepeatStyle>,
}

impl Elasped {
    /// Create new [`Elasped`]
    pub fn new(elasped: Duration) -> Elasped {
        Elasped {
            now: elasped,
            previous: elasped,
            repeat_style: None,
        }
    }
}

/// Report the result after using the method [`TweenTimer::tick`]
pub enum TickResult {
    /// Result from ticking is normal.
    Continue,
    /// Result from ticking is timer has repeated.
    Repeated,
    /// Result from ticking is completed.
    Completed,
}

/// Tween timer
#[derive(Debug, Component, Clone, PartialEq, Eq, Hash, Reflect)]
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
    pub speed_scale: Duration,
    /// Configure to repeat.
    pub repeat: Option<Repeat>,
    /// Configure to repeat with a style.
    pub repeat_style: Option<RepeatStyle>,
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

    // pub fn set_elasped(&mut self, elasped: Duration) -> &mut Self {
    //     self.elasped.now = elasped;
    //     // self.elasped.now = elasped;
    //     self
    // }

    /// Set direction
    pub fn set_direction(
        &mut self,
        direction: AnimationDirection,
    ) -> &mut Self {
        self.direction = direction;
        self
    }

    /// Set repeat
    pub fn set_repeat(&mut self, repeat: Option<Repeat>) -> &mut Self {
        self.repeat = repeat;
        self
    }

    /// Set repeat style
    pub fn set_repeat_style(
        &mut self,
        repeat_style: Option<RepeatStyle>,
    ) -> &mut Self {
        self.repeat_style = repeat_style;
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
        let is_edge = match self.direction {
            AnimationDirection::Forward => {
                self.elasped.now >= self.length
                    && self.elasped.now == self.elasped.previous
            }
            AnimationDirection::Backward => {
                self.elasped.now == Duration::ZERO
                    && self.elasped.now == self.elasped.previous
            }
        };
        match self.repeat {
            Some(repeat) => repeat.exhausted() && is_edge,
            None => is_edge,
        }
    }

    /// Update the `now` in [`Elasped`] for `duration` in a `direction`.
    pub fn tick(
        &mut self,
        duration: Duration,
        direction: AnimationDirection,
    ) -> TickResult {
        use AnimationDirection::*;
        use RepeatStyle::*;
        match (
            direction,
            self.repeat,
            self.repeat_style.unwrap_or_default(),
        ) {
            (Forward, None, _) => {
                if self.elasped.now >= self.length {
                    self.elasped = Elasped {
                        now: self.length,
                        previous: self.elasped.previous,
                        repeat_style: None,
                    };
                    return TickResult::Completed;
                }
                let new_now = (self.elasped.now + duration).min(self.length);
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.previous,
                    repeat_style: None,
                };
                TickResult::Continue
            }
            (Backward, None, _) => {
                if self.elasped.now == Duration::ZERO {
                    self.elasped = Elasped {
                        now: Duration::ZERO,
                        previous: self.elasped.previous,
                        repeat_style: None,
                    };
                    return TickResult::Completed;
                }
                let new_now = self.elasped.now.saturating_sub(duration);
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.previous,
                    repeat_style: None,
                };
                TickResult::Continue
            }
            (Forward, Some(mut r), WrapAround) => {
                let new_now = self.elasped.now + duration;
                let will_wrap = new_now >= self.length;
                if will_wrap && !r.try_advance_counter() {
                    self.elasped = Elasped {
                        now: self.length,
                        previous: self.elasped.previous,
                        repeat_style: None,
                    };
                    return TickResult::Completed;
                }
                let new_now = duration_rem(new_now, self.length);
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.previous,
                    repeat_style: if will_wrap {
                        Some(WrapAround)
                    } else {
                        None
                    },
                };
                if will_wrap {
                    TickResult::Repeated
                } else {
                    TickResult::Continue
                }
            }
            (Backward, Some(mut r), WrapAround) => {
                let will_wrap = duration > self.elasped.now;
                if will_wrap && !r.try_advance_counter() {
                    self.elasped = Elasped {
                        now: Duration::ZERO,
                        previous: self.elasped.previous,
                        repeat_style: None,
                    };
                    return TickResult::Completed;
                }
                let new_now = if will_wrap {
                    neg_duration_rem(duration - self.elasped.now, self.length)
                } else {
                    self.elasped.now - duration
                };
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.previous,
                    repeat_style: if will_wrap {
                        Some(WrapAround)
                    } else {
                        None
                    },
                };
                if will_wrap {
                    TickResult::Repeated
                } else {
                    TickResult::Continue
                }
            }
            (Forward, Some(mut r), PingPong) => {
                let new_now = self.elasped.now + duration;
                let will_pingpong = new_now > self.length;
                if will_pingpong {
                    if !r.try_advance_counter() {
                        self.elasped = Elasped {
                            now: self.length,
                            previous: self.elasped.previous,
                            repeat_style: None,
                        };
                        return TickResult::Completed;
                    }
                    let new_now = neg_duration_rem(new_now, self.length);
                    self.direction = Backward;
                    self.elasped = Elasped {
                        now: new_now,
                        previous: self.elasped.previous,
                        repeat_style: Some(PingPong),
                    };
                    TickResult::Repeated
                } else {
                    self.elasped = Elasped {
                        now: new_now,
                        previous: self.elasped.previous,
                        repeat_style: None,
                    };
                    TickResult::Continue
                }
            }
            (Backward, Some(mut r), PingPong) => {
                let will_pingpong = duration > self.elasped.now;
                if will_pingpong {
                    if !r.try_advance_counter() {
                        self.elasped = Elasped {
                            now: Duration::ZERO,
                            previous: self.elasped.previous,
                            repeat_style: None,
                        };
                        return TickResult::Completed;
                    }
                    let new_now =
                        duration_rem(duration - self.elasped.now, self.length);
                    self.direction = Forward;
                    self.elasped = Elasped {
                        now: new_now,
                        previous: self.elasped.previous,
                        repeat_style: Some(PingPong),
                    };
                    TickResult::Repeated
                } else {
                    self.elasped = Elasped {
                        now: self.elasped.now - duration,
                        previous: self.elasped.previous,
                        repeat_style: None,
                    };
                    TickResult::Continue
                }
            }
        }
    }

    /// Set currently elasped now to `duration`.
    pub fn set_tick(&mut self, duration: Duration) {
        self.elasped.now = duration;
        self.elasped.repeat_style = None;
    }

    /// Update the `previous` in [`Elasped`] to `now` and set `repeat_style` to
    /// None. Only call if the currently elasped has been correctly apply
    /// and doesn't need to be accounted for anymore.
    pub fn collaspe_elasped(&mut self) {
        self.elasped.previous = self.elasped.now;
        self.elasped.repeat_style = None;
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
            repeat_style: Default::default(),
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
        times_repeated: usize,
    },
    /// Repeat for this amount of times
    Times {
        /// Times to repeat for
        #[allow(missing_docs)]
        times: usize,
        /// Times this timer has repeated.
        #[allow(missing_docs)]
        times_repeated: usize,
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
    pub fn times(times: usize) -> Repeat {
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
    pub fn try_advance_counter(&mut self) -> bool {
        match self {
            Repeat::Infinitely => {}
            Repeat::InfinitelyCounted { times_repeated } => {
                *times_repeated += 1;
            }
            Repeat::Times {
                times,
                times_repeated,
            } => {
                if times_repeated >= times {
                    return false;
                }
                *times_repeated += 1;
            }
        }
        true
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

fn duration_rem(duration: Duration, max: Duration) -> Duration {
    let duration = duration.as_secs_f32();
    let max = max.as_secs_f32();
    let output = duration % max;
    Duration::from_secs_f32(output)
}

fn neg_duration_rem(neg_duration: Duration, max: Duration) -> Duration {
    let neg_duration = -neg_duration.as_secs_f32();
    let max = max.as_secs_f32();
    let output = neg_duration.rem_euclid(max);
    Duration::from_secs_f32(output)
}
