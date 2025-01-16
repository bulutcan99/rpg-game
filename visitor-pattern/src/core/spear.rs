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

impl<'a> Weapon for Spear<'a> {
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
        format!("Shoot! Deals damage at range {}.", self.sharpness)
    }
}
