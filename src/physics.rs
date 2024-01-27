pub mod collision_archetypes {
    use bevy_rapier2d::prelude::*;

    pub const PLAYER: CollisionGroups = CollisionGroups::new(
        Group::from_bits_retain(0x30000000),
        Group::from_bits_retain(0x30000000));
    pub const BULLET: CollisionGroups = CollisionGroups::new(
        Group::from_bits_retain(0xc0000000),
        Group::from_bits_retain(0x30000000));
    pub const ENEMY: CollisionGroups = CollisionGroups::new(
        Group::from_bits_retain(0xf0000000),
        Group::from_bits_retain(0xf0000000));
}
