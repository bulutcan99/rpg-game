use crate::core::player::Player;
use crate::core::weapon::Weapon;

pub struct Sword<'a> {
    pub weapon: Weapon<'a>,
    pub durability: u8,
}

impl<'a> Sword<'a> {
    pub fn new(name: String, player: &'a Player, rarity: u8) -> Sword<'a> {
        Sword {
            weapon: Weapon::new(name, rarity),
            durability: 100_u8,
        }
    }
}
