use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{player, enemies, systems, utils::despawn_with_component, GameState};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_with_component::<OnGameScreen>)

            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())

            .add_plugins(LdtkPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(enemies::EnemyPlugin)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<GroundBundle>(1)
            .add_systems(OnExit(GameState::Game), despawn_with_component::<Wall>)

            .add_systems(Update, systems::delete_bullets.run_if(in_state(GameState::Game)));

    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct GroundBundle {
    wall: Wall,
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
            scaling_mode: ScalingMode::WindowSize(8.0),
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
