use crate::core::warrior::Player;
use crate::core::sword::Sword;
use core::weapon_factory::{MeleeWeaponFactory, RangedWeaponFactory, WeaponFactory};
use std::io;

pub mod core;
fn main() -> io::Result<()> {
    let mut player1 = Player::new("Bulut".to_string(), 5, (0.0, 1.0));
    let mut player2 = Player::new("Beyza".to_string(), 1, (0.0, 1.1));

    let melee_factory = MeleeWeaponFactory;
    let ranged_factory = RangedWeaponFactory;

    let sword = melee_factory.create_weapon();
    let _bow = ranged_factory.create_weapon();

    println!("ATTACKED {:?}", sword.attack());
    Ok(())
}
