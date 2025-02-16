use crate::core::combat::damage::DamageOutput;
use crate::core::entity::player::stat::{Attribute, WhichAttribute};
use crate::core::entity::weapon::weapon::Weapon;

use super::error::Result;

/// Implement Status for Alive and Dead
pub struct Alive;
pub struct Dead;

pub trait AliveStatus {
    fn is_alive() -> bool;
}

impl AliveStatus for Alive {
    fn is_alive() -> bool {
        true
    }
}

impl AliveStatus for Dead {
    fn is_alive() -> bool {
        false
    }
}

pub trait Class: Send + Sync {
    const MAIN_STAT: WhichAttribute;
    type Status: AliveStatus;
    fn get_name(&self) -> &str;
    fn is_alive(&self) -> bool;
    fn get_health(&self) -> f32;
    fn increase_health(&mut self, increase: f32) -> Result<()>;
    fn set_health(&mut self, new_health: f32) -> Result<()>;
    fn get_max_health(&self) -> f32;
    fn increase_max_health(&mut self, health: f32) -> Result<()>;
    fn set_max_health(&mut self, new_max_health: f32) -> Result<()>;
    fn get_stamina(&self) -> f32;
    fn increase_stamina(&mut self, increase: f32) -> Result<()>;
    fn set_stamina(&mut self, new_stamina: f32) -> Result<()>;
    fn get_level(&self) -> u32;
    fn increase_level(&mut self) -> Result<()>;
    fn set_level(&mut self, new_level: u32) -> Result<()>;
    fn get_experience(&self) -> u32;
    fn add_experience(&mut self, xp: u32);
    fn get_position(&self) -> (f32, f32);
    fn get_weapon(&self) -> Option<Box<dyn Weapon>>;
    fn set_weapon(&mut self, weapon: Box<dyn Weapon>) -> Result<()>;
    fn get_gold(&self) -> u32;
    fn increase_gold(&mut self, gold: u32) -> Result<()>;
    fn set_gold(&mut self, new_gold: u32) -> Result<()>;
    fn get_stat(&self) -> Attribute;
    fn increase_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()>;
    fn set_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()>;
}

pub trait AliveClass: Class {
    type DeadType: DeadClass;

    fn take_damage(&mut self, damage: f32);
    fn strike<P: AliveClass>(&mut self, target: &mut P) -> DamageOutput;
    fn set_position(&mut self, position: (f32, f32));
    fn move_by(&mut self, dx: f32, dy: f32);
    fn die(self) -> Self::DeadType;
}

pub trait DeadClass: Class {
    type AliveType: AliveClass;

    fn resurrect(self) -> Self::AliveType;
}
