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

use bevy::prelude::*;
use std::time::Duration;

/// Contains the current elasped time and other useful information
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Elasped {
    /// The current elasped time
    pub now: Duration,
    /// The previous elasped time
    pub previous: Duration,
    /// [`RepeatStyle`] if the tween player just ended and repeated in some way.
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

/// State of a tween player, animation direction, and repeat configuration
#[derive(Debug, Component, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TweenTimer {
    /// Stop the ticking system from updating this player.
    pub paused: bool,
    /// The current elasped time with other useful information.
    elasped: Elasped,
    /// When this player should stop or repeat if configured.
    pub duration_limit: Duration,
    /// Playback direction of the current player.
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
    /// Set the duration limit of this player
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

    /// Returns true if the tween player is actually finished for real,
    /// accounting its repeat configuration.
    pub fn is_finished(&self) -> bool {
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
            Some(repeat) => repeat.is_finished() && is_edge,
            None => is_edge,
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
    pub fn is_finished(&self) -> bool {
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

/// Tween repeat behavior
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RepeatStyle {
    /// When the direction is forward is reached the duration limit:
    ///  - go to second 0
    /// When the direction is backward is reached second 0:
    ///  - go to duration limit
    ///
    /// Basically integer wrap around, you get it.
    #[default]
    WrapAround,
    /// When the direction is forward is reached the duration limit:
    ///  - Change the direction to backward
    /// When the direction is backward is reached second 0:
    ///  - Change the direction to forward
    PingPong,
}

/// Specfy which way the tween player is playing
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AnimationDirection {
    /// Playing forward
    #[default]
    Forward,
    /// Playing backward
    Backward,
}

/// Event that emitted when a tween player just ended. This will be emitted for
/// the one that just repeated as well.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event, Reflect)]
pub struct TweenTimerEnded {
    /// Tween timer that just ended
    pub timer: Entity,
    /// Currently direction. If is [`RepeatStyle::PingPong`], the current direction
    /// will be its already changed direction.
    pub current_direction: AnimationDirection,
    /// The repeat this tween player had.
    pub with_repeat: Option<Repeat>,
}

impl TweenTimerEnded {
    /// Returns true if the tween player is actually finished for real,
    /// accounting its repeat configuration.
    pub fn is_finished(&self) -> bool {
        self.with_repeat
            .map(|repeat| repeat.is_finished())
            .unwrap_or(true)
    }
}

/// Updates any [`TweenTimer`] elasped time and handles the repeat if configured.
pub fn tick_tween_timer_system(
    time: Res<Time<Real>>,
    mut q_timer: Query<(Entity, &mut TweenTimer)>,
    mut ended_writer: EventWriter<TweenTimerEnded>,
) {
    use AnimationDirection::*;
    use RepeatStyle::*;
    let delta = time.delta();
    q_timer.iter_mut().for_each(|(entity, mut timer)| {
        if timer.paused {
            return;
        }

        let is_prev_finished = timer.is_finished();
        if is_prev_finished {
            return;
        }

        let delta = Duration::from_secs_f32(
            delta.as_secs_f32() * timer.speed_scale.as_secs_f32(),
        );

        let (is_now_finished, is_repeated) = 'm: {
            match (
                timer.direction,
                timer.repeat,
                timer.repeat_style.unwrap_or_default(),
            ) {
                (Forward, None, _) => {
                    if timer.elasped.now >= timer.duration_limit {
                        timer.elasped = Elasped {
                            now: timer.duration_limit,
                            previous: timer.elasped.now,
                            repeat_style: None,
                        };
                        break 'm (true, false);
                    }
                    let new_now =
                        (timer.elasped.now + delta).min(timer.duration_limit);
                    timer.elasped = Elasped {
                        now: new_now,
                        previous: timer.elasped.now,
                        repeat_style: None,
                    };
                    (false, false)
                }
                (Backward, None, _) => {
                    if timer.elasped.now == Duration::ZERO {
                        timer.elasped = Elasped {
                            now: Duration::ZERO,
                            previous: timer.elasped.now,
                            repeat_style: None,
                        };
                        break 'm (true, false);
                    }
                    let new_now = timer.elasped.now.saturating_sub(delta);
                    timer.elasped = Elasped {
                        now: new_now,
                        previous: timer.elasped.now,
                        repeat_style: None,
                    };
                    (false, false)
                }
                (Forward, Some(mut r), WrapAround) => {
                    let new_now = timer.elasped.now + delta;
                    let will_wrap = new_now >= timer.duration_limit;
                    if will_wrap && !r.try_advance_counter() {
                        timer.elasped = Elasped {
                            now: timer.duration_limit,
                            previous: timer.elasped.now,
                            repeat_style: None,
                        };
                        break 'm (true, false);
                    }
                    let new_now = duration_rem(new_now, timer.duration_limit);
                    timer.elasped = Elasped {
                        now: new_now,
                        previous: timer.elasped.now,
                        repeat_style: if will_wrap {
                            Some(WrapAround)
                        } else {
                            None
                        },
                    };
                    (false, true)
                }
                (Backward, Some(mut r), WrapAround) => {
                    let will_wrap = delta > timer.elasped.now;
                    if will_wrap && !r.try_advance_counter() {
                        timer.elasped = Elasped {
                            now: Duration::ZERO,
                            previous: timer.elasped.now,
                            repeat_style: None,
                        };
                        break 'm (true, false);
                    }
                    let new_now = if will_wrap {
                        neg_duration_rem(
                            delta - timer.elasped.now,
                            timer.duration_limit,
                        )
                    } else {
                        timer.elasped.now - delta
                    };
                    timer.elasped = Elasped {
                        now: new_now,
                        previous: timer.elasped.now,
                        repeat_style: if will_wrap {
                            Some(WrapAround)
                        } else {
                            None
                        },
                    };
                    (false, true)
                }
                (Forward, Some(mut r), PingPong) => {
                    let new_now = timer.elasped.now + delta;
                    let will_pingpong = new_now > timer.duration_limit;
                    if will_pingpong {
                        if !r.try_advance_counter() {
                            timer.elasped = Elasped {
                                now: timer.duration_limit,
                                previous: timer.elasped.previous,
                                repeat_style: None,
                            };
                            break 'm (true, false);
                        }
                        let new_now =
                            neg_duration_rem(new_now, timer.duration_limit);
                        timer.direction = Backward;
                        timer.elasped = Elasped {
                            now: new_now,
                            previous: timer.elasped.now,
                            repeat_style: Some(PingPong),
                        };
                        (false, true)
                    } else {
                        timer.elasped = Elasped {
                            now: new_now,
                            previous: timer.elasped.now,
                            repeat_style: None,
                        };
                        (false, false)
                    }
                }
                (Backward, Some(mut r), PingPong) => {
                    let will_pingpong = delta > timer.elasped.now;
                    if will_pingpong {
                        if !r.try_advance_counter() {
                            timer.elasped = Elasped {
                                now: Duration::ZERO,
                                previous: timer.elasped.previous,
                                repeat_style: None,
                            };
                            break 'm (true, false);
                        }
                        let new_now = duration_rem(
                            delta - timer.elasped.now,
                            timer.duration_limit,
                        );
                        timer.direction = Forward;
                        timer.elasped = Elasped {
                            now: new_now,
                            previous: timer.elasped.now,
                            repeat_style: Some(PingPong),
                        };
                        (false, true)
                    } else {
                        timer.elasped = Elasped {
                            now: timer.elasped.now - delta,
                            previous: timer.elasped.now,
                            repeat_style: None,
                        };
                        (false, false)
                    }
                }
            }
        };

        match (is_now_finished, is_repeated) {
            (true, false) | (false, true) => {
                ended_writer.send(TweenTimerEnded {
                    timer: entity,
                    current_direction: timer.direction,
                    with_repeat: timer.repeat,
                });
            }
            (true, true) => unreachable!(),
            (false, false) => {}
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
