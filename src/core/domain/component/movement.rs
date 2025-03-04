use bevy::math::{Vec2};
use bevy::prelude::Component;

#[derive(Component)]
pub struct Position(Vec2);


#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Vision(pub u8);
