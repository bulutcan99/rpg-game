use core::fmt;
use std::any::Any;
use std::fmt::Debug;

/// Attack Speed 1.0 means default 1 attack per second
pub trait Weapon: Send + Sync {
    /// Already setted at creation.
    fn get_name(&self) -> &str;
    fn get_rarity(&self) -> Rarity;
    fn get_price(&self) -> u8;
    fn get_weight(&self) -> u8;
    fn get_attack_speed(&self) -> f32;
    fn get_attack_damage(&self) -> f32;
    fn get_range(&self) -> u8;
    fn get_required_level(&self) -> u32;

    /// They can change after.
    fn get_durability(&self) -> u8;
    fn set_durability(&mut self, durability: u8);

    fn clone_box(&self) -> Box<dyn Weapon>;
    fn as_any(&self) -> &dyn Any;
}

impl Debug for dyn Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Weapon")
            .field("name", &self.get_name())
            .field("rarity", &self.get_rarity())
            .field("price", &self.get_price())
            .field("weight", &self.get_weight())
            .field("attack_speed", &self.get_attack_speed())
            .field("attack_damage", &self.get_attack_damage())
            .field("range", &self.get_range())
            .field("durability", &self.get_durability())
            .field("required_level", &self.get_required_level())
            .finish()
    }
}

impl Clone for Box<dyn Weapon> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
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
            2 => Self::Rare,
            3 => Self::Epic,
            4 => Self::Legendary,
            _ => Self::Common,
        }
    }
}
