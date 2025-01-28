use super::weapon::Weapon;

#[derive(Debug, Clone)]
pub struct Spear {
    name: String,
    rarity: String,
    price: u32,
    sharpness: u8,
}

impl Spear {
    pub fn new(name: String, rarity: String, price: u32, sharpness: u8) -> Self {
        Self {
            name,
            rarity,
            price,
            sharpness,
        }
    }
}

impl Weapon for Spear {
    fn name(&self) -> &str {
        &self.name
    }

    fn rarity(&self) -> &str {
        &self.rarity
    }

    fn price(&self) -> u32 {
        self.price
    }
}
