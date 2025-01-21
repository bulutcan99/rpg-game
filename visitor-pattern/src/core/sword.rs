use std::{
    io::{self, Result},
    mem::ManuallyDrop,
    sync::{Arc, Mutex},
};

use super::{player::Player, weapon::Weapon};

#[derive(Debug, Clone)]
pub struct Sword {
    name: String,
    rarity: String,
    price: u32,
    weight: u8,
    attack_speed: f32,
    durability: u8,
    equipped_by: Option<Arc<Player>>,
}

impl Sword {
    pub fn new(name: String, rarity: String, price: u32, weight: u8, attack_speed: f32) -> Self {
        Self {
            name,
            rarity,
            price,
            weight,
            attack_speed,
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
    fn weight(&self) -> u8 {
        self.weight
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

    fn equipped_by(&self) -> Option<Arc<Player>> {
        self.equipped_by.clone()
    }

    fn set_equip(&mut self, player: Arc<Player>) {
        self.equipped_by = Some(player)
    }

    //todo: result degil kendi enumimizi olusturcaz
    // success, yansima, veya kullanici olduyse vs gibi
    fn attack(&self, target: Arc<Mutex<Player>>) -> Result<f32> {
        //todo: eger sadece silah kusanilmis ise attack olabilir!

        //todo: hasar hesaplamasi icin bir seyler dusun!
        //todo: rarity bir enum

        let damage = self.attack_speed * 3.0;

        let mut target = target.lock().map_err(|_| "Failed to lock player")?;

        target.get_damage(damage);

        Ok(damage)
    }
}
