use core::{
    entity::{
        player::{self, player::Player, warrior::Warrior},
        weapon::weapon::Weapon,
    },
    services::factory::weapon_factory::RandomWeaponFactory,
};
use std::io;

pub mod core;
fn main() -> io::Result<()> {
    let mut player1 = Warrior::new("Bulut".to_string(), 5, (0.0, 1.0));
    let mut player2 = Warrior::new("Beyza".to_string(), 1, (0.0, 1.1));

    let mut swords = RandomWeaponFactory::create_weapons(10, None);

    for sword in swords.iter() {
        println!(
            "Sword Name: {} | Rarity: {:?} | Damage: {} | Range: {}",
            sword.get_name(),
            sword.get_rarity(),
            sword.get_attack_damage(),
            sword.get_range()
        );
    }

    let sword = swords.remove(0);

    player1.set_weapon(sword).unwrap();
    println!("PLAYER: {:#?}", player1);
    player1.strike(Box::new(player2));
    Ok(())
}
