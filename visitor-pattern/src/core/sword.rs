use std::io::{self, Result};

use super::{player::Player, weapon::Weapon};

#[derive(Debug, Clone)]
pub struct Sword<'a> {
    name: String,
    rarity: String,
    price: u32,
    durability: u8,
    equipped_by: Option<&'a Player>,
}

impl<'a> Sword<'a> {
    pub fn new(name: String, rarity: String, price: u32) -> Self {
        Self {
            name,
            rarity,
            price,
            durability: 100_u8,
            equipped_by: None,
        }
    }
    pub fn equip_item(&mut self, player: &'a Player) -> Result<()> {
        if let Some(_) = self.equipped_by {
            return Err(io::Error::new(io::ErrorKind::Other, "Already equipped!"));
        }

        self.equipped_by = Some(player);
        Ok(())
    }
    pub fn equipped_by(&self) -> Option<&'a Player> {
        if let Some(player) = self.equipped_by {
            return Some(player);
        }

        None
    }
}

impl<'a> Weapon for Sword<'a> {
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
        format!("Shoot! Deals damage at range {}.", self.durability)
    }
}
