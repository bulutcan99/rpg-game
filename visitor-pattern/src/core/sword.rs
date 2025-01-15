use crate::core::player::Player;
use crate::core::weapon::Weapon;

pub struct Sword<'a> {
    pub weapon: Weapon<'a>,
    pub durability: u8,
}

impl<'a> Sword<'a> {
    pub fn new(weapon: Weapon<'a>) -> Sword<'a> {
        Sword {
            weapon: weapon,
            durability: 100_u8,
        }
    }
}
