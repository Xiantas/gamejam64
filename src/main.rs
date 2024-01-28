mod bullets;
mod player;
mod ui;
mod utils;
mod game;
mod enemies;
mod physics;
mod mouse;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F3)))
        .add_state::<GameState>()
        .add_plugins(mouse::MousePlugin)
        .add_plugins((ui::splash::SplashPlugin, ui::menu::MenuPlugin, game::GamePlugin))
        .run();
}
