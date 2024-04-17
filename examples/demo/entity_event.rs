use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_tween::prelude::*;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TweenerBundle::new(Duration::from_secs_f32(0.5))
            .with_repeat(Repeat::times(5)),
        On::<TweenerEnded>::run(|listener: Listener<TweenerEnded>| {
            if listener.is_completed() {
                println!("done!");
            } else {
                println!("repeating");
            }
        }),
    ));
}
