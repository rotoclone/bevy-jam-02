use bevy::render::camera::RenderTarget;

use crate::*;

pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition(None))
            .insert_resource(WindowDimensions(Vec2::ZERO))
            .add_system_to_stage(CoreStage::PreUpdate, cursor_position_system);
    }
}

pub struct CursorPosition(pub Option<Vec2>);

pub struct WindowDimensions(pub Vec2);

#[derive(Component)]
pub struct MainCamera;

/// Updates the game's `CursorPosition`
/// From https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn cursor_position_system(
    windows: Res<Windows>,
    mut cursor_position: ResMut<CursorPosition>,
    mut window_dimensions: ResMut<WindowDimensions>,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = query.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    window_dimensions.0.x = wnd.width();
    window_dimensions.0.y = wnd.height();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        cursor_position.0 = Some(world_pos);
    } else {
        cursor_position.0 = None;
    }
}
