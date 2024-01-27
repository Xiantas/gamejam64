use bevy::prelude::*;

use crate::components::Bullet;

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
