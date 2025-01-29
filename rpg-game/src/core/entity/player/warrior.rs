use super::player::{DamageValidation, Player};
use crate::core::entity::weapon::weapon::Weapon;
use std::io::{self, Result};

#[derive(Debug, Clone)]
pub struct Warrior {
    name: String,
    health: f32,
    strength: u8,
    weapon: Option<Box<dyn Weapon>>,
    position: (f32, f32),
}

impl Warrior {
    pub fn new(name: String, strength: u8, position: (f32, f32)) -> Warrior {
        Warrior {
            name,
            health: 100_f32,
            strength,
            weapon: None,
            position,
        }
    }
}

impl Player for Warrior {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> f32 {
        self.health
    }

    fn set_health(&mut self, new_health: f32) {
        self.health = new_health;
    }

    fn get_weapon(&self) -> Option<Box<dyn Weapon>> {
        self.weapon.clone()
    }

    fn set_weapon(&mut self, weapon: Box<dyn Weapon>) -> Result<()> {
        if self.weapon.is_some() {
            return Err(io::Error::new(io::ErrorKind::Other, "Weapon already equipped").into());
        }
        self.weapon = Some(weapon);
        Ok(())
    }

    fn get_position(&self) -> (f32, f32) {
        self.position
    }

    fn set_position(&mut self, position: (f32, f32)) -> Result<()> {
        self.position = position;
        Ok(())
    }

    fn get_main_stat(&self) -> u8 {
        self.strength
    }

    fn set_main_stat(&mut self, stat: u8) -> Result<()> {
        self.strength = stat;
        Ok(())
    }

    fn take_damage(&mut self, damage: f32) {
        if self.health < damage {
            println!("{} is dead!", self.name);
            self.health = 0.0;
        } else {
            self.health -= damage;
            println!(
                "{} took {} damage, remaining health: {}",
                self.name, damage, self.health
            );
        }
    }

    fn strike(&self, target: Box<dyn Player>) -> DamageValidation {
        if let Some(ref weapon) = self.weapon {
            let attack_damage = self.strength as f32 * weapon.get_attack_damage();

            let distance = ((self.position.0 - target.get_position().0).powi(2)
                + (self.position.1 - target.get_position().1).powi(2))
            .sqrt();

            if distance > weapon.get_range() as f32 {
                println!("Target is out of range!");
                return DamageValidation::NoDamage;
            }

            if target.get_health() < attack_damage {
                println!("{} killed {}", self.get_name(), target.get_name());
                return DamageValidation::Died;
            }

            println!(
                "{} dealt {} damage to {}",
                self.get_name(),
                attack_damage,
                target.get_name()
            );
            return DamageValidation::Damage(attack_damage);
        } else {
            println!("{} has no weapon!", self.get_name());
            return DamageValidation::NoDamage;
        }
    }
}
