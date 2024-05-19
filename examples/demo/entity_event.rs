use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_tween::{
    bevy_time_runner::{TimeRunnerEnded, TimeRunnerPlugin},
    combinator::forward,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            EventListenerPlugin::<TimeRunnerEnded>::default(),
            TimeRunnerPlugin::default(),
            DefaultTweenPlugins,
        ))
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
        .insert(|a, pos| forward(Duration::from_secs_f32(0.5))(a, pos));
}
