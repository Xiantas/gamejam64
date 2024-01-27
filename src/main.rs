mod components;
mod systems;
mod resources;
mod setups;
mod player;
mod ui;
mod utils;
mod game;

use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("gamejam64"),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_systems(Startup, setups::setup_graphics)
        .add_plugins((ui::splash::SplashPlugin, ui::menu::MenuPlugin, game::GamePlugin))
        .run();
}