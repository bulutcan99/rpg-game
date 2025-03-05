use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component};

use crate::core::domain::entity::class::ClassType;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Vision(pub u8);

#[derive(Bundle)]
pub struct ClassMovementBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub vision: Vision,
}

impl ClassMovementBundle {
    pub fn new(class: ClassType) -> Self {
        match class {
            ClassType::Warrior => Self {
                position: Position(Vec2::new(0.0, 0.0)),
                velocity: Velocity(Vec2::new(3.0, 3.0)),
                vision: Vision(10),
            },
            ClassType::Rogue => Self {
                position: Position(Vec2::new(5.0, 5.0)),
                velocity: Velocity(Vec2::new(5.0, 5.0)),
                vision: Vision(12),
            },
            ClassType::Archer => Self {
                position: Position(Vec2::new(-5.0, -5.0)),
                velocity: Velocity(Vec2::new(4.0, 4.0)),
                vision: Vision(15),
            },
        }
    }
}
