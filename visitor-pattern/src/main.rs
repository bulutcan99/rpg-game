use crate::core::player::Player;
use crate::core::sword::Sword;
use core::weapon_factory::{BowFactory, SpearFactory, SwordFactory, WeaponFactory};
use std::io;

pub mod core;
fn main() -> io::Result<()> {
    let mut player1 = Player::new("Bulut".to_string(), 5, (0.0, 1.0));
    let mut player2 = Player::new("Beyza".to_string(), 1, (0.0, 1.1));

    let sword_factory: Box<dyn WeaponFactory> = Box::new(SwordFactory);
    let bow_factory: Box<dyn WeaponFactory> = Box::new(BowFactory);
    let spear_factory: Box<dyn WeaponFactory> = Box::new(SpearFactory);

    // Silahları üretmek
    let sword = sword_factory.create_weapon();
    let bow = bow_factory.create_weapon();
    let spear = spear_factory.create_weapon();

    // Silahların bilgilerini yazdırmak
    println!(
        "Weapon: {}, Rarity: {}, Price: {} Gold, Attack: {}",
        sword.clone().name(),
        sword.rarity(),
        sword.price(),
        sword.attack()
    );
    println!(
        "Weapon: {}, Rarity: {}, Price: {} Gold, Attack: {}",
        bow.name(),
        bow.rarity(),
        bow.price(),
        bow.attack()
    );
    println!(
        "Weapon: {}, Rarity: {}, Price: {} Gold, Attack: {}",
        spear.name(),
        spear.rarity(),
        spear.price(),
        spear.attack()
    );

    sword.
    Ok(())
}
