use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{GameState, mouse::MouseInfos, physics::collision_archetypes, game::OnGameScreen, components::Bullet};

#[derive(Component)]
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

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(Player{})
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Collider::ball(8.0))
        .insert(collision_archetypes::PLAYER)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(GravityScale(0.0))
        .insert(OnGameScreen);
}

pub fn move_player(
    keyboard_input: Res<Input<ScanCode>>,
    mut movements: Query<&mut Velocity, With<Player>>
) {
    let mut movements = movements.single_mut();
    let y_input: f32 =
        if keyboard_input.pressed(ScanCode(18)) {1.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(32)) {-1.0} else {0.0};
    let x_input: f32 =
        if keyboard_input.pressed(ScanCode(31)) {-1.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(33)) {1.0} else {0.0};

    movements.linvel = 100.0 * Vect{x: x_input, y: y_input}.normalize_or_zero();
}

pub fn shoot(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut mouse: ResMut<MouseInfos>,
) {
    if mouse.clicking {
        if let Some(mouse_pos) = mouse.pos {
            let player_pos = player.single().translation;
            let dir = 500.0*(mouse_pos.xy() - player_pos.xy()).normalize();

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
                    Collider::ball(4.0),
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