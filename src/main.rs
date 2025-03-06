use crate::core::domain::entity::class::{ClassBundle, ClassType};
use crate::core::domain::system::command::spawn_user;
use crate::core::domain::system::render::animate_sprite;
use bevy::prelude::*;

mod core;
mod error;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, spawn_user)
        .add_systems(Update, animate_sprite)
        .run();
}
