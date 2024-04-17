use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tween::{
    prelude::*, resource_tween_system, tween_timer::AnimationDirection,
    tweener::Tweener,
};
use rand::prelude::*;

mod my_interpolate {
    use bevy::prelude::*;
    use bevy_tween::prelude::*;
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
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (big_x_do_effect, mouse_hold))
        .add_tween_systems(resource_tween_system::<
            my_interpolate::EffectIntensity,
        >())
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
    commands.spawn((
        EffectTweener,
        TweenerBundle::new(Duration::from_secs(1)),
        TimeSpan::try_from(..Duration::from_secs(1)).unwrap(),
        EaseFunction::QuarticIn,
        ResourceTween::new(my_interpolate::EffectIntensity {
            start: 0.,
            end: 1.,
        }),
        ComponentTween::new_target(
            big_x,
            interpolate::SpriteColor {
                start: Color::WHITE,
                end: Color::PINK,
            },
        ),
    ));
    commands.spawn((
        RotateTweener,
        TweenerBundle::new(Duration::from_secs_f32(1.))
            .with_repeat(Repeat::Infinitely),
        TimeSpan::try_from(..Duration::from_secs_f32(1.)).unwrap(),
        EaseFunction::Linear,
        ComponentTween::new_target(
            big_x,
            interpolate::AngleZ {
                start: 0.,
                end: PI * 0.5,
            },
        ),
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
