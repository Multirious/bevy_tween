mod utils;

use std::f32::consts::TAU;

use bevy::{
    color::palettes::css::{DEEP_PINK, WHITE},
    prelude::*,
};
use bevy_tween::{
    bevy_time_runner::TimeRunner,
    interpolate::{scale, sprite_color, translation},
    prelude::*,
    tween::AnimationTarget,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy::feathers::FeathersPlugins,
            DefaultTweenPlugins::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (utils::main_cursor_world_coord_system, jeb_follows_cursor),
        )
        .insert_resource(bevy::feathers::theme::UiTheme(
            bevy::feathers::dark_theme::create_dark_theme(),
        ))
        .init_resource::<Config>()
        .init_resource::<utils::MainCursorWorldCoord>()
        .run();
}

#[derive(Clone, Copy)]
enum UpdateKind {
    CursorMoved,
    CursorStopped,
    AnimatorCompleted,
}

// Let us change the tween ease and duration at runtime
#[derive(Clone, Copy, Resource)]
struct Config {
    tween_duration: Duration,
    tween_ease: EaseKind,
    update_kind: UpdateKind,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            update_kind: UpdateKind::CursorMoved,
            tween_duration: Duration::from_millis(500),
            tween_ease: EaseKind::ExponentialOut,
        }
    }
}

/// Marker component for the square that will be following the cursor
#[derive(Component)]
struct Jeb;

/// Marker component for the tween entity we will be modifying to make the follow
/// effect
#[derive(Component)]
struct JebTranslationAnimator;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, utils::MainCamera));

    // Spawning the square
    commands
        .spawn((
            Sprite {
                image: asset_server.load("square_filled.png"),
                ..default()
            },
            Jeb,
            AnimationTarget,
        ))
        .with_children(|c| {
            // Spawning the marker for an animator that will be responsible
            // for the follow effect
            c.spawn(JebTranslationAnimator);

            let jeb = AnimationTarget.into_target();
            // Spawning an animator that's responsible for a rotating effect
            c.animation()
                .repeat(Repeat::Infinitely)
                .repeat_style(RepeatStyle::PingPong)
                .insert_tween_here(
                    Duration::from_secs(2),
                    EaseKind::CubicInOut,
                    jeb.with_closure(|transform: &mut Transform, value, _| {
                        let start = 0.;
                        let end = TAU;
                        transform.rotation =
                            Quat::from_rotation_z(start.lerp(end, value));
                    }),
                );

            // Spawning a Tweener that's responsible for scaling effect
            // when you launch up the demo.
            c.animation().insert_tween_here(
                Duration::from_secs(1),
                EaseKind::QuinticIn,
                jeb.with(scale(Vec3::ZERO, Vec3::ONE)),
            );
        });

    commands.spawn_scene(config_ui());
}

fn jeb_follows_cursor(
    mut commands: Commands,
    coord: Res<utils::MainCursorWorldCoord>,
    config: Res<Config>,
    q_jeb: Query<&Transform, With<Jeb>>,
    q_jeb_translation_animator: Query<
        (Entity, Option<&TimeRunner>),
        With<JebTranslationAnimator>,
    >,
    mut cursor_moved: MessageReader<CursorMoved>,
) {
    let Some(coord) = coord.0 else {
        return;
    };
    if let (Ok(jeb_transform), Ok((jeb_animator_entity, jeb_time_runner))) =
        (q_jeb.single(), q_jeb_translation_animator.single())
    {
        let update = match config.update_kind {
            UpdateKind::CursorMoved => cursor_moved.read().next().is_some(),
            UpdateKind::CursorStopped => {
                let dx = (coord.x - jeb_transform.translation.x).abs();
                let dy = (coord.x - jeb_transform.translation.x).abs();
                let is_near_coord = dx < 0.05 && dy < 0.05;
                cursor_moved.read().next().is_none() && !is_near_coord
            }
            UpdateKind::AnimatorCompleted => match jeb_time_runner {
                Some(jeb_time_runner) => {
                    jeb_time_runner.is_completed()
                        && coord != jeb_transform.translation.xy()
                }
                None => true,
            },
        };
        if update {
            let jeb = AnimationTarget.into_target();
            commands
                .entity(jeb_animator_entity)
                .animation()
                .insert_tween_here(
                    config.tween_duration,
                    config.tween_ease,
                    (
                        jeb.with(translation(
                            jeb_transform.translation,
                            Vec3::new(coord.x, coord.y, 0.),
                        )),
                        jeb.with(sprite_color(
                            into_color(WHITE),
                            into_color(DEEP_PINK),
                        )),
                    ),
                );
        }
    }
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}

#[derive(Clone, Default, Component)]
struct EaseMenuButton;

