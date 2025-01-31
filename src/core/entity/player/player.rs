use std::io::Result;

use crate::core::entity::weapon::weapon::Weapon;

use super::{
    damage::DamageOutput,
    stat::{Stat, WhichStat},
};

pub trait Player: Send + Sync {
    /// General infos
    fn get_name(&self) -> &str;
    fn is_alive(&self) -> bool;

    fn get_health(&self) -> f32;
    //TODO: for overflow
    fn increase_health(&mut self, increase: f32) -> Result<()>;
    fn set_health(&mut self, new_health: f32) -> Result<()>;

    fn get_max_health(&self) -> f32;
    fn increase_max_health(&self, health: f32) -> Result<()>;
    fn set_max_health(&mut self, new_max_health: f32) -> Result<()>;

    fn get_stamina(&self) -> f32;
    fn increase_stamina(&mut self, increase: f32) -> Result<()>;
    fn set_stamina(&mut self, new_stamina: f32) -> Result<()>;

    fn get_level(&self) -> u32;
    fn increase_level(&mut self) -> Result<()>;
    fn set_level(&mut self, new_level: u32) -> Result<()>;

    fn get_experience(&self) -> u32;
    //TODO: levellerin ne kadar xp gerektirdigi ayarlanmali
    fn add_experience(&mut self, xp: u32);

    /// Equipment
    fn get_weapon(&self) -> Option<Box<dyn Weapon>>;
    //TODO: elinde silah varsa hata
    fn set_weapon(&mut self, weapon: Box<dyn Weapon>) -> Result<()>;

    fn get_gold(&self) -> u32;
    fn increase_gold(&self, gold: u32) -> Result<()>;
    fn set_gold(&self, new_gold: u32) -> Result<()>;

    /// Action
    fn take_damage(&mut self, damage: f32);
    fn strike(&mut self, target: &mut dyn Player) -> DamageOutput;
    fn use_skill(
        &mut self,
        skill_name: &str,
        target: Option<&mut dyn Player>,
    ) -> Result<(), SkillError>;

    /// Movement
    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, position: (f32, f32));
    fn move_by(&mut self, dx: f32, dy: f32);
    /// **Mutlak yerine görece hareket**

    /// Stats
    fn get_stat(&self) -> Stat;
    fn increase_stat(&mut self, amount: u8, stat: WhichStat) -> Result<()>;
    fn set_stat(&mut self, amount: u8, stat: WhichStat) -> Result<()>;

    /// Skills
    fn get_skills(&self) -> Vec<&str>;
}
