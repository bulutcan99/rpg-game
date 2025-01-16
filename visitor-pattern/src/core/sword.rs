use super::{player::Player, weapon::Weapon};

pub struct Sword<'a> {
    name: String,
    rarity: String,
    price: u32,
    durability: u8,
    equipped_by: Option<&'a Player>,
}

impl<'a> Sword<'a> {
    pub fn new(name: String, rarity: String, price: u32) -> Self {
        Self {
            name,
            rarity,
            price,
            durability: 100_u8, // Başlangıçta tam dayanıklılık
            equipped_by: None,
        }
    }
}

impl<'a> Weapon for Sword<'a> {
    fn name(&self) -> &str {
        &self.name
    }

    fn rarity(&self) -> &str {
        &self.rarity
    }

    fn price(&self) -> u32 {
        self.price
    }

    fn attack(&self) -> String {
        format!("Shoot! Deals damage at range {}.", self.durability)
    }
}
