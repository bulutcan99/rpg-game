use std::{
    io::{self, Result},
    sync::Arc,
};

use super::{player::Player, weapon::Weapon};

#[derive(Debug, Clone)]
pub struct Sword {
    name: String,
    rarity: String,
    price: u32,
    durability: u8,
    equipped_by: Option<Arc<Player>>,
}

impl Sword {
    pub fn new(name: String, rarity: String, price: u32) -> Self {
        Self {
            name,
            rarity,
            price,
            durability: 100_u8,
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

impl Weapon for Sword {
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
        format!("Swing the sword! Deals {} damage.", self.durability)
    }
}
