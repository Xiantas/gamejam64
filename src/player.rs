use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{GameState, mouse::MouseInfos, physics::collision_archetypes, game::OnGameScreen, components::Bullet};

#[derive(Default, Component)]
pub struct Player {
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_player)
            .add_systems(Update, (
                move_player,
                shoot,
                sync_player_camera,
            ).run_if(in_state(GameState::Game)));
    }
}

//todo maybe use it
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    rigidbody: RigidBody,
    velocity: Velocity,
    transform_bundle: TransformBundle,
    collider: Collider,
    collision_groups: CollisionGroups,
    gravity_scale: GravityScale,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::default(),
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            transform_bundle: TransformBundle::default(),
            collider: Collider::ball(4.0),
            collision_groups: collision_archetypes::PLAYER,
            gravity_scale: GravityScale(0.0),
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(PlayerBundle::default())
        .insert(OnGameScreen);
}

pub fn move_player(
    keyboard_input: Res<Input<ScanCode>>,
    mut player_velocity: Query<&mut Velocity, With<Player>>
) {

    let Ok(mut player_velocity) = player_velocity.get_single_mut() else { return };

    let y_input: f32 =
        if keyboard_input.pressed(ScanCode(18)) {1.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(32)) {-1.0} else {0.0};
    let x_input: f32 =
        if keyboard_input.pressed(ScanCode(31)) {-1.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(33)) {1.0} else {0.0};

        player_velocity.linvel = 100.0 * Vect{x: x_input, y: y_input}.normalize_or_zero();
}

pub fn shoot(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut mouse: ResMut<MouseInfos>,
) {

    let Ok(player) = player.get_single() else { return };

    if mouse.clicking {
        if let Some(mouse_pos) = mouse.pos {

            let player_pos = player.translation;
            let dir = 60.0*(mouse_pos.xy() - player_pos.xy()).normalize();

            commands
                .spawn((
                    Bullet::default(),
                    RigidBody::Dynamic,
                    Velocity {
                        linvel: Vect {
                            x: dir.x,
                            y: dir.y,
                        },
                        angvel: 0.0,
                    },
                    Collider::ball(2.0),
                    collision_archetypes::BULLET,
                    TransformBundle::from(Transform::from_translation(player_pos)),
                    GravityScale(0.0),
                    Sensor,
                    OnGameScreen,
                ));
        }
        mouse.clicking = false;
    }
}

pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<OrthographicProjection>)>,
) {
    let Ok(player) = player.get_single() else { return };
    let Ok(mut camera_transform) = camera.get_single_mut() else { return };

    camera_transform.translation = player.translation;
}