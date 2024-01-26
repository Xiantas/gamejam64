use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MouseInfos {
    pub pos: Option<Vec3>,
    pub clicking: bool,
}
