use crate::prelude::*;

#[derive(Component)]
pub struct Asteroid(pub asteroid::AsteroidSize);

#[derive(Component)]
pub struct PlayerShip;

#[derive(Component)]
pub struct Gun {
    pub last_fired: Instant,
    pub cooldown: Duration
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct LifeTime{
    pub timer: Timer,
}

#[derive(Component)]
pub struct Health{
    pub value: u32
}