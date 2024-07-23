use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    core_pipeline::bloom::BloomSettings,
    prelude::*,
    reflect::ParsedPath,
    sprite::Mesh2dHandle,
};

use bevy_tween::{
    bevy_time_runner::TimeRunner,
    builder::{forward, parallel, sequence},
    prelude::*,
    targets::TargetResource,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_annulus_shape,
                delayed_start.run_if(
                    bevy::time::common_conditions::once_after_delay(
                        Duration::from_secs(1),
                    ),
                ),
            ),
        )
        .insert_resource(Percentage(0.))
        .register_type::<AnnulusShape>()
        .run();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct AnnulusShape(Annulus);

#[derive(Resource)]
struct Percentage(f32);

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            ..Default::default()
        },
        BloomSettings::OLD_SCHOOL,
    ));

    let annulus = commands
        .spawn((
            AnnulusShape(Annulus::new(0., 0.)),
            ColorMesh2dBundle {
                material: materials.add(into_color(WHITE * 2.)),
                ..Default::default()
            },
        ))
        .id()
        .into_target();

    let text = commands
        .spawn(TextBundle {
            text: Text::from_sections("bevy_tween".chars().map(|c| {
                TextSection::new(
                    c,
                    TextStyle {
                        color: into_color(WHITE.with_alpha(0.)),
                        font_size: 75.,
                        ..Default::default()
                    },
                )
            })),
            style: Style {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
        .into_target();

    commands.animation().add(parallel((
        TargetResource
            .dynamic_set()
            .resource(|r: &mut Percentage, v| {
                r.0 = *v;
                println!("Percentage: {:.1}", v);
            })
            .tween(0., 100., Duration::from_secs(9), EaseFunction::Linear),
        text.dynamic_set()
            .component(|text: &mut Text, v: &f32| {
                let v = *v * text.sections.len() as f32;
                let vfloor = v.floor();
                let vimax = vfloor as usize;
                for i in 0..vimax {
                    text.sections[i].style.color.set_alpha(1.);
                }
                let a = v - vfloor;
                if a > 0. {
                    text.sections[vimax].style.color.set_alpha(a);
                }
            })
            .tween(0., 1., Duration::from_secs(3), EaseFunction::QuadraticOut),
        annulus
            .dynamic_set()
            .component_handle(
                |h: &Handle<ColorMaterial>| h,
                |c, v| c.color = *v,
            )
            .tween(
                into_color(WHITE * 5.),
                into_color(DEEP_PINK * 2.),
                Duration::from_secs(9),
                EaseFunction::CubicOut,
            ),
        annulus
            .dynamic_set()
            .component(|c: &mut AnnulusShape, v: &f32| {
                let base_radius = 300.0_f32;
                c.0.inner_circle.radius = base_radius - *v;
                c.0.outer_circle.radius = base_radius + *v;
            })
            .tween(0., 50., Duration::from_secs(3), EaseFunction::CubicOut),
        sequence((
            forward(Duration::from_secs(3)),
            annulus
                .dynamic_set()
                .path::<AnnulusShape, f32>(
                    ParsedPath::parse(".0.outer_circle.radius").unwrap(),
                )
                .tween(
                    350.,
                    2300.,
                    Duration::from_secs(3),
                    EaseFunction::CubicIn,
                ),
        )),
    )));
}

fn delayed_start(mut query: Query<&mut TimeRunner>) {
    query.single_mut().set_paused(false);
}

fn update_annulus_shape(
    mut query: Query<(&AnnulusShape, &mut Mesh2dHandle), Changed<AnnulusShape>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    query.iter_mut().for_each(|(shape, mut mesh_handle)| {
        let new_mesh = bevy::render::mesh::AnnulusMeshBuilder {
            annulus: shape.0,
            resolution: 128,
        }
        .build();
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            *mesh = new_mesh;
        } else {
            mesh_handle.0 = meshes.add(new_mesh);
        };
    })
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}
