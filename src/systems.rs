use bevy::{
    prelude::*,
};

use bevy_rapier2d::prelude::*;

use crate::{
    resources::MouseInfos,
    components::Bullet,
};

pub fn delete_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.time_to_live.tick(time.delta());
        if bullet.time_to_live.finished() {
            commands.entity(entity).despawn();
        }
    }
}
