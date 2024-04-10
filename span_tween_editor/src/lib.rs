use bevy::{prelude::*, utils::HashMap};
use bevy_tween::span_tween::TweenTimeSpan;

mod ui;

pub struct SpanTweenEditorPlugin;

impl Plugin for SpanTweenEditorPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Resource)]
pub struct EditorSettings {
    tweener: Option<Entity>,
}

#[derive(Component)]
pub struct EditorData {
    scale: Vec2,
    tracks: Vec<Track>,
}

pub struct Track {
    color: Color,
    tweens: HashMap<Entity, TweenTimeSpan>,
}
