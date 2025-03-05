use crate::core::domain::component::combat::AttackPower;
use crate::core::domain::entity::class::{ClassBundle, ClassType};
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res};

pub fn setup_warrior(mut commands: Commands, asset_server: Res<AssetServer>) {
    let warrior_texture = asset_server.load("assets/warrior.png");

    commands.spawn(ClassBundle::new(ClassType::Warrior, warrior_texture));
}

pub fn print_attack_power(query: Query<&AttackPower>) {
    for attack_power in query.iter() {
        println!("Attack Power: {}", attack_power.power);
    }
}
