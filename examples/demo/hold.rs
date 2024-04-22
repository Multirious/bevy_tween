use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tween::{
    prelude::*, tween::TargetComponent, tween_timer::AnimationDirection,
    tweener::Tweener,
};
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

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
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
pub struct EffectTweener;

#[derive(Component)]
pub struct RotateTweener;

#[derive(Default, Resource)]
pub struct EffectIntensitiy(f32);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    use interpolate::{effect_intensity, sprite_color};
    window.single_mut().cursor.icon = CursorIcon::Pointer;
    commands.spawn(Camera2dBundle::default());
    let big_x = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("big_x.png"),
                ..Default::default()
            },
            BigX,
        ))
        .id();
    let big_x = TargetComponent::from(big_x);
    commands.spawn((
        EffectTweener,
        TweenerBundle::new(secs(1.)).tween_here(),
        EaseFunction::QuarticIn,
        effect_intensity(0., 1.),
        big_x.tween(sprite_color(Color::WHITE, Color::PINK)),
    ));
    commands.spawn((
        RotateTweener,
        TweenerBundle::new(Duration::from_secs_f32(1.))
            .with_repeat(Repeat::Infinitely)
            .tween_here(),
        EaseFunction::Linear,
        big_x.tween(interpolate::angle_z(0., PI * 0.5)),
    ));
}

fn mouse_hold(
    mut q_effect_tween_timer: Query<&mut Tweener, With<EffectTweener>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let mouse_down = mouse_button.pressed(MouseButton::Left);
    q_effect_tween_timer.single_mut().timer.direction = if mouse_down {
        AnimationDirection::Forward
    } else {
        AnimationDirection::Backward
    };
}

fn big_x_do_effect(
    effect_intensity: Res<EffectIntensitiy>,
    mut q_big_x: Query<&mut Transform, With<BigX>>,
    mut q_rotate_tweener: Query<&mut Tweener, With<RotateTweener>>,
) {
    let mut rng = rand::thread_rng();
    let dx: f32 = rng.gen();
    let dy: f32 = rng.gen();
    q_big_x.single_mut().translation =
        Vec3::new(dx - 0.5, dy - 0.5, 0.) * 100. * effect_intensity.0;

    q_rotate_tweener.single_mut().timer.speed_scale =
        Duration::from_secs_f32(effect_intensity.0);
}
