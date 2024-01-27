pub mod collision_layers {
    use bevy_rapier2d::prelude::*;

    use Group as G;

    // GROUP_1 : Objects that can collide with other objects of the same group.
    // GROUP_2 : Bullets
    // GROUP_3:  Objects that can collide with walls
    // GROUP_4 : Moving entities

    pub const PLAYER: CollisionGroups = CollisionGroups::new(
        G::GROUP_4,
        G::GROUP_3.union(G::GROUP_4)
    );
    pub const BULLET: CollisionGroups = CollisionGroups::new(
        G::GROUP_2,
        G::GROUP_3.union(G::GROUP_4)
    );
    pub const ENEMY: CollisionGroups = CollisionGroups::new(
        G::GROUP_1.union(G::GROUP_4),
        G::ALL
    );
    pub const WALL: CollisionGroups = CollisionGroups::new(
        G::GROUP_3,
        G::ALL
    );
}
