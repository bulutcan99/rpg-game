use crate::core::domain::entity::player::Player;
use crate::core::domain::system::user::command::spawn_user;
use crate::core::domain::system::user::render::{execute_animations, trigger_animation};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

mod core;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, spawn_user)
        .add_systems(Update, execute_animations)
        .add_systems(Update, trigger_animation)
        .run();
    Ok(())
}
