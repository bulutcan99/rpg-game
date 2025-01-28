use std::io::Result;

use super::weapon::{self, Weapon};

pub trait Player<W>
where
    W: Weapon,
{
    fn get_name(&self) -> &str;
    fn get_health(&self) -> f32;
    fn get_weapon(&self) -> Option<W>;
    // takili bir silah varsa silah takamaz
    fn equip_weapon(&self, weapon: W) -> Result<()>;
}
