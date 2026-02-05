use std::f32::consts::PI;

use bevy::window::CursorIcon;
use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    prelude::*,
    window::{PrimaryWindow, SystemCursorIcon},
};
use bevy_tween::{bevy_time_runner::TimeRunner, prelude::*};
use rand::prelude::*;

mod interpolate {
    use bevy::prelude::*;
    use bevy_tween::{prelude::*, resource_tween_system};

    pub use bevy_tween::interpolate::*;

    pub fn custom_interpolators_plugin(app: &mut App) {
        app.add_tween_systems(resource_tween_system::<EffectIntensity>());
    }

    pub struct EffectIntensity {
        pub start: f32,
        pub end: f32,
    }

    impl Interpolator for EffectIntensity {
        type Item = super::EffectIntensitiy;

        fn interpolate(
            &self,
            item: &mut Self::Item,
            value: f32,
            _previous_value: f32,
        ) {
            item.0 = self.start.lerp(self.end, value)
        }
    }

    pub fn effect_intensity(
        start: f32,
        end: f32,
    ) -> ResourceTween<EffectIntensity> {
        ResourceTween::new(EffectIntensity { start, end })
    }
}

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultTweenPlugins,
            interpolate::custom_interpolators_plugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (big_x_do_effect, mouse_hold))
        .init_resource::<EffectIntensitiy>()
        .run();
}

#[derive(Component)]
pub struct BigX;

#[derive(Component)]
pub struct EffectAnimator;

#[derive(Component)]
pub struct RotatationAnimator;

#[derive(Default, Resource)]
pub struct EffectIntensitiy(f32);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    use interpolate::{effect_intensity, sprite_color};
    commands
        .entity(window.single().unwrap())
        .insert(CursorIcon::System(SystemCursorIcon::Pointer));
    commands.spawn(Camera2d);
    let big_x = commands
        .spawn((
            Sprite {
                image: asset_server.load("big_x.png"),
                color: into_color(DEEP_PINK),
                ..default()
            },
            BigX,
        ))
        .id();
    let big_x = big_x.into_target();
    commands
        .spawn(EffectAnimator)
        .animation()
        .insert_tween_here(
            secs(1.),
            EaseKind::QuarticIn,
            (
                effect_intensity(0., 1.),
                big_x.with(sprite_color(
                    into_color(DEEP_PINK),
                    into_color(WHITE),
                )),
            ),
        );
    commands
        .spawn(RotatationAnimator)
        .animation()
        .repeat(Repeat::Infinitely)
        .insert_tween_here(
            secs(1.),
            EaseKind::Linear,
            big_x.with(interpolate::angle_z(0., PI * 0.5)),
        );
}

fn mouse_hold(
    mut q_effect_animator: Query<&mut TimeRunner, With<EffectAnimator>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let mouse_down = mouse_button.pressed(MouseButton::Left);
    q_effect_animator
        .single_mut()
        .unwrap()
        .set_direction(if mouse_down {
            TimeDirection::Forward
        } else {
            TimeDirection::Backward
        });
}

fn big_x_do_effect(
    effect_intensity: Res<EffectIntensitiy>,
    mut q_big_x: Query<&mut Transform, With<BigX>>,
    mut q_rotation_animator: Query<&mut TimeRunner, With<RotatationAnimator>>,
) {
    let mut rng = rand::rng();
    let dx: f32 = rng.random();
    let dy: f32 = rng.random();
    q_big_x.single_mut().unwrap().translation =
        Vec3::new(dx - 0.5, dy - 0.5, 0.) * 100. * effect_intensity.0;

    q_rotation_animator
        .single_mut()
        .unwrap()
        .set_time_scale(effect_intensity.0);
}
