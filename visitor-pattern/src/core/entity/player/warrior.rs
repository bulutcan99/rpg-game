use std::io::{self, Result};

use super::{
    player::Player,
    weapon::{self, Weapon},
};

#[derive(Debug, Clone)]
pub struct Warrior<W>
where
    W: Weapon,
{
    name: String,
    health: f32,
    strength: u8,
    weapon: Option<W>,
    position: (f32, f32),
}

impl<W> Warrior<W>
where
    W: Weapon,
{
    pub fn new(name: String, strength: u8, position: (f32, f32), weapon: W) -> Warrior<W> {
        Warrior {
            name,
            health: 100_f32,
            strength,
            weapon: Some(weapon),
            position,
        }
    }
}

impl<W> Player<W> for Warrior<W>
where
    W: Weapon,
{
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> f32 {
        self.health
    }

    fn get_weapon(&self) -> Option<W> {
        self.weapon.clone()
    }

    fn set_weapon(&self, weapon: W) -> Result<()> {
        if self.weapon.is_some() {
            return Err(io::Error::new(io::ErrorKind::Other, "Weapon already equipped").into());
        }
        self.weapon = Some(weapon);
        Ok(())
    }

    fn get_position(&self) -> (f32, f32) {
        self.position
    }

    fn set_position(&self, position: (f32, f32)) -> Result<()> {
        self.position = position;
        Ok(())
    }

    fn get_strength(&self) -> u8 {
        self.strength
    }

    fn set_strength(&self, str: u8) -> Result<()> {
        self.strength = str;
        Ok(())
    }

    fn take_damage(&self, damage: f32) {
        if self.health < damage {
            println!("{} is dead!", self.name);
        } else {
            self.health -= damage;
            println!(
                "{} took {} damage, remaining health: {}",
                self.name, damage, self.health
            );
        }
    }

    //todo: hasar hesaplamasi algoritmasi
    fn strike(&self) -> Result<f32> {
        if let Some(ref weapon) = self.weapon {
            println!("{} strikes with {}!", self.name, weapon.name());
            Ok(self.strength as f32)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No weapon equipped").into())
        }
    }
}
