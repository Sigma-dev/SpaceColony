use bevy::{app::{App, Plugin, Startup, Update}, math::Vec2, prelude::{Query, ResMut, Resource, With}, render::camera::{self, Camera}, transform::components::{GlobalTransform, Transform}, window::Window};

#[derive(Resource, Default)]
pub struct MousePosition {
    pub world_position: Vec2,
}

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
        .add_systems(Update, update_mouse_position);
    }
}

fn update_mouse_position(
    mut mouse_position: ResMut<MousePosition>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_position.world_position = world_position;
    }
}