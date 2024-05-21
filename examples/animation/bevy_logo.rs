use bevy::{app::AppExit, prelude::*, time::Stopwatch, utils::HashMap};
use bevy_lookup_curve::{
    editor::LookupCurveEditor, LookupCurve, LookupCurvePlugin,
};
use bevy_svg::prelude::*;
use bevy_time_runner::TimeRunner;
use bevy_tween::{
    combinator::{backward, forward, sequence, tween},
    interpolate::{rotation, rotation_to, translation_to},
    prelude::*,
    tween::TargetComponent,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LookupCurvePlugin,
            SvgPlugin,
            DefaultTweenPlugins,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                restart_animation,
                camera_control.run_if(|debug_mode: Res<DebugConfig>| {
                    debug_mode.camera_control
                }),
            ),
        )
        .add_systems(Last, save_curve_on_exit)
        .insert_resource(DebugConfig {
            camera_control: true,
            camera_far_view: false,
            start_tween: true,
        })
        .insert_resource(Curves(vec![(Handle::default(), "cam_curve.ron")]))
        .run()
}

#[derive(Resource)]
struct Curves(Vec<(Handle<LookupCurve>, &'static str)>);

#[derive(Resource)]
struct DebugConfig {
    camera_control: bool,
    camera_far_view: bool,
    start_tween: bool,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct AnimationAnimator;

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

type Tween<I> = ComponentTween<I>;

fn setup(
    mut commands: Commands,
    mut curves: ResMut<Curves>,
    debug_config: Res<DebugConfig>,
    asset_server: Res<AssetServer>,
) {
    let bevy_text = asset_server.load::<Svg>("bevy_text.svg");
    let bird_light = asset_server.load::<Svg>("bird_light.svg");
    let bird_dim = asset_server.load::<Svg>("bird_dim.svg");
    let bird_dark = asset_server.load::<Svg>("bird_dark.svg");

    let cam_curve = asset_server.load::<LookupCurve>("cam_curve.ron");
    curves.0[0].0 = cam_curve.clone();

    let cam_rot_0 =
        Quat::from_xyzw(0.12498875, 0.3675439, -0.04992207, 0.9202112);
    let cam_rot_1 = Quat::IDENTITY;

    let cam_pos_0 = Vec3::new(4.2928004, -3.907999, 1.0312049);
    let cam_pos_1 = Vec3::new(6.5, -2.7, 9.6);

    let bird_light_pos_1 = Vec3::new(0., 0., 0.);
    let bird_light_pos_0 = bird_light_pos_1 + Vec3::new(-200., 100., 0.);

    let bevy_text_pos_1 = Vec3::new(3., 0., 0.);

    let bevy_text_rot_0 = Quat::from_rotation_x(90_f32.to_radians());
    let bevy_text_rot_1 = Quat::default();

    let camera = commands
        .spawn((
            MainCamera,
            Camera3dBundle {
                transform: Transform::from_translation(
                    if debug_config.camera_far_view {
                        cam_pos_1
                    } else {
                        cam_pos_0
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
            transform: Transform::from_translation(bird_light_pos_0),
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
            transform: Transform::from_translation(bevy_text_pos_1)
                .with_rotation(bevy_text_rot_0),
            ..Default::default()
        })
        .id();

    let bird_light = TargetComponent::Entity(bird_light);
    let camera = TargetComponent::Entity(camera);
    let bevy_text = TargetComponent::Entity(bevy_text);

    let mut bird_light_pos = bird_light.state(bird_light_pos_0);
    let mut cam_pos = camera.state(cam_pos_0);
    let mut cam_rot = camera.state(cam_rot_0);

    commands
        .spawn(AnimationAnimator)
        .animation()
        .insert(sequence((
            tween(
                secs(7.),
                EaseFunction::QuinticOut,
                bird_light_pos.with(translation_to(bird_light_pos_1)),
            ),
            backward(secs(5.)),
            tween(
                secs(10.),
                (
                    cam_curve.clone(), /* LookupCurveEditor::new(cam_curve) */
                ),
                (
                    cam_pos.with(translation_to(cam_pos_1)),
                    cam_rot.with(rotation_to(cam_rot_1)),
                ),
            ),
            backward(secs(4.)),
            tween(
                secs(1.),
                EaseFunction::BackOut,
                bevy_text.with(rotation(bevy_text_rot_0, bevy_text_rot_1)),
            ),
            forward(secs(10.)),
        )));
}

fn restart_animation(
    input: Res<ButtonInput<KeyCode>>,
    mut animator: Query<&mut TimeRunner, With<AnimationAnimator>>,
) {
    if input.pressed(KeyCode::KeyR) {
        let mut animator = animator.single_mut();
        animator.set_tick(0.);
    }
}

fn input_maybe_just_pressed(
    input: &ButtonInput<KeyCode>,
    is_just: bool,
    key: KeyCode,
) -> bool {
    if is_just {
        input.just_pressed(key)
    } else {
        input.pressed(key)
    }
}

fn camera_control(
    input: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut cam_transform = q_camera.single_mut();

    let move_speed = 0.1;
    let rot_speed = 0.01;

    let is_just = input.pressed(KeyCode::ShiftLeft);

    let mut dir = Vec3::ZERO;
    if input_maybe_just_pressed(&input, is_just, KeyCode::Space) {
        dir += Vec3::Y * move_speed;
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::ControlLeft) {
        dir += Vec3::NEG_Y * move_speed;
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::KeyW) {
        dir += Vec3::NEG_Z * move_speed;
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::KeyS) {
        dir += Vec3::Z * move_speed;
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::KeyA) {
        dir += Vec3::NEG_X * move_speed;
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::KeyD) {
        dir += Vec3::X * move_speed;
    }
    cam_transform.translation += dir;

    if input_maybe_just_pressed(&input, is_just, KeyCode::ArrowUp) {
        cam_transform.rotation *= Quat::from_axis_angle(Vec3::X, rot_speed);
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::ArrowDown) {
        cam_transform.rotation *= Quat::from_axis_angle(Vec3::X, -rot_speed);
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::ArrowLeft) {
        cam_transform.rotation =
            Quat::from_axis_angle(Vec3::Y, rot_speed) * cam_transform.rotation;
    }
    if input_maybe_just_pressed(&input, is_just, KeyCode::ArrowRight) {
        cam_transform.rotation =
            Quat::from_axis_angle(Vec3::Y, -rot_speed) * cam_transform.rotation;
    }

    if input.just_pressed(KeyCode::KeyC) {
        println!(
            "Cam: {} {}",
            cam_transform.translation, cam_transform.rotation
        );
    }
}

fn save_curve_on_exit(
    curves: Res<Curves>,
    lookup_curve: Res<Assets<LookupCurve>>,
    mut app_exit: EventReader<AppExit>,
) {
    use std::{fs::File, io::Write};
    if app_exit.read().last().is_some() {
        for (curve, name) in &curves.0 {
            println!("Saving {name}");
            let curve = lookup_curve.get(curve).unwrap();
            let file_path = "assets/".to_string() + name;
            let mut file = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)
                .unwrap();
            let s = ron::to_string(curve).unwrap();
            file.write_all(s.as_bytes()).unwrap();
        }
    }
}

// fn init_cam_curve() {
//     use bevy_lookup_curve::{Knot, KnotInterpolation};
//     use std::{fs::File, io::Write};

//     let curve = LookupCurve::new(vec![
//         Knot {
//             position: Vec2::ZERO,
//             interpolation: KnotInterpolation::Cubic,
//             ..Default::default()
//         },
//         Knot {
//             position: Vec2::ONE,
//             interpolation: KnotInterpolation::Linear,
//             ..Default::default()
//         },
//     ]);

//     let mut file = File::options()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open("assets/cam_curve.ron")
//         .unwrap();
//     let to_string = &ron::to_string(&curve).unwrap();
//     file.write_all(to_string.as_bytes()).unwrap();
// }
