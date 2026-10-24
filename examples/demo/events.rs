use bevy::prelude::*;
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded,
    combinator::{forward, root_event, sequence},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, DefaultTweenPlugins::default()))
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<TweenEvent<&'static str>>| {
            println!("TweenEvent on {}: {}", trigger.entity, trigger.data)
        })
        .add_observer(|trigger: On<TimeRunnerEnded>| {
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

    commands
        .animation()
        .insert(sequence((
            forward(Duration::from_secs_f32(3.)),
            root_event("tween"),
            forward(Duration::from_secs_f32(0.5)),
            root_event("event"),
        )))
        .observe(|trigger: On<TweenEvent<&'static str>>| {
            println!("TweenEvent on root: {}", trigger.data)
        });
}
