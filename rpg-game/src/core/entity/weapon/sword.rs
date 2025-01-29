use super::weapon::{Rarity, Weapon};

#[derive(Debug, Clone)]
pub struct Sword {
    name: String,
    rarity: Rarity,
    price: u32,
    weight: u8,
    attack_damage: f32,
    attack_speed: f32,
    durability: u8,
    range: u8,
}

impl Sword {
    pub fn new(
        name: String,
        rarity: Rarity,
        price: u32,
        weight: u8,
        attack_damage: f32,
        attack_speed: f32,
        range: u8,
    ) -> Self {
        Self {
            name,
            rarity,
            price,
            weight,
            attack_damage,
            attack_speed,
            durability: 100_u8,
            range,
        }
    }
}

impl Weapon for Sword {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_rarity(&self) -> Rarity {
        self.rarity.clone()
    }

    fn get_price(&self) -> u32 {
        self.price
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }

    fn get_attack_speed(&self) -> f32 {
        self.attack_speed
    }

    fn get_attack_damage(&self) -> f32 {
        self.attack_damage * self.rarity.get_value()
    }

    fn durability(&self) -> u8 {
        self.durability
    }

    fn set_durability(&mut self, durability: u8) {
        self.durability -= durability
    }

    fn get_range(&self) -> u8 {
        self.range
    }
}
