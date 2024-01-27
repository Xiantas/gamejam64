mod components;
mod systems;
mod resources;
mod setups;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, setups::setup_graphics)
        .add_systems(Update, systems::delete_bullets)
        .run();
}
