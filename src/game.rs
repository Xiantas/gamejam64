use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player, enemies, systems, utils::despawn_with_component, GameState};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_with_component::<OnGameScreen>)

            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())

            .add_plugins(player::PlayerPlugin)
            .add_plugins(enemies::EnemyPlugin)

            .add_systems(Update, systems::delete_bullets.run_if(in_state(GameState::Game)));
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

fn game_setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 0.5,
            ..Default::default()
        },
        ..Default::default()
    }).insert(OnGameScreen);
}

fn game(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}
