use std::io::{self, Result};
use std::sync::Arc;

use super::{player::Player, weapon::Weapon};

#[derive(Debug, Clone)]
pub struct Bow {
    name: String,
    rarity: String,
    price: u32,
    range: u32,
    equipped_by: Option<Arc<Player>>,
}

impl Bow {
    pub fn new(name: String, rarity: String, price: u32, range: u32) -> Self {
        Self {
            name,
            rarity,
            price,
            range,
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

    fn attack(&self) -> String {
        format!("Shoot! Deals damage at range {}.", self.range)
    }
}
