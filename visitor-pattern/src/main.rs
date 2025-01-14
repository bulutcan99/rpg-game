use crate::core::game_objects::AttackObject;
use crate::core::item::Sword;
use crate::core::player::Player;
use std::io;

pub mod core;
fn main() -> io::Result<()> {
    let mut player1 = Player::new("Bulut".to_string(), 5, (0.0, 1.0));
    let mut player2 = Player::new("Beyza".to_string(), 1, (0.0, 1.1));
    let sword = Sword::new("Rare".to_string(), &player1, 3);
    sword.attack(&mut player2)?;

    println!("USER HP: {:?}", player2.get_health());
    Ok(())
}