use bevy::{
    prelude::*,
    time::{
        Timer,
        TimerMode,
    },
};

#[derive(Component)]
pub struct Player {
}

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
