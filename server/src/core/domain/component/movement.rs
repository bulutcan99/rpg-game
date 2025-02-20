use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Position {
    location: Vec2,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Vision(pub u8);
