use bevy::{
    prelude::*,
    time::{
        Timer,
        TimerMode,
    },
};
use bevy_rapier2d::prelude::*;

use crate::{
    physics::collision_layers,
    game::{
        Wall,
        enemy::Enemy,
    },
};

#[derive(Component, Clone)]
pub struct Bullet {
    pub time_to_live: Timer,
    pub damage: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            damage: 1.0,
            time_to_live: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub transform_bundle: TransformBundle,
    pub gravity_scale: GravityScale,
    pub sensor: Sensor,
    pub colliding_entities: CollidingEntities,
    pub active_events: ActiveEvents,
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for BulletBundle {
    fn default() -> Self {
        BulletBundle {
            bullet: Bullet::default(),
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: Collider::ball(2.0),
            collision_groups: collision_layers::BULLET,
            transform_bundle: TransformBundle::default(),
            gravity_scale: GravityScale(0.0),
            sensor: Sensor::default(),
            colliding_entities: CollidingEntities::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            sprite: Sprite::default(),
            texture: Handle::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

pub fn deprecate_bullets(
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

pub fn bullets_stop_on_wall(
    mut commands: Commands,
    bullets: Query<(Entity, &CollidingEntities), With<Bullet>>,
    walls: Query<Entity, With<Wall>>,
) {
    for (e, ce) in &bullets {
        for w in &walls {
            if ce.contains(w) {
                commands.entity(e).despawn();
                break;
            }
        }
    }
}

pub fn bullet_damage(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy)>,
    bullets: Query<(Entity, &Bullet, &CollidingEntities)>,
) {
    for (b_entity, bullet, collisions) in &bullets {
        for (e_entity, mut enemy) in &mut enemies {
            if collisions.contains(e_entity) {
                enemy.health -= bullet.damage;
                enemy.track_player = true;
                commands.entity(b_entity).despawn();
                if enemy.health < 0.0 {
                    commands.entity(e_entity).despawn();
                }
            }
        }
    }
}
