/// Attack Speed 1.0 means default 1 attack per second
pub trait Weapon {
    /// Already setted at creation.
    fn get_name(&self) -> &str;
    fn get_rarity(&self) -> Rarity;
    fn get_price(&self) -> u32;
    fn get_weight(&self) -> u8;
    fn get_attack_speed(&self) -> f32;
    fn get_attack_damage(&self) -> f32;
    fn get_range(&self) -> u8;

    /// They can change after.
    fn durability(&self) -> u8;
    fn set_durability(&mut self, durability: u8);
}

pub enum Rarity {
    Legendary,
    Epic,
    Rare,
    Common,
}

impl Rarity {
    pub fn get_value(&self) -> f32 {
        match self {
            Self::Legendary => 4.0,
            Self::Epic => 3.0,
            Self::Rare => 2.0,
            Self::Common => 1.0,
        }
    }
}

impl From<i32> for Rarity {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Common,
            2 => Rarity::Rare,
            3 => Self::Epic,
            4 => Rarity::Legendary,
        }
    }
}
