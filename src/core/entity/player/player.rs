use std::io::Result;

use crate::core::entity::weapon::weapon::Weapon;

use super::{
    damage::DamageOutput,
    stat::{Stat, WhichStat},
};

pub trait Player: Send + Sync {
    fn get_name(&self) -> &str;

    fn get_health(&self) -> f32;
    fn increase_health(&mut self, increase: f32);
    fn set_health(&mut self, new_health: f32);

    fn get_weapon(&self) -> Option<Box<dyn Weapon>>;
    fn set_weapon(&mut self, weapon: Box<dyn Weapon>) -> Result<()>;

    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, position: (f32, f32)) -> Result<()>;

    fn get_stat(&self) -> Stat;
    fn incrase_stat(&mut self, increase: u8, which_stat: WhichStat) -> Result<()>;
    fn set_stat(&mut self, stat: u8, which_stat: WhichStat) -> Result<()>;

    fn take_damage(&mut self, damage: f32);
    fn strike(&self, target: Box<dyn Player>) -> DamageOutput;
}
