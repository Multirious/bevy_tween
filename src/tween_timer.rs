//! Module containg implementation of a tween timer
// //!
// //! A tween player is one big part of a tween in this crate.
// //!
// //! [`TweenTimer`] alone do not handles any tweening behavior but instead
// //! delegates it through components and systems like below with system order as
// //! documented in [`TweenSystemSet`]:
// //!  1. Update [`TweenTimer`]'s elasped time
// //!  2. Any tween player implementation updates any [`TweenState`] that
// //!     it responsibles for.
// //!  3. Systems in [`interpolation`] query any [`TweenState`]s and
// //!     output and insert the result in the same entity as [`TweenInterpolationValue`] component.
// //!  4. Systems in [`tween`] query any [`TweenInterpolationValue`] in its entity
// //!     then update its tweening value.
// //! This method of communication with agreed upon components like an interface or
// //! dependency injection I guess, is heavily utilized in this crate for maximum
// //! decoupling and flexbility.
// //!
// //! With [`TweenTimer`], it consist of informations any other specific
// //! tween player implementation may want.
// //! The current elasped time, repeating setting, and repeating behavior is
// //! automatically handled by [`tick_tween_timer_system`]. The average
// //! users may not need to deal with the details of items in this module but
// //! instead the usage in specific tween player implementation.
// //!
// //! [`interpolation`]: crate::interpolation
// //! [`span_tween`]: crate::span_tween
// //! [`tween`]: crate::tween
// //! [`TweenState`]: crate::tween::TweenState
// //! [`TweenInterpolationValue`]: crate::tween::TweenInterpolationValue
// //! [`TweenSystemSet`]: crate::TweenSystemSet

use std::time::Duration;

use bevy::prelude::*;

/// Contains the current elasped time and other useful information
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
    /// Result from ticking is all done.
    AllDone,
}

/// Tween timer
#[derive(Debug, Component, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TweenTimer {
    /// Stop the ticking system from updating this timer.
    pub paused: bool,
    /// The current elasped time with other useful information.
    elasped: Elasped,
    /// When this timer should stop or repeat if configured.
    pub duration_limit: Duration,
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
    pub fn new(duration_limit: Duration) -> TweenTimer {
        TweenTimer {
            duration_limit,
            ..Default::default()
        }
    }

    /// Set the duration limit of this timer
    pub fn set_duration(&mut self, duration: Duration) -> &mut Self {
        self.duration_limit = duration;
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

    /// Returns true if the tween timer all done.
    /// All done meaning that there will be nore more ticking and all
    /// configured repeat is exhausted.
    pub fn is_all_done(&self) -> bool {
        let is_edge = match self.direction {
            AnimationDirection::Forward => {
                self.elasped.now >= self.duration_limit
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

    /// Update the timer by ticking for `duration` in a `direction`.
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
                if self.elasped.now >= self.duration_limit {
                    self.elasped = Elasped {
                        now: self.duration_limit,
                        previous: self.elasped.now,
                        repeat_style: None,
                    };
                    return TickResult::AllDone;
                }
                let new_now =
                    (self.elasped.now + duration).min(self.duration_limit);
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.now,
                    repeat_style: None,
                };
                TickResult::Continue
            }
            (Backward, None, _) => {
                if self.elasped.now == Duration::ZERO {
                    self.elasped = Elasped {
                        now: Duration::ZERO,
                        previous: self.elasped.now,
                        repeat_style: None,
                    };
                    return TickResult::AllDone;
                }
                let new_now = self.elasped.now.saturating_sub(duration);
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.now,
                    repeat_style: None,
                };
                TickResult::Continue
            }
            (Forward, Some(mut r), WrapAround) => {
                let new_now = self.elasped.now + duration;
                let will_wrap = new_now >= self.duration_limit;
                if will_wrap && !r.try_advance_counter() {
                    self.elasped = Elasped {
                        now: self.duration_limit,
                        previous: self.elasped.now,
                        repeat_style: None,
                    };
                    return TickResult::AllDone;
                }
                let new_now = duration_rem(new_now, self.duration_limit);
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.now,
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
                        previous: self.elasped.now,
                        repeat_style: None,
                    };
                    return TickResult::AllDone;
                }
                let new_now = if will_wrap {
                    neg_duration_rem(
                        duration - self.elasped.now,
                        self.duration_limit,
                    )
                } else {
                    self.elasped.now - duration
                };
                self.elasped = Elasped {
                    now: new_now,
                    previous: self.elasped.now,
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
                let will_pingpong = new_now > self.duration_limit;
                if will_pingpong {
                    if !r.try_advance_counter() {
                        self.elasped = Elasped {
                            now: self.duration_limit,
                            previous: self.elasped.previous,
                            repeat_style: None,
                        };
                        return TickResult::AllDone;
                    }
                    let new_now =
                        neg_duration_rem(new_now, self.duration_limit);
                    self.direction = Backward;
                    self.elasped = Elasped {
                        now: new_now,
                        previous: self.elasped.now,
                        repeat_style: Some(PingPong),
                    };
                    TickResult::Repeated
                } else {
                    self.elasped = Elasped {
                        now: new_now,
                        previous: self.elasped.now,
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
                        return TickResult::AllDone;
                    }
                    let new_now = duration_rem(
                        duration - self.elasped.now,
                        self.duration_limit,
                    );
                    self.direction = Forward;
                    self.elasped = Elasped {
                        now: new_now,
                        previous: self.elasped.now,
                        repeat_style: Some(PingPong),
                    };
                    TickResult::Repeated
                } else {
                    self.elasped = Elasped {
                        now: self.elasped.now - duration,
                        previous: self.elasped.now,
                        repeat_style: None,
                    };
                    TickResult::Continue
                }
            }
        }
    }
}

