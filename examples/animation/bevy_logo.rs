use bevy::prelude::*;
use bevy_svg::prelude::*;
use bevy_tween::prelude::*;
use interpolate::{AngleZ, Rotation, Translation};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            camera_control.run_if(|debug_mode: Res<DebugConfig>| {
                debug_mode.camera_control
            }),
        )
        .insert_resource(DebugConfig {
            camera_control: true,
            camera_far_view: false,
            start_tween: true,
        })
        .run()
}

#[derive(Resource)]
struct DebugConfig {
    camera_control: bool,
    camera_far_view: bool,
    start_tween: bool,
}

#[derive(Component)]
struct MainCamera;

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

type Tween<I> = ComponentTween<I>;

fn setup(
    mut commands: Commands,
    debug_config: Res<DebugConfig>,
    asset_server: Res<AssetServer>,
) {
    let bevy_text: Handle<Svg> = asset_server.load("bevy_text.svg");
    let bird_light: Handle<Svg> = asset_server.load("bird_light.svg");
    let bird_dim: Handle<Svg> = asset_server.load("bird_dim.svg");
    let bird_dark: Handle<Svg> = asset_server.load("bird_dark.svg");

    let start_cam_pos = Vec3::new(2.3, -3.2, 2.8);
    let end_cam_pos = Vec3::new(6.5, -2.7, 9.6);
    let final_bird_light_transform = Transform::from_xyz(0., 0., 0.);
    let start_bird_light_transform = final_bird_light_transform
        .with_translation(
            final_bird_light_transform.translation + Vec3::new(-200., 100., 0.),
        );

    let final_bevy_text_transform = Transform::from_xyz(3., 0., 0.);
    let bevy_text_rotate_start = Quat::from_rotation_x(90_f32.to_radians());
    let bevy_text_rotate_end = Quat::default();

    let camera = commands
        .spawn((
            MainCamera,
            Camera3dBundle {
                transform: Transform::from_translation(
                    if debug_config.camera_far_view {
                        end_cam_pos
                    } else {
                        start_cam_pos
                    },
                ),
                ..Default::default()
            },
        ))
        .id();

    let bird_light = commands
        .spawn(Svg3dBundle {
            svg: bird_light,
            origin: Origin::TopLeft,
            transform: start_bird_light_transform,
            ..Default::default()
        })
        .id();
    // let bird_dim = commands.spawn(Svg3dBundle {
    //     svg: bird_dim,
    //     origin: Origin::TopLeft,
    //     transform: final_bird_pos,
    //     ..Default::default()
    // }).id();
    // let bird_dark = commands.spawn(Svg3dBundle {
    //     svg: bird_dark,
    //     origin: Origin::TopLeft,
    //     transform: final_bird_pos,
    //     ..Default::default()
    // }).id();
    let bevy_text = commands
        .spawn(Svg3dBundle {
            svg: bevy_text,
            origin: Origin::TopLeft,
            transform: final_bevy_text_transform
                .with_rotation(bevy_text_rotate_start),
            ..Default::default()
        })
        .id();

    commands
        .spawn(SpanTweenerBundle::new(secs(30.)))
        .with_children(|c| {
            c.span_tweens()
                .tween(
                    secs(7.),
                    EaseFunction::QuinticOut,
                    Tween::new_target(
                        bird_light,
                        Translation {
                            start: start_bird_light_transform.translation,
                            end: final_bird_light_transform.translation,
                        },
                    ),
                )
                .backward(secs(5.))
                .tween(
                    secs(10.),
                    EaseFunction::QuarticInOut,
                    Tween::new_target(
                        camera,
                        Translation {
                            start: start_cam_pos,
                            end: end_cam_pos,
                        },
                    ),
                )
                .backward(secs(4.))
                .tween(
                    secs(1.),
                    EaseFunction::BackOut,
                    Tween::new_target(
                        bevy_text,
                        Rotation {
                            start: bevy_text_rotate_start,
                            end: bevy_text_rotate_end,
                        },
                    ),
                );
        });
}

fn camera_control(
    input: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut cam_transform = q_camera.single_mut();
    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::ControlLeft) {
        if input.just_pressed(KeyCode::ArrowUp) {
            dir += Vec3::new(0., 0.1, 0.);
        }
        if input.just_pressed(KeyCode::ArrowDown) {
            dir += Vec3::new(0., -0.1, 0.);
        }
        if input.just_pressed(KeyCode::ArrowLeft) {
            dir += Vec3::new(-0.1, 0., 0.);
        }
        if input.just_pressed(KeyCode::ArrowRight) {
            dir += Vec3::new(0.1, 0., 0.);
        }
        if input.just_pressed(KeyCode::KeyW) {
            dir += Vec3::new(0., 0., 0.1);
        }
        if input.just_pressed(KeyCode::KeyS) {
            dir += Vec3::new(0., 0., -0.1);
        }
    } else {
        if input.pressed(KeyCode::ArrowUp) {
            dir += Vec3::new(0., 0.1, 0.);
        }
        if input.pressed(KeyCode::ArrowDown) {
            dir += Vec3::new(0., -0.1, 0.);
        }
        if input.pressed(KeyCode::ArrowLeft) {
            dir += Vec3::new(-0.1, 0., 0.);
        }
        if input.pressed(KeyCode::ArrowRight) {
            dir += Vec3::new(0.1, 0., 0.);
        }
        if input.pressed(KeyCode::KeyW) {
            dir += Vec3::new(0., 0., -0.1);
        }
        if input.pressed(KeyCode::KeyS) {
            dir += Vec3::new(0., 0., 0.1);
        }
    }
    cam_transform.translation += dir;

    if input.just_pressed(KeyCode::Space) {
        println!("Cam pos: {}", cam_transform.translation);
    }
}
