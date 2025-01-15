use super::weapon::Weapon;
use crate::core::player::Player;

pub struct Spear<'a> {
    name: String,
    rarity: String,
    price: u32,
    sharpness: u8,
    equipped_by: Option<&'a Player>, // 'static yaşam süresi varsayıldı, ancak buna ihtiyacınız varsa değiştirebilirsiniz
}

impl<'a> Spear<'a> {
    pub fn new(name: String, rarity: String, price: u32, sharpness: u8) -> Self {
        Self {
            name,
            rarity,
            price,
            sharpness,
            equipped_by: None,
        }
    }
}

impl<'a> Weapon<'a> for Spear<'a> {
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
        let damage = self.sharpness * 10; // Sharpness değerine göre hasar hesaplanıyor
        format!("Pierce! Deals {} stabbing damage.", damage)
    }
}
