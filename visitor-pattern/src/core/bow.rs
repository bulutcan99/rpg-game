use crate::core::weapon::Weapon;

pub struct Bow<'a> {
    weapon: Weapon<'a>,
    range: u8,
}

impl<'a> Bow<'a> {
    pub fn new(name: String, rarity: u8, range: u8) -> Bow<'a> {
        Bow {
            weapon: Weapon::new(name, rarity),
            range,
        }
    }
}
