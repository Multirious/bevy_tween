use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Resource)]
pub struct MainCursorWorldCoord(pub Option<Vec2>);

pub fn main_cursor_world_coord_system(
    mut coord: ResMut<MainCursorWorldCoord>,
    q_primary_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_primary_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coord.0 = Some(world_position);
    } else {
        coord.0 = None;
    }
}
