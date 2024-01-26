use bevy::{
    prelude::*,
    window::PrimaryWindow,
    input::mouse::MouseMotion,
};

use bevy_rapier2d::prelude::*;

use crate::{
    resources::MouseInfos,
    components::{
        Player, Bullet,
    },
};

pub fn move_player(
    keyboard_input: Res<Input<ScanCode>>,
    mut movements: Query<&mut Velocity, With<Player>>
) {
    let mut movements = movements.single_mut();
    let y_input: f32 =
        if keyboard_input.pressed(ScanCode(18)) {200.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(32)) {-200.0} else {0.0};
    let x_input: f32 =
        if keyboard_input.pressed(ScanCode(31)) {-200.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(33)) {200.0} else {0.0};

    movements.linvel = Vect{x: x_input, y: y_input};
}

pub fn shoot(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut mouse: ResMut<MouseInfos>,
) {
    if mouse.clicking {
        if let Some(mouse_pos) = mouse.pos {
            let player_pos = player.single().translation;
            let dir = 100.0*(mouse_pos.xy() - player_pos.xy()).normalize();

            commands
                .spawn(Bullet::default())
                .insert(RigidBody::Dynamic)
                .insert(Velocity {
                    linvel: Vect {
                        x: dir.x,
                        y: dir.y,
                    },
                    angvel: 0.0,
                })
                .insert(Collider::ball(30.0))
                .insert(CollisionGroups::new(
                    Group::GROUP_2,
                    Group::GROUP_3))
                .insert(TransformBundle::from(Transform::from_translation(player_pos)))
                .insert(GravityScale(0.0));
        }
        mouse.clicking = false;
    }
}

pub fn delete_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.time_to_live.tick(time.delta());
        if bullet.time_to_live.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn clicks_handeling(
    mut mouse: ResMut<MouseInfos>,
    clicks: Res<Input<MouseButton>>,
) {
    mouse.clicking |= clicks.pressed(MouseButton::Left)
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
