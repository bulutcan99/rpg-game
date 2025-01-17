use std::io::{self, Result};
use std::sync::Arc;

use super::weapon::Weapon;
use crate::core::player::Player;

#[derive(Debug, Clone)]
pub struct Spear {
    name: String,
    rarity: String,
    price: u32,
    sharpness: u8,
    equipped_by: Option<Arc<Player>>,
}

impl Spear {
    pub fn new(name: String, rarity: String, price: u32, sharpness: u8) -> Self {
        Self {
            name,
            rarity,
            price,
            sharpness,
            equipped_by: None,
        }
    }

    pub fn equip_item(&mut self, player: Arc<Player>) -> Result<()> {
        if self.equipped_by.is_some() {
            return Err(io::Error::new(io::ErrorKind::Other, "Already equipped!"));
        }

        self.equipped_by = Some(player);
        Ok(())
    }

    pub fn equipped_by(&self) -> Option<Arc<Player>> {
        self.equipped_by.clone() 
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

    fn attack(&self) -> String {
        format!("Thrust! Deals {} damage with sharpness.", self.sharpness)
    }
}
