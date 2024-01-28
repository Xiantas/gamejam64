use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        let gravity_scale = GravityScale(0.0);

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::ball(4.0),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale,
                ..Default::default()
            },
            "Wall" => ColliderBundle {
                collider: Collider::cuboid(4., 4.),
                rigid_body: RigidBody::Fixed,
                rotation_constraints,
                gravity_scale,
                ..Default::default()
            },
            "Enemy" => ColliderBundle {
                collider: Collider::ball(4.0),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<&EntityInstance> for SensorBundle {
    fn from(entity_instance: &EntityInstance) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        
        match entity_instance.identifier.as_ref() {
            "Exit" => SensorBundle {
                collider: Collider::cuboid(4., 4.),
                sensor: Sensor,
                rotation_constraints,
                active_events: ActiveEvents::COLLISION_EVENTS,
            },
            // Bullet =>
            _ => SensorBundle::default(),
        }
    }
}
