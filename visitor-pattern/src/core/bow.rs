use super::player::Player;
use crate::core::weapon::Weapon;

pub struct Bow<'a> {
    name: String,
    rarity: String,
    price: u32,
    range: u32,
    equipped_by: Option<&'a Player>,
}

impl<'a> Bow<'a> {
    pub fn new(name: String, rarity: String, price: u32, range: u32) -> Self {
        Self {
            name,
            rarity,
            price,
            range,
            equipped_by: None,
        }
    }
}

impl<'a> Weapon<'a> for Bow<'a> {
    fn name(&self) -> &str {
        &self.name
    }

    fn rarity(&self) -> &str {
        &self.rarity
    }

    fn price(&self) -> u32 {
        self.price
    }

    fn equipped_by(&self) -> Option<&'a Player> {
        self.equipped_by
    }

    fn set_equipped_by(&mut self, player: Option<&'a Player>) {
        self.equipped_by = player;
    }

    fn attack(&self) -> String {
        format!("Shoot! Deals damage at range {}.", self.range)
    }
}
