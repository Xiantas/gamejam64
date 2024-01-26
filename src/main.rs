mod components;
mod systems;
mod resources;
mod setups;

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
        .init_resource::<resources::MouseInfos>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setups::setup_graphics, setups::setup_player))
        .add_systems(Update, (
            systems::move_player,
            systems::shoot,
            systems::clicks_handeling,
            systems::mouse_pos_updater,
            systems::delete_bullets))
        .run();
}
