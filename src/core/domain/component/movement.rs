use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Vision(pub u8);

#[derive(Component)]
pub struct Movement {
    pub position: Position,
    pub velocity: Velocity,
    pub vision: Vision,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            position: Position(Vec2::new(0.0, 0.0)),
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            vision: Vision(10),
        }
    }
}
