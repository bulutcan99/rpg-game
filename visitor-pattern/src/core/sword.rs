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

impl<'a> Weapon<'a> for Sword<'a> {
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

    // set_equipped_by fonksiyonu
    fn set_equipped_by(&mut self, player: Option<&'a Player>) {
        self.equipped_by = player;
    }

    fn attack(&self) -> String {
        if self.durability > 0 {
            format!("Slash! Deals {} damage.", 1000) // Sabit bir değerle saldırı
        } else {
            format!("The sword is broken!")
        }
    }
}
