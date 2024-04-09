use bevy::{prelude::*, utils::HashSet};
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts,
};
use bevy_tween::{
    prelude::*,
    span_tween::{SpanTweener, TweenTimeSpan},
};

mod reflect_data;

pub struct SpanTweenEditorPlugin;

impl Plugin for SpanTweenEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, editor_system)
            .init_resource::<EditorSetting>();
    }
}

#[derive(Default, Resource)]
struct EditorSetting {
    tweener: Option<Entity>,
}

#[derive(Component)]
struct EditorData {
    vertical_scale: f32,
    horizontal_scale: f32,
    selected_tween: Option<usize>,
    selected_tweens: HashSet<usize>,
    tweens: Vec<(TweenTimeSpan, Entity)>,
}

impl Default for EditorData {
    fn default() -> Self {
        EditorData {
            vertical_scale: 1.,
            horizontal_scale: 1.,
            selected_tween: None,
            selected_tweens: HashSet::default(),
            tweens: Vec::new(),
        }
    }
}

fn editor_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut editor: ResMut<EditorSetting>,
    q_tweener_entity: Query<Entity, With<SpanTweener>>,
    mut q_tweener: Query<(&mut SpanTweener, Option<&mut EditorData>)>,
    q_name: Query<&Name>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Span tween editor")
        .resizable(true)
        .hscroll(true)
        .vscroll(true)
        .show(ctx, |ui| {
            editor_setting(&mut editor, &q_name, &q_tweener_entity, ui);

            let tweener = editor
                .tweener
                .and_then(|e| q_tweener.get_mut(e).ok().map(|q| (e, q)));
            if let Some((tweener_entity, (mut tweener, editor_data))) = tweener
            {
                timer_setting(&mut tweener, ui);
                match editor_data {
                    Some(mut editor_data) => {
                        timing(&mut editor, &mut editor_data, &mut tweener, ui);
                    }
                    None => {
                        commands
                            .entity(tweener_entity)
                            .insert(EditorData::default());
                    }
                }
            }
        });
}

fn editor_setting(
    editor: &mut EditorSetting,
    q_name: &Query<&Name>,
    q_tweener_entity: &Query<Entity, With<SpanTweener>>,
    ui: &mut egui::Ui,
) {
    egui::SidePanel::left("editor_setting").show_inside(ui, |ui| {
        egui::ComboBox::from_id_source("select_tweener")
            .selected_text({
                match editor.tweener {
                    Some(tweener) => match q_name.get(tweener) {
                        Ok(name) => format!("{name}"),
                        Err(_) => format!("{tweener:?}"),
                    },
                    None => "None".to_string(),
                }
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut editor.tweener, None, "None");
                q_tweener_entity.iter().for_each(|tweener| {
                    ui.selectable_value(
                        &mut editor.tweener,
                        Some(tweener),
                        q_name
                            .get(tweener)
                            .map(|name| format!("{name}"))
                            .unwrap_or_else(|_| format!("{tweener:?}")),
                    );
                })
            });
    });
}

fn timer_setting(tweener: &mut SpanTweener, ui: &mut egui::Ui) {
    egui::TopBottomPanel::top("timeer_settings").show_inside(ui, |ui| {
        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label("Enabled:");
                let mut unpaused = !tweener.timer.paused;
                ui.add(egui::Checkbox::without_text(&mut unpaused));
                tweener.timer.paused = !unpaused;

                ui.label("Repeat:");
                let mut enable_repeat = tweener.timer.repeat.is_some();
                ui.add(egui::Checkbox::without_text(&mut enable_repeat));
                match (tweener.timer.repeat.is_some(), enable_repeat) {
                    (false, true) => {
                        tweener.timer.set_repeat(Some((
                            Repeat::Infinitely,
                            RepeatStyle::WrapAround,
                        )));
                    }
                    (true, false) => {
                        tweener.timer.set_repeat(None);
                    }
                    _ => {}
                }
            },
        );
    });
}

fn timing(
    editor: &mut EditorSetting,
    editor_data: &mut EditorData,
    tweener: &mut SpanTweener,
    ui: &mut egui::Ui,
) {
    egui::TopBottomPanel::top("timing").show_inside(ui, |ui| {
        let mut slider_now = tweener.timer.elasped().now;
        let timer_length = tweener.timer.length.as_secs_f32();
        ui.add(
            egui::Slider::new(&mut slider_now, (0.)..=timer_length)
                .handle_shape(egui::style::HandleShape::Rect {
                    aspect_ratio: 0.1,
                }),
        );
        if slider_now != tweener.timer.elasped().now {
            tweener.timer.set_tick(slider_now);
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add(TweensUi {
                now: &mut 0.,
                editor_data,
            });
        });
    });
}

struct TweensUi<'a> {
    now: &'a mut f32,
    editor_data: &'a mut EditorData,
}

impl<'a> Widget for TweensUi<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let TweensUi { now, editor_data } = self;

        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Min),
            |ui| {
                let response = ui.allocate_response(
                    egui::Vec2::new(10., 10.),
                    egui::Sense {
                        click: true,
                        drag: true,
                        focusable: false,
                    },
                );
                let timing_head_x = 0.;
                ui.painter().rect(egui::Rect {
                    min: ,
                    max: 
                }, , , )
                response
            },
        );

        let response = ui.allocate_response(
            ui.available_size(),
            egui::Sense {
                click: true,
                drag: true,
                focusable: false,
            },
        );
        if response.hovered() {
            ui.painter().rect(
                response.rect.with_max_x(response.rect.max.x - 10.),
                0.,
                egui::Color32::WHITE,
                (0., egui::Color32::WHITE),
            );
        } else {
            ui.painter().rect(
                response.rect,
                0.,
                egui::Color32::WHITE,
                (0., egui::Color32::WHITE),
            );
        }
        response
    }
}
