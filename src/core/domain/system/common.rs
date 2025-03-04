use crate::core::domain::component::movement::Velocity;
use bevy::prelude::{Query, Res, Time, Transform};

pub fn movement(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    todo!()
}
