pub mod data;
pub mod logic;

use bevy::prelude::*;

use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), logic::enemy_setup)
            .add_systems(Update, logic::enemies_player_rushing.run_if(in_state(GameState::Game)))
            .add_systems(Update, logic::bullet_damage);
    }
}
