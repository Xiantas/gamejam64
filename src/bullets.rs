use bevy::{
    prelude::*,
    time::{
        Timer,
        TimerMode,
    },
};
use bevy_rapier2d::prelude::*;

use crate::physics::collision_layers;

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
