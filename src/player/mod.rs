mod components;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    resources,
    player::{
        components::*,
        systems::*,
    }
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                move_player,
                shoot,
                clicks_handeling,
                mouse_pos_updater,
            ))
            .init_resource::<resources::MouseInfos>();
    }
}

//todo maybe use it
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    rigidbody: RigidBody,
    velocity: Velocity,
    transform_bundle: TransformBundle,
    collider: Collider,
    collision_groups: CollisionGroups,
    gravity_scale: GravityScale,
}
