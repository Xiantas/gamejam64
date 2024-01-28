use std::time::Duration;

use bevy::{
    prelude::*,
    render::view::{
        InheritedVisibility,
        ViewVisibility,
    },
    time::{
        Timer,
        TimerMode,
    },
};
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    GameState,
    mouse::MouseInfos,
    physics::collision_layers,
    game::{
        OnGameScreen,
        enemy::Enemy,
    },
    bullets::{
        BulletBundle,
    },
};

#[derive(Default, Component)]
pub struct Player {
    pub speed: f32,
    pub reload: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_player)
            .add_systems(Update, (
                set_player_position_from_ldtk_entity,
                move_player,
                death,
                shoot,
                sync_player_camera,
            ).run_if(in_state(GameState::Game)));
    }
}

//todo maybe use it
#[derive(Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    rigidbody: RigidBody,
    velocity: Velocity,
    transform_bundle: TransformBundle,
    collider: Collider,
    collision_groups: CollisionGroups,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    sprite: Sprite,
    texture: Handle<Image>,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,

    active_events: ActiveEvents,
    colliding_entities: CollidingEntities,
}

fn set_player_position_from_ldtk_entity(
    entity_query: Query<&EntityInstance, Added<EntityInstance>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut player_transform) = player_transform.get_single_mut() else { return };
    
    for entity_instance in entity_query.iter() {
        if entity_instance.identifier == "Player" {
            player_transform.translation.x = entity_instance.grid.x as f32 * 8.0;
            // Fix the y position because the y axis is inverted in ldtk
            player_transform.translation.y = (31-entity_instance.grid.y) as f32 * 8.0;
            return;
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn(PlayerBundle{
            texture: asset_server.load("wizard_red_staff_idle_01.png"),
            sprite: Sprite{
                // resize the sprite to a center region (on the character) of 16x16 pixels in the texture
                rect: Some(Rect::new(2., 2., 18., 18.)),
                // resize the sprite to 8x8 pixels matching the size of the tile
                custom_size: Some(Vec2::ONE * 16.),
                anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.125)),
                ..Sprite::default()
            },
            // 5 is the z-index of the player to b on top of the tiles
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(Vec2::ZERO.extend(5.0))),
            locked_axes: LockedAxes::ROTATION_LOCKED_Z,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(2.5),
            collision_groups: collision_layers::PLAYER,
            gravity_scale: GravityScale(0.0),
            player: Player {
                speed: 45.0,
                reload: Timer::new(Duration::from_secs_f32(0.25), TimerMode::Once),
            },
            active_events: ActiveEvents::COLLISION_EVENTS,
            ..PlayerBundle::default()
        })
        .insert(OnGameScreen);
}

pub fn move_player(
    keyboard_input: Res<Input<ScanCode>>,
    mut player: Query<(&mut Velocity, &Player)>
) {

    let Ok((mut player_velocity, player)) = player.get_single_mut() else { return };

    let y_input: f32 =
        if keyboard_input.pressed(ScanCode(18)) {1.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(32)) {-1.0} else {0.0};
    let x_input: f32 =
        if keyboard_input.pressed(ScanCode(31)) {-1.0} else {0.0} +
        if keyboard_input.pressed(ScanCode(33)) {1.0} else {0.0};

        player_velocity.linvel = player.speed * Vect{x: x_input, y: y_input}.normalize_or_zero();
}

pub fn shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut player: Query<(&Transform, &mut Player)>,
    mut mouse: ResMut<MouseInfos>,
) {

    let Ok((player, mut p)) = player.get_single_mut() else { return };

    if p.reload.tick(time.delta()).finished() && mouse.clicking {
        p.reload.reset();
        if let Some(mouse_pos) = mouse.pos {

            let player_pos = player.translation;
            let dir = 100.0*(mouse_pos.xy() - player_pos.xy()).normalize();

            commands.spawn(
                BulletBundle {
                    velocity: Velocity {
                        linvel: Vect {x: dir.x, y: dir.y},
                        ..default()
                    },
                    transform_bundle: TransformBundle::from(
                        Transform::from_translation(player_pos)),
                    texture: asset_server.load("gem_yellow.png"),
                    ..default()
                })
                .insert(OnGameScreen);
        }
    }
    mouse.clicking = false;
}

pub fn death(
    player: Query<&CollidingEntities, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level: ResMut<LevelSelection>,
) {
    for c in &player {
        for e in &enemies {
            if c.contains(e) {
                *level = LevelSelection::index(0);
                game_state.set(GameState::Menu);
            }
        }
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

