use super::weapon::Weapon;

#[derive(Debug, Clone)]
pub struct Bow {
    name: String,
    rarity: String,
    price: u32,
    weight: u8,
    attack_speed: f32,
    durability: u8,
    range: u32,
}

impl Bow {
    pub fn new(name: String, rarity: String, price: u32, weight: u8, attack_speed: f32) -> Self {
        Self {
            name,
            rarity,
            price,
            weight,
            attack_speed,
            durability: 100_u8,
            range: 1.0,
        }
    }
}

impl Weapon for Bow {
    fn name(&self) -> &str {
        &self.name
    }

    fn rarity(&self) -> &str {
        &self.rarity
    }

    fn price(&self) -> u32 {
        self.price
    }

    fn attack_speed(&self) -> f32 {
        self.attack_speed
    }

    fn durability(&self) -> u8 {
        self.durability
    }

    fn set_durability(&mut self, durability: u8) {
        self.durability -= durability
    }

    fn weight(&self) -> u8 {
        self.weight
    }
}
