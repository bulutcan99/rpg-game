use crate::core::game_objects::AttackObject;
use crate::core::player::Player;
use crate::core::weapon::Weapon;
use std::io;

pub struct Bow<'a> {
    weapon: Weapon<'a>,
    range: u8,
}

impl<'a> Bow<'a> {
    pub fn new(name: String, range: u8, player: &'a Player, greatness: u8) -> Bow<'a> {
        Bow {
            weapon: Weapon::new(name, greatness, player),
            range,
        }
    }
}

impl<'a> AttackObject for Bow<'a> {
    //TODO: mesafeye gore vectorel range hesaplama
    fn attack(&self, player: &mut Player) -> io::Result<()> {
        self.weapon.attack(player)
    }
}

