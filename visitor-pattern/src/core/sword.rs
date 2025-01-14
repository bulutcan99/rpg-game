use crate::core::game_objects::AttackObject;
use crate::core::player::Player;
use crate::core::weapon::Weapon;
use std::io;

pub struct Sword<'a> {
    pub weapon: Weapon<'a>,
    pub durability: u8,
}

impl<'a> Sword<'a> {
    pub fn new(name: String, player: &'a Player, greatness: u8) -> Sword<'a> {
        Sword {
            weapon: Weapon::new(name, greatness, player),
            durability: 100_u8,
        }
    }
}

impl<'a> AttackObject for Sword<'a> {
    //TODO: her attackta durability azalcak
    fn attack(&self, player: &mut Player) -> io::Result<()> {
        self.weapon.attack(player)
    }
}