fn config_ui() -> impl Scene {
    use bevy::{
        feathers::{
            controls::{
                ButtonVariant, FeathersButton, FeathersMenu,
                FeathersMenuButton, FeathersMenuItem, FeathersMenuPopup,
                FeathersScrollbar, FeathersSlider, ScrollbarGutter,
            },
            rounded_corners::RoundedCorners,
            theme::{ThemeBackgroundColor, ThemedText},
            tokens,
        },
        reflect::{
            TypeInfo, Typed,
            enums::{DynamicEnum, DynamicVariant, Enum, VariantInfo},
        },
        ui_widgets::{
            Activate, ScrollArea, SliderPrecision, SliderStep, SliderValue,
            ValueChange, slider_self_update,
        },
    };

    let property = |text: &str| {
        bsn! {
            Text(text)
            ThemedText
            Node {
                grid_column: GridPlacement::start(1),
            }
        }
    };
    let control = || {
        bsn! {
            Node {
                grid_column: GridPlacement::start(2),
            }
        }
    };
    let ease_menu_items = {
        let TypeInfo::Enum(info) = EaseKind::type_info() else {
            unreachable!()
        };
        info.iter()
            .filter_map(|variant| {
                let VariantInfo::Unit(u) = variant else {
                    return None;
                };
                let name = u.name();
                let value = EaseKind::from_reflect(&DynamicEnum::new(
                    name,
                    DynamicVariant::Unit,
                ))
                .unwrap();
                let scene = bsn! {
                    @FeathersMenuItem {
                        @caption: bsn! { Text(name) },
                    }
                    on(move |
                        _: On<Activate>,
                        mut config: ResMut<Config>,
                        mut q_text: Query<&mut Text, With<EaseMenuButton>>,
                    | {
                        config.tween_ease = value;
                        let mut text = q_text.single_mut().unwrap();
                        text.0 = name.to_string();
                    })
                };
                Some(scene)
            })
            .collect::<Vec<_>>()
    };
    let update_kind_on = |val| {
        on(
            move |e: On<Activate>,
                  mut config: ResMut<Config>,
                  mut q_button_variant: Query<&mut ButtonVariant>,
                  q_parent: Query<&ChildOf>,
                  q_children: Query<&Children>| {
                config.update_kind = val;
                *q_button_variant.get_mut(e.entity).unwrap() =
                    ButtonVariant::Primary;
                let parent = q_parent.get(e.entity).unwrap();
                let other_buttons = q_children.get(parent.parent()).unwrap();
                for button in other_buttons {
                    if *button == e.entity {
                        continue;
                    }
                    *q_button_variant.get_mut(*button).unwrap() =
                        ButtonVariant::Normal;
                }
            },
        )
    };
    bsn! {
        Node {
            left: px(20),
            top: px(20),
            padding: px(20),
            display: Display::Grid,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            grid_template_columns: vec![
                RepeatedGridTrack::auto(1),
            ],
            grid_auto_columns: vec![GridTrack::fr(1.)],
            row_gap: px(10),
            column_gap: px(10),
            border_radius: BorderRadius::all(px(10)),
        }
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children [
            @property("Tween duration")
            --
            @control()
            Children [
                @FeathersSlider {
                    @min: 0.1,
                    @max: 5.0,
                }
                SliderStep(0.1)
                SliderPrecision(1)
                SliderValue(0.5)
                on(slider_self_update)
                on(|v: On<ValueChange<f32>>, mut config: ResMut<Config>| {
                    config.tween_duration = Duration::from_secs_f32(v.value);
                })
                --
                Node { margin: UiRect::left(px(5)) }
                Text("s")
                ThemedText
            ]
            --
            @property("Tween ease")
            --
            @control()
            @FeathersMenu
            Children [
                Node {
                    width: percent(100),
                }
                @FeathersMenuButton {
                    @caption: bsn! {
                        template(|ctx| {
                            Ok(Text(
                                ctx.resource::<Config>()
                                    .tween_ease
                                    .variant_name()
                                    .to_string()
                            ))
                        })
                        EaseMenuButton
                        ThemedText
                    }
                }
                --
                @FeathersMenuPopup
                Node {
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::right(px(14)),
                }
                ScrollbarGutter(px(14))
                Children [
                    #MenuItems Node {
                        overflow: Overflow::scroll_y(),
                        max_height: vh(50),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Stretch,
                        align_items: AlignItems::Stretch,
                    }
                    ScrollArea
                    Children [
                        {ease_menu_items}
                    ]
                    --
                    Node {
                        position_type: PositionType::Absolute,
                        right: px(0),
                        top: px(0),
                        bottom: px(0),
                        width: px(6),
                    }
                    @FeathersScrollbar {
                        @target: #MenuItems,
                    }
                ]
            ]
            --
            @property("Update kind")
            --
            @control()
            Node {
                column_gap: px(1),
            }
            Children [
                #CursorMovedButton @FeathersButton {
                    @caption: bsn! { Text("CursorMoved") ThemedText },
                    @corners: RoundedCorners::Left,
                    @variant: ButtonVariant::Primary,
                }
                update_kind_on(UpdateKind::CursorMoved)
                --
                #CursorStoppedButton @FeathersButton {
                    @caption: bsn! { Text("CursorStopped") ThemedText },
                    @corners: RoundedCorners::None,
                }
                update_kind_on(UpdateKind::CursorStopped)
                --
                #AnimatorCompletedButton @FeathersButton {
                    @caption: bsn! { Text("AnimatorCompleted") ThemedText },
                    @corners: RoundedCorners::Right,
                }
                update_kind_on(UpdateKind::AnimatorCompleted)
            ]
        ]
    }
}
