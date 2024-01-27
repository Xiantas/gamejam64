use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

