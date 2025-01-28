use std::io::Result;

use crate::core::entity::weapon::weapon::Weapon;

pub trait Player<W>
where
    W: Weapon,
{
    fn get_name(&self) -> &str;

    fn get_health(&self) -> f32;
    fn set_health(&mut self, new_health: f32);

    fn get_weapon(&self) -> Option<W>;
    // takili bir silah varsa silah takamaz
    fn set_weapon(&mut self, weapon: W) -> Result<()>;

    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, position: (f32, f32)) -> Result<()>;

    fn get_main_stat(&self) -> u8;
    fn set_main_stat(&mut self, str: u8) -> Result<()>;

    fn take_damage(&mut self, damage: f32);
    //basksa bir enum olusturup yansitma, hasar, olduruldu donulsun
    fn strike(&self, target: Box<dyn Player<W>>) -> DamageValidation;
}

pub enum DamageValidation {
    NoDamage,
    Reflect,
    Damage(f32),
    Died,
}
