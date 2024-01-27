use bevy::{
    prelude::*,
    time::{
        Timer,
        TimerMode,
    },
};

#[derive(Component, Clone)]
pub struct Bullet {
    pub time_to_live: Timer,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            time_to_live: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub speed: f32,
}