impl Default for TweenTimer {
    fn default() -> Self {
        TweenTimer {
            paused: Default::default(),
            elasped: Default::default(),
            duration_limit: Default::default(),
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
    /// Repeat infinitely and count the times this tween has repeated
    InfinitelyCounted {
        #[allow(missing_docs)]
        times_repeated: usize,
    },
    /// Repeat for this amount of times
    Times {
        #[allow(missing_docs)]
        times: usize,
        #[allow(missing_docs)]
        times_repeated: usize,
    },
}

impl Repeat {
    /// Repeat infinitely
    pub fn infinitely() -> Repeat {
        Repeat::Infinitely
    }

    /// Repeat infinitely and count the times this tween has repeated
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

/// Event that emitted when a tween timer just ended. This will be emitted for
/// the one that just repeated as well.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event, Reflect)]
pub struct TweenTimerEnded {
    /// Tween timer that just ended
    pub timer: Entity,
    /// Currently direction. If is [`RepeatStyle::PingPong`], the current direction
    /// will be its already changed direction.
    pub current_direction: AnimationDirection,
    /// The repeat this tween timer had.
    pub with_repeat: Option<Repeat>,
}

impl TweenTimerEnded {
    /// Returns true if the tween timer all done.
    /// All done meaning that there will be nore more ticking and all
    /// configured repeat is exhausted.
    pub fn is_all_done(&self) -> bool {
        self.with_repeat
            .map(|repeat| repeat.exhausted())
            .unwrap_or(true)
    }
}

/// Updates any [`TweenTimer`] elasped time and handles the repeat if configured.
pub fn tick_tween_timer_system(
    time: Res<Time<Real>>,
    mut q_timer: Query<(Entity, &mut TweenTimer)>,
    mut ended_writer: EventWriter<TweenTimerEnded>,
) {
    let delta = time.delta();
    q_timer.iter_mut().for_each(|(entity, mut timer)| {
        if timer.paused {
            return;
        }

        let is_prev_all_done = timer.is_all_done();
        if is_prev_all_done {
            return;
        }

        let delta = Duration::from_secs_f32(
            delta.as_secs_f32() * timer.speed_scale.as_secs_f32(),
        );

        let timer_direction = timer.direction;
        let tick_result = timer.tick(delta, timer_direction);

        match tick_result {
            TickResult::AllDone | TickResult::Repeated => {
                ended_writer.send(TweenTimerEnded {
                    timer: entity,
                    current_direction: timer.direction,
                    with_repeat: timer.repeat,
                });
            }
            TickResult::Continue => {}
        }
    })
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
