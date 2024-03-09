use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tween::{
    prelude::*,
    tween_timer::{AnimationDirection, TweenTimer},
};
use rand::prelude::*;

mod my_interpolator {
    use bevy::prelude::*;
    use bevy_tween::prelude::*;
    pub struct ShakeIntensity {
        pub start: f32,
        pub end: f32,
    }
    impl Interpolator for ShakeIntensity {
        type Item = super::ShakeIntensitiy;

        fn interpolate(&self, item: &mut Self::Item, value: f32) {
            item.0 = self.start.lerp(self.end, value)
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (shake_big_x, mouse_hold_then_shake))
        .add_systems(
            PostUpdate,
            bevy_tween::tween::resource_tween_system::<
                my_interpolator::ShakeIntensity,
            >,
        )
        .init_resource::<ShakeIntensitiy>()
        .run();
}

#[derive(Component)]
pub struct BigX;

#[derive(Component)]
pub struct ShakeTween;

#[derive(Default, Resource)]
pub struct ShakeIntensitiy(f32);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    window.single_mut().cursor.icon = CursorIcon::Pointer;
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("big_x.png"),
            ..Default::default()
        },
        BigX,
    ));
    commands.spawn((
        ShakeTween,
        SpanTweenPlayerBundle::new(Duration::from_secs(1)),
        SpanTweenBundle::new(..Duration::from_secs(1), EaseFunction::Linear),
        ResourceTween::new(my_interpolator::ShakeIntensity {
            start: 0.,
            end: 1.,
        }),
    ));
}

fn mouse_hold_then_shake(
    mut q_shake_tween: Query<&mut TweenTimer, With<ShakeTween>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    q_shake_tween.single_mut().direction =
        if mouse_button.pressed(MouseButton::Left) {
            AnimationDirection::Forward
        } else {
            AnimationDirection::Backward
        };
}
fn shake_big_x(
    shake_intensity: Res<ShakeIntensitiy>,
    mut q_big_x: Query<&mut Transform, With<BigX>>,
) {
    let mut rng = rand::thread_rng();
    let dx: f32 = rng.gen();
    let dy: f32 = rng.gen();
    q_big_x.single_mut().translation =
        Vec3::new(dx - 0.5, dy - 0.5, 0.) * 100. * shake_intensity.0;
}
