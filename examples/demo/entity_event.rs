use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded,
    combinator::{event, forward, sequence},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(On::<TimeRunnerEnded>::run(
            |listener: Listener<TimeRunnerEnded>| {
                if listener.is_completed() {
                    println!("done!");
                } else {
                    println!("repeating");
                }
            },
        ))
        .animation()
        .repeat(Repeat::times(5))
        .insert(forward(Duration::from_secs_f32(0.5)));

    commands
        .spawn(On::<TweenEvent<&'static str>>::run(
            |listener: Listener<TweenEvent<&'static str>>| {
                println!("{}", listener.data);
            },
        ))
        .animation()
        .insert(sequence((
            forward(Duration::from_secs_f32(3.)),
            event("event"),
            forward(Duration::from_secs_f32(0.5)),
            event("listener"),
        )));
}
