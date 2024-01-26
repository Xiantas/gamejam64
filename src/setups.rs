use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Player;

pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn(Player{})
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Collider::ball(50.0))
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_1))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(GravityScale(0.0));
}
