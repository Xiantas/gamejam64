use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 4.5,
            speed: 40.0,
        }
    }
}

use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<EnemyBundle>("Enemy")
            .add_systems(Update, enemies_player_rushing.run_if(in_state(GameState::Game)))
            .add_systems(Update, bullet_damage);
    }
}

use crate::{
    bullets::Bullet,
    player::Player,
    physics::collision_layers,
};

use super::collision::ColliderBundle;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    sprite_bundle: SpriteBundle,
    sensor_bundle: ColliderBundle,
    collision_groups: CollisionGroups,
    colliding_entities: CollidingEntities,
    active_events: ActiveEvents,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_groups: collision_layers::ENEMY,
            colliding_entities: CollidingEntities::default(),
            sensor_bundle: ColliderBundle::default(),
            enemy: Enemy::default(),
            sprite_bundle: SpriteBundle::default(),
        }
    }
}

impl LdtkEntity for EnemyBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> EnemyBundle {
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("demon_idle_01.png"),
                sprite: Sprite{
                    // resize the sprite to a center region (on the character) of 16x16 pixels in the texture
                    rect: Some(Rect::new(2., 2., 18., 18.)),
                    // resize the sprite to 8x8 pixels matching the size of the tile
                    custom_size: Some(Vec2::ONE * 32.),
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.125)),
                    ..Default::default()
                },
                ..Default::default()
            },
            sensor_bundle: ColliderBundle::from(entity_instance),
            ..Default::default()
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