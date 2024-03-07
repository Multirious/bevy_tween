//! Module containg important implementations of a tween player
//!
//! A tween player is one big part of a tween in this crate.
//!
//! [`TweenPlayerState`] alone do not handles any tweening behavior but instead
//! delegates it through components and systems like below as documented in [`TweenSystemSet`]:
//!  1. Update [`TweenPlayerState`]'s elasped time
//!  2. Any tween player implementation updates any [`TweenState`] that
//!     it responsibles for.
//!  3. Systems in [`interpolation`] query any [`TweenState`]s and
//!     output and insert the result in the same entity as [`TweenInterpolationValue`] component.
//!  4. Systems in [`tween`] query any [`TweenInterpolationValue`] in its entity
//!     then update its tweening value.
//! This method of communication with agreed upon components like an interface or
//! dependency injection I guess, is heavily utilized in this crate for maximum
//! decoupling and flexbility.
//!
//! With [`TweenPlayerState`], it consist of informations any other specific
//! tween player implementation may want.
//! The current elasped time, repeating setting, and repeating behavior is
//! automatically handled by [`tick_tween_player_state_system`]. The average
//! users may not need to deal with the details of items in this module but
//! instead the usage in specific tween player implementation.
//!
//! [`interpolation`]: crate::interpolation
//! [`span_tween`]: crate::span_tween
//! [`tween`]: crate::tween
//! [`TweenState`]: crate::tween::TweenState
//! [`TweenInterpolationValue`]: crate::tween::TweenInterpolationValue
//! [`TweenSystemSet`]: crate::TweenSystemSet

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
#[derive(Debug, Default, Component, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TweenPlayerState {
    /// Stop the ticking system from updating this player.
    pub paused: bool,
    /// The current elasped time with other useful information.
    elasped: Elasped,
    /// When this player should stop or repeat if configured.
    pub duration_limit: Duration,
    /// Playback direction of the current player.
    pub direction: AnimationDirection,
    /// Configure to repeat.
    pub repeat: Option<Repeat>,
    /// Configure to repeat with a style.
    pub repeat_style: Option<RepeatStyle>,
}

impl TweenPlayerState {
    /// Create new [`TweenPlayerState`] with this duration.
    pub fn new(duration_limit: Duration) -> TweenPlayerState {
        TweenPlayerState {
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

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Event, Reflect)]
// pub struct TweenPlayerEnded {
//     pub tween_player: Entity,
//     pub repeat: Option<(Repeat, RepeatStyle, AnimationDirection)>,
// }

// impl TweenPlayerEnded {
//     pub fn will_repeat(&self) -> bool {
//         self.repeat
//             .map(|(repeat, ..)| match repeat {
//                 Repeat::Infinitely => true,
//                 Repeat::InfinitelyCounted { .. } => true,
//                 Repeat::Times {
//                     times,
//                     times_repeated,
//                 } => times < times_repeated,
//             })
//             .unwrap_or(false)
//     }
// }

/// Updates any [`TweenPlayerState`] elasped time and handles the repeat if configured.
pub fn tick_tween_player_state_system(
    time: Res<Time<Real>>,
    mut q_tween_player: Query<&mut TweenPlayerState>,
) {
    use AnimationDirection::*;
    use RepeatStyle::*;
    let delta = time.delta();
    q_tween_player.iter_mut().for_each(|mut tween_player| {
        if !tween_player.paused {
            match (
                tween_player.direction,
                tween_player.repeat,
                tween_player.repeat_style.unwrap_or_default(),
            ) {
                (Forward, None, _) => {
                    if tween_player.elasped.now >= tween_player.duration_limit {
                        return;
                    }
                    let new_now = (tween_player.elasped.now + delta)
                        .min(tween_player.duration_limit);
                    tween_player.elasped = Elasped {
                        now: new_now,
                        previous: tween_player.elasped.now,
                        repeat_style: None,
                    };
                }
                (Backward, None, _) => {
                    if tween_player.elasped.now == Duration::ZERO {
                        return;
                    }
                    let new_now =
                        tween_player.elasped.now.saturating_sub(delta);
                    tween_player.elasped = Elasped {
                        now: new_now,
                        previous: tween_player.elasped.now,
                        repeat_style: None,
                    };
                }
                (Forward, Some(mut r), WrapAround) => {
                    let new_now = tween_player.elasped.now + delta;
                    let will_wrap = new_now >= tween_player.duration_limit;
                    if will_wrap && !r.try_advance_counter() {
                        tween_player.elasped = Elasped {
                            now: tween_player.duration_limit,
                            previous: tween_player.elasped.now,
                            repeat_style: None,
                        };
                        return;
                    }
                    let new_now =
                        duration_rem(new_now, tween_player.duration_limit);
                    tween_player.elasped = Elasped {
                        now: new_now,
                        previous: tween_player.elasped.now,
                        repeat_style: if will_wrap {
                            Some(WrapAround)
                        } else {
                            None
                        },
                    };
                }
                (Backward, Some(mut r), WrapAround) => {
                    let will_wrap = delta > tween_player.elasped.now;
                    if will_wrap && !r.try_advance_counter() {
                        tween_player.elasped = Elasped {
                            now: Duration::ZERO,
                            previous: tween_player.elasped.now,
                            repeat_style: None,
                        };
                        return;
                    }
                    let new_now = if will_wrap {
                        neg_duration_rem(
                            delta - tween_player.elasped.now,
                            tween_player.duration_limit,
                        )
                    } else {
                        tween_player.elasped.now - delta
                    };
                    tween_player.elasped = Elasped {
                        now: new_now,
                        previous: tween_player.elasped.now,
                        repeat_style: if will_wrap {
                            Some(WrapAround)
                        } else {
                            None
                        },
                    };
                }
                (Forward, Some(mut r), PingPong) => {
                    let new_now = tween_player.elasped.now + delta;
                    let will_pingpong = new_now > tween_player.duration_limit;
                    if will_pingpong {
                        if !r.try_advance_counter() {
                            tween_player.elasped = Elasped {
                                now: tween_player.duration_limit,
                                previous: tween_player.elasped.previous,
                                repeat_style: None,
                            };
                            return;
                        }
                        let new_now = neg_duration_rem(
                            new_now,
                            tween_player.duration_limit,
                        );
                        tween_player.direction = Backward;
                        tween_player.elasped = Elasped {
                            now: new_now,
                            previous: tween_player.elasped.now,
                            repeat_style: Some(PingPong),
                        };
                    } else {
                        tween_player.elasped = Elasped {
                            now: new_now,
                            previous: tween_player.elasped.now,
                            repeat_style: None,
                        };
                    }
                }
                (Backward, Some(mut r), PingPong) => {
                    let will_pingpong = delta > tween_player.elasped.now;
                    if will_pingpong {
                        if !r.try_advance_counter() {
                            tween_player.elasped = Elasped {
                                now: Duration::ZERO,
                                previous: tween_player.elasped.previous,
                                repeat_style: None,
                            };
                            return;
                        }
                        let new_now = duration_rem(
                            delta - tween_player.elasped.now,
                            tween_player.duration_limit,
                        );
                        tween_player.direction = Forward;
                        tween_player.elasped = Elasped {
                            now: new_now,
                            previous: tween_player.elasped.now,
                            repeat_style: Some(PingPong),
                        };
                    } else {
                        tween_player.elasped = Elasped {
                            now: tween_player.elasped.now - delta,
                            previous: tween_player.elasped.now,
                            repeat_style: None,
                        };
                    }
                }
            }
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
