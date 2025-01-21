use std::{
    io::Result,
    sync::{Arc, Mutex},
};

use super::player::Player;

pub trait Weapon {
    /// Already setted at creation.
    fn name(&self) -> &str;
    // Legendary/Epic/Rare/Common
    fn rarity(&self) -> &str;
    fn price(&self) -> u32;
    fn weight(&self) -> u8;
    fn attack_speed(&self) -> f32;

    /// They can change after.
    fn durability(&self) -> u8;
    fn set_durability(&mut self, durability: u8);

    fn equipped_by(&self) -> Option<Arc<Player>>;
    fn set_equip(&mut self, player: Arc<Player>);

    /// Some action methods.
    /// Bunu player'in methotlarina da tasiyabiliriz
    fn attack(&self, target: Arc<Mutex<Player>>) -> Result<f32>;
}
