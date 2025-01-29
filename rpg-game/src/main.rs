use core::{
    entity::{player::warrior::Warrior, weapon::weapon::Weapon},
    services::factory::weapon_factory::RandomWeaponFactory,
};
use std::io;

pub mod core;
fn main() -> io::Result<()> {
    let mut player1: Warrior<W> = Warrior::new("Bulut".to_string(), 5, (0.0, 1.0));
    let mut player2 = Warrior::new("Beyza".to_string(), 1, (0.0, 1.1));

    let swords = RandomWeaponFactory::create_weapons(10, Some("Sword"));

    for sword in swords {
        println!(
            "Sword Name: {} | Rarity: {:?} | Damage: {} | Range: {}",
            sword.get_name(),
            sword.get_rarity(),
            sword.get_attack_damage(),
            sword.get_range()
        );
    }
    Ok(())
}
