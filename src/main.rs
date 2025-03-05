use bevy::a11y::AccessibilitySystem::Update;
use bevy::app::{App, First, Startup};
use bevy::DefaultPlugins;

use crate::core::domain::command::class::{print_attack_power, setup_warrior};

mod core;
mod error;
#[tokio::main]
async fn main() -> error::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_warrior)
        .add_systems(First, print_attack_power)
        .run();
    Ok(())
}
