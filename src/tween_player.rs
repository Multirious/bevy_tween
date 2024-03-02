use bevy::prelude::*;
use std::time::Duration;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Elasped {
    pub now: Duration,
    pub previous: Duration,
    pub repeat_style: Option<RepeatStyle>,
}

#[derive(Debug, Default, Component, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TweenPlayerState {
    pub paused: bool,
    pub elasped: Elasped,
    pub duration_limit: Duration,
    pub direction: AnimationDirection,
    pub repeat: Option<Repeat>,
    pub repeat_style: Option<RepeatStyle>,
}

impl TweenPlayerState {
    pub fn new(duration_limit: Duration) -> TweenPlayerState {
        TweenPlayerState {
            duration_limit,
            ..Default::default()
        }
    }
    pub fn with_paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }
    // pub fn with_elasped(mut self, elasped: Duration) -> Self {
    //     self.elasped = elasped;
    //     self
    // }
    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_repeat(mut self, repeat: Option<Repeat>) -> Self {
        self.repeat = repeat;
        self
    }
    pub fn with_repeat_style(
        mut self,
        repeat_style: Option<RepeatStyle>,
    ) -> Self {
        self.repeat_style = repeat_style;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Repeat {
    Infinitely,
    InfinitelyCounted { times_repeated: usize },
    Times { times: usize, times_repeated: usize },
}

impl Repeat {
    pub fn infinitely() -> Repeat {
        Repeat::Infinitely
    }
    pub fn infinitely_counted() -> Repeat {
        Repeat::InfinitelyCounted { times_repeated: 0 }
    }
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RepeatStyle {
    #[default]
    WrapAround,
    PingPong,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AnimationDirection {
    #[default]
    Forward,
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
