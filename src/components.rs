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
    pub damage: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            damage: 1.0,
            time_to_live: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}
