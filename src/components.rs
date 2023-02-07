use crate::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct LifeTime{
    pub timer: Timer,
}

#[derive(Component)]
pub struct Health{
    pub value: i32
}