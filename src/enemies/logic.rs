use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::data::Enemy;
use crate::{
    components::Bullet,
    player::Player,
    physics::collision_archetypes,
};

pub fn enemies_player_rushing(
    mut enemies: Query<(&mut Velocity, &Transform, &Enemy)>,
    player: Query<&Transform, With<Player>>,
) {
    let player = player.single();
    for (mut v, t, e) in &mut enemies {
        let dir = (player.translation - t.translation).xy().normalize_or_zero();
        v.linvel = e.speed * Vect{x: dir.x, y: dir.y};
    }
}

pub fn bullet_damage(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy, &CollidingEntities)>,
    bullets: Query<(Entity, &Bullet)>,
) {
    for (e_entity, mut enemy, collisions) in &mut enemies {
        for (b_entity, bullet) in &bullets {
            if collisions.contains(b_entity) {
                enemy.health -= bullet.damage;
                commands.entity(b_entity).despawn();
                if enemy.health < 0.0 {
                    commands.entity(e_entity).despawn();
                }
            }
        }
    }
}

pub fn enemy_setup(
    mut commands: Commands,
) {
    for i in 0..3 {
        // TODO: use bundle and default values
        commands.spawn((
            Enemy {
                health: 50.0,
                speed: 60.0
            },
            TransformBundle::from(
                Transform::from_xyz(0.0, (i+1) as f32 * 200.0, 0.0)
            ),
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(3.0),
            collision_archetypes::ENEMY,
            GravityScale(0.0),
            CollidingEntities::default(),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}
