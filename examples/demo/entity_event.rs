use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_tween::{
    bevy_time_runner::TimeRunnerEnded, combinator::forward, prelude::*,
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
}
