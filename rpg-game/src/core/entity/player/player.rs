use std::io::Result;

use crate::core::entity::weapon::weapon::Weapon;

pub enum DamageValidation {
    NoDamage,
    Damage(f32),
    Died,
}

pub trait Player: Send + Sync {
    fn get_name(&self) -> &str;

    fn get_health(&self) -> f32;
    fn set_health(&mut self, new_health: f32);

    fn get_weapon(&self) -> Option<Box<dyn Weapon>>;
    fn set_weapon(&mut self, weapon: Box<dyn Weapon>) -> Result<()>;

    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, position: (f32, f32)) -> Result<()>;

    fn get_main_stat(&self) -> u8;
    fn set_main_stat(&mut self, stat: u8) -> Result<()>;

    fn take_damage(&mut self, damage: f32);

    fn strike(&self, target: Box<dyn Player>) -> DamageValidation;
}
