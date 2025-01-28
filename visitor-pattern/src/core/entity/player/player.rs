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
    fn set_weapon(&self, weapon: W) -> Result<()>;

    fn get_position(&self) -> (f32, f32);
    fn set_position(&self, position: (f32, f32)) -> Result<()>;

    fn get_strength(&self) -> u8;
    fn set_strength(&self, str: u8) -> Result<()>;

    fn take_damage(&self, damage: f32);
    //basksa bir enum olusturup yansitma, hasar, olduruldu donulsun
    fn strike(&self) -> Result<f32>;
}
