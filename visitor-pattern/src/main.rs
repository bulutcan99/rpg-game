use crate::core::player::Player;
use crate::core::sword::Sword;
use core::{
    weapon::{self, AttackObject},
    weapon_factory::{self, WeaponFactory},
};
use std::io;

pub mod core;
fn main() -> io::Result<()> {
    let mut player1 = Player::new("Bulut".to_string(), 5, (0.0, 1.0));
    let mut player2 = Player::new("Beyza".to_string(), 1, (0.0, 1.1));

    let weapon_factory = WeaponFactory;
    let weapon = weapon_factory.create_weapon("Greatness".to_string(), 4);
    if let Some(sword) = weapon.downcast_ref::<Sword>() {
        sword.set_equipped_by(&player1); // player1 ile silahı ilişkilendiriyoruz
        println!("USER HP: {:?}", player2.get_health()); // player2'nin sağlık durumu yazdırılıyor
    } else {
        println!("Failed to cast to Sword");
    }

    Ok(())
}
