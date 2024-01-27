use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource, Default)]
pub struct MouseInfos {
    pub pos: Option<Vec3>,
    pub clicking: bool,
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MouseInfos>()
            .add_systems(Update, (
                clicks_handeling,
                mouse_pos_updater,
            ));
    }
}

pub fn mouse_pos_updater(
    mut mouse: ResMut<MouseInfos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    mouse.pos = window.cursor_position()
        .and_then(|cur_pos| camera.viewport_to_world_2d(camera_transform, cur_pos))
        .map(|v2| v2.extend(0.0));
}

pub fn clicks_handeling(
    mut mouse: ResMut<MouseInfos>,
    clicks: Res<Input<MouseButton>>,
) {
    mouse.clicking |= clicks.pressed(MouseButton::Left);
}