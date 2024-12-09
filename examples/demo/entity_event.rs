use bevy::prelude::*;
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded,
    combinator::{event, forward, sequence},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_observer(|trigger: Trigger<TweenEvent<&'static str>>| {
            println!("TweenEvent: {}", trigger.data)
        })
        .add_observer(|trigger: Trigger<TimeRunnerEnded>| {
            if trigger.is_completed() {
                println!("done!");
            } else {
                println!("repeating");
            }
        })
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .animation()
        .repeat(Repeat::times(5))
        .insert(forward(Duration::from_secs_f32(0.5)));

    commands.animation().insert(sequence((
        forward(Duration::from_secs_f32(3.)),
        event("tween"),
        forward(Duration::from_secs_f32(0.5)),
        event("event"),
    )));
}
