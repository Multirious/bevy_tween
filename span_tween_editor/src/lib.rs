use std::any::TypeId;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts,
};
use bevy_tween::{
    prelude::*,
    span_tween::{SpanTweener, TweenTimeSpan},
    tween::TweenerMarker,
};

mod reflect_data;
// use reflect_data::ReflectList;

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
    playhead_drag: f32,
    view_offset: egui::Vec2,
    vertical_scale: f32,
    horizontal_scale: f32,
    // selected_tween: Option<(usize, usize)>,
    // selected_tweens: HashSet<(usize, usize)>,
    tracks: Vec<Track>,
}

impl Default for EditorData {
    fn default() -> Self {
        EditorData {
            playhead_drag: 20.,
            view_offset: egui::Vec2::ZERO,
            vertical_scale: 1.,
            horizontal_scale: 1.,
            // selected_tween: None,
            // selected_tweens: HashSet::default(),
            tracks: Vec::default(),
        }
    }
}

struct Track {
    tweens: HashMap<Entity, TweenTimeSpan>,
    height: f32,
    color: egui::Color32,
}

impl Default for Track {
    fn default() -> Self {
        Track {
            tweens: HashMap::default(),
            height: 10.,
            color: egui::Color32::GRAY,
        }
    }
}

#[derive(Event)]
struct ResetTrack {
    tweener: Entity,
}

fn init_tracks(
    mut reset_track: EventReader<ResetTrack>,
    q_children: &Query<&Children>,
    q_other_tweener: &Query<&TweenerMarker>,
    q_tween: &Query<(&TweenTimeSpan, EntityRef)>,
) {
    for &ResetTrack { tweener } in reset_track.read() {
        let children = q_children.get(tweener).ok();
        let children = children
            .iter()
            .flat_map(|a| a.iter())
            .filter(|c| !q_other_tweener.contains(**c));
        let tweens = q_tween.iter_many([&tweener].into_iter().chain(children));

        let mut track = Track::default();
        for (span, entity_ref) in tweens {
            track.tweens.insert(entity_ref.id(), *span);
        }
    }
}

#[allow(clippy::too_many_arguments)]
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
                        tweens_ui(
                            &mut editor,
                            &mut editor_data,
                            &mut tweener,
                            ui,
                        );
                    }
                    None => {
                        commands.entity(tweener_entity).insert(EditorData {
                            ..Default::default()
                        });
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

fn tweens_ui(
    editor: &mut EditorSetting,
    editor_data: &mut EditorData,
    tweener: &mut SpanTweener,
    ui: &mut egui::Ui,
) {
    egui::CentralPanel::default()
        .frame(
            egui::Frame::central_panel(ui.style())
                .fill(ui.style().visuals.widgets.open.weak_bg_fill),
        )
        .show_inside(ui, |ui| {
            let mut now = tweener.timer.elasped().now;
            TweensUi {
                playhead: &mut now,
                length: tweener.timer.length.as_secs_f32(),
                editor_data,
            }
            .ui(ui);
        });
}

struct TweensUi<'a> {
    playhead: &'a mut f32,
    length: f32,
    editor_data: &'a mut EditorData,
}

impl<'a> TweensUi<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let TweensUi {
            playhead,
            length,
            editor_data,
        } = self;
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            const SCALE: f32 = 100.;
            const HEIGHT: f32 = 15.;

            let response = ui.allocate_response(
                egui::Vec2::new(length * SCALE, HEIGHT),
                egui::Sense {
                    click: true,
                    drag: true,
                    focusable: false,
                },
            );

            let rect = response.rect;

            let playhead_x = rect.left() + *playhead * SCALE;

            timeline(rect.min, HEIGHT, length, SCALE, ui);

            for track_ in &editor_data.tracks {
                let response = ui.allocate_response(
                    egui::vec2(length * SCALE, track_.height),
                    egui::Sense {
                        click: true,
                        drag: true,
                        focusable: false,
                    },
                );
                let rect = response.rect;
                track(rect.min, length, SCALE, track_, ui);
            }

            ui.painter().line_segment(
                [
                    egui::pos2(playhead_x, rect.top()),
                    egui::pos2(playhead_x, rect.bottom()),
                ],
                (1., egui::Color32::WHITE),
            );

            response
        })
        .response
    }
}

fn timeline(
    pos: egui::Pos2,
    height: f32,
    length: f32,
    scale: f32,
    ui: &mut egui::Ui,
) {
    ui.painter().rect_filled(
        egui::Rect::from_min_max(pos, pos + egui::vec2(length * scale, height)),
        0.,
        ui.style().visuals.widgets.noninteractive.bg_fill,
    );

    let max_tick = (length * 8.).ceil() as i32;

    for i in 0..max_tick {
        let (brightness, shortness) = match i % 8 {
            0 => (130, height * 0.8),
            4 => (120, height * 0.6),
            2 | 6 => (110, height * 0.5),
            1 | 3 | 5 | 7 => (100, height * 0.45),
            _ => unreachable!(),
        };
        let tick_x = pos.x + (i as f32 / 8.) * scale;
        ui.painter().line_segment(
            [
                egui::pos2(tick_x, pos.y),
                egui::pos2(tick_x, height - shortness),
            ],
            (1., egui::Color32::from_gray(brightness)),
        );
    }
}

fn track(
    pos: egui::Pos2,
    length: f32,
    scale: f32,
    track: &Track,
    ui: &mut egui::Ui,
) {
    ui.painter().rect_filled(
        egui::Rect::from_min_max(
            pos,
            pos + egui::Vec2::new(length * scale, track.height),
        ),
        0.,
        egui::Color32::WHITE,
    );
    for span in track.tweens.values() {
        let min = span.min().duration().as_secs_f32();
        let max = span.max().duration().as_secs_f32();
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(min * scale, pos.y),
                egui::pos2(max * scale, pos.y + track.height),
            ),
            0.,
            track.color,
        );
    }
}
