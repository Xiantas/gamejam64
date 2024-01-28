use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), enemy_setup)
            .add_systems(Update, enemies_player_rushing.run_if(in_state(GameState::Game)))
            .add_systems(Update, bullet_damage);
    }
}

use crate::{
    bullets::Bullet,
    player::Player,
    physics::collision_layers,
};

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    transform_bundle: TransformBundle,
    rigidbody: RigidBody,
    velocity: Velocity,
    collider: Collider,
    collision_groups: CollisionGroups,
    gravity_scale: GravityScale,
    colliding_entities: CollidingEntities,
    active_events: ActiveEvents,
    locked_axes: LockedAxes,
    sprite: Sprite,
    texture: Handle<Image>,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            enemy: Enemy {
                health: 4.5,
                speed: 40.0,
            },
            transform_bundle: TransformBundle::default(),
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: Collider::ball(3.0),
            collision_groups: collision_layers::ENEMY,
            gravity_scale: GravityScale(0.0),
            colliding_entities: CollidingEntities::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::ROTATION_LOCKED_Z,
            sprite: Sprite::default(),
            texture: Handle::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

pub fn enemies_player_rushing(
    mut enemies: Query<(&mut Velocity, &Transform, &Enemy)>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player.get_single() else { return };

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
    asset_server: Res<AssetServer>,
) {
    commands.spawn(EnemyBundle {
        transform_bundle: TransformBundle::from(
              Transform::from_xyz(30.0, 30.0, 5.0)),
              ..default()
    });
}
