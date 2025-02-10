use crate::core::entity::player::stat::{Attribute, WhichAttribute};
use crate::core::entity::weapon::weapon::Weapon;

use super::{
    damage::DamageOutput,
    error::Result,
};

// player state durumu handle edilcek state pattern ile
pub trait Class: Send + Sync {
    /// General infos
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

    /// Equipment
    fn get_weapon(&self) -> Option<&dyn Weapon>;
    fn set_weapon(&mut self, weapon: &dyn Weapon) -> Result<()>;

    fn get_gold(&self) -> u32;
    fn increase_gold(&self, gold: u32) -> Result<()>;
    fn set_gold(&self, new_gold: u32) -> Result<()>;

    /// Action
    fn take_damage(&mut self, damage: f32);
    fn strike<P: Class>(&mut self, target: &mut P) -> DamageOutput;

    /// Movement
    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, position: (f32, f32));
    fn move_by(&mut self, dx: f32, dy: f32);

    /// Stats
    fn get_stat(&self) -> Attribute;
    fn get_main_stat(&self) -> WhichAttribute;
    fn increase_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()>;
    fn set_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()>;

    //TODO: Skills
    // fn get_skills(&self) -> Vec<String>;
    // fn use_skill<P: Class>(&mut self, skill_name: &str, target: Option<&mut P>) -> Result<()>;

    fn calculate_attack_damage(&self) -> f32{
        let stats = self.get_stat();
        let main_stat = self.get_main_stat();

        let (main_factor, secondary_factor) = match main_stat {
            WhichAttribute::Str => (1.0, 0.5),
            WhichAttribute::Dex => (1.0, 0.5),
            WhichAttribute::Int => (1.0, 0.5),
        };

        let main_value = match main_stat {
            WhichAttribute::Str => stats.strength as f32,
            WhichAttribute::Dex => stats.dexterity as f32,
            WhichAttribute::Int => stats.intelligence as f32,
        };

         let secondary_value = (stats.strength + stats.dexterity + stats.intelligence) as f32
            - main_value;

        (main_value * main_factor) + (secondary_value * secondary_factor)
    }
}

