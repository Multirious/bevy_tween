use bevy::{prelude::*, time::Stopwatch};
use bevy_tween::{
    builder::{parallel, tween},
    prelude::*,
};

#[derive(Component)]
pub struct Circle {
    radius: f32,
    hue: f32,
    spikiness: f32,
}

mod interpolate {
    use super::Circle;
    use bevy::prelude::*;
    use bevy_tween::{
        component_dyn_tween_system, component_tween_system, prelude::*,
    };

    pub fn interpolators_plugin(app: &mut App) {
        app.add_tween_systems((
            component_dyn_tween_system::<Circle>(),
            component_tween_system::<CircleRadius>(),
            component_tween_system::<CircleHue>(),
        ));
    }

    pub struct CircleRadius {
        start: f32,
        end: f32,
    }

    impl Interpolator for CircleRadius {
        type Item = Circle;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            item.radius = self.start.lerp(self.end, value);
        }
    }

    pub fn circle_radius(start: f32, end: f32) -> CircleRadius {
        CircleRadius { start, end }
    }

    pub struct CircleHue {
        start: f32,
        end: f32,
    }

    impl Interpolator for CircleHue {
        type Item = Circle;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            item.hue = self.start.lerp(self.end, value);
        }
    }

    pub fn circle_hue(start: f32, end: f32) -> CircleHue {
        CircleHue { start, end }
    }
}
use interpolate::{circle_hue, circle_radius};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            DefaultTweenPlugins,
            interpolate::interpolators_plugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, what_happen)
        .run();
}

fn setup(mut commands: Commands) {
    let circle_id = commands
        .spawn(Circle {
            radius: 1.,
            hue: 0.,
            spikiness: 2.,
        })
        .id();

    let circle = circle_id.into_target();
    commands.animation().insert(parallel((
        tween(
            Duration::from_secs(2),
            EaseFunction::Linear,
            circle.with(circle_hue(0., 10.)),
        ),
        tween(
            Duration::from_secs(2),
            EaseFunction::Linear,
            circle.with(circle_radius(1., 50.)),
        ),
        tween(
            Duration::from_secs(2),
            EaseFunction::Linear,
            // Requires [`component_dyn_tween_system`]
            circle.with_closure(|circle: &mut Circle, value| {
                circle.spikiness = (2.).lerp(4., value);
            }),
        ),
    )));
}

fn what_happen(
    time: Res<Time>,
    q_circle: Query<&Circle>,
    mut time_passed: Local<Stopwatch>,
    mut print_tick: Local<Stopwatch>,
) {
    time_passed.tick(time.delta());
    print_tick.tick(time.delta());
    if time_passed.elapsed_secs() < 3. && print_tick.elapsed_secs() > 0.2 {
        let circle = q_circle.single();
        println!(
            "{:.2} {:.2} {:.2}",
            circle.hue, circle.radius, circle.spikiness
        );
        print_tick.reset();
    }
}
