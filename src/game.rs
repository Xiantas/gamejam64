use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{physics::collision_layers, player, bullets, utils::despawn_with_component, GameState};

mod exit;
mod collision;
pub mod enemy;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, (game, add_walls_colliders).run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_with_component::<OnGameScreen>)

            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
//            .add_plugins(RapierDebugRenderPlugin::default())

            .add_plugins(LdtkPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(enemy::EnemyPlugin)
            .insert_resource(LevelSelection::index(0))

            .register_ldtk_entity::<exit::ExitBundle>("Exit")
            .add_systems(Update, exit::exit_detection.run_if(in_state(GameState::Game)))

            .add_systems(Update, (
                bullets::deprecate_bullets,
                bullets::bullets_stop_on_wall
            ).run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
struct WallBundle {
    rigidbody: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
    wall: Wall,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            rigidbody: RigidBody::Fixed,
            collider: Collider::cuboid(4.0, 4.0),
            collision_groups: collision_layers::WALL,
            wall: Wall,
        }
    }
}

fn add_walls_colliders(
    mut commands: Commands,
    entity_query: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>,
) {
    for (entity, tile_enum_tags) in entity_query.iter() {
        if tile_enum_tags.tags.contains(&"Wall".into()) && tile_enum_tags.source_enum_uid.is_some() {
            commands.entity(entity).insert(WallBundle::default());
        }
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 1.0,
            near: -1000.0,
            scaling_mode: ScalingMode::FixedVertical(80.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(OnGameScreen);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test_level.ldtk"),
        ..Default::default()
    })
    .insert(OnGameScreen);
}

fn game(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}
