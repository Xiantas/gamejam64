use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;

use super::collision::SensorBundle;

#[derive(Default, Component)]
pub struct Exit;

#[derive(Default, Bundle, LdtkEntity)]
pub struct ExitBundle {
    pub exit: Exit,
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
}

pub fn exit_detection(
    mut level: ResMut<LevelSelection>,
    player: Query<&CollidingEntities, With<Player>>,
    exit: Query<Entity, With<Exit>>,
) {
    let Ok(player_collisions) = player.get_single() else { return };
    let Ok(exit) = exit.get_single() else { return };

    if player_collisions.contains(exit) {
        if let LevelSelection::Indices (LevelIndices { level: level_id, .. }) = *level {
            *level = LevelSelection::index(level_id + 1);
        }
    }
}