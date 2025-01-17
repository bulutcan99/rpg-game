use std::io::{self, Result};

use super::weapon::Weapon;
use crate::core::player::Player;

#[derive(Debug, Clone)]
pub struct Spear<'a> {
    name: String,
    rarity: String,
    price: u32,
    sharpness: u8,
    equipped_by: Option<&'a Player>,
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
