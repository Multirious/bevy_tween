use super::{EditorData, EditorSettings, Track};
use bevy::prelude::*;
use bevy::ui;

fn ui_plugin(app: &mut App) {
    app.add_systems(Update, window_system);
}

struct Window {
    size: Vec2,
    min_size: Vec2,
}

fn window_system() {}
