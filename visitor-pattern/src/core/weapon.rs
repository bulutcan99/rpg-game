// weapon.rs
use crate::core::player::Player;
use std::io;

pub trait AttackObject {
    fn get_name(&self) -> &str;
    fn get_rarity(&self) -> u8;
    fn get_equipped_by(&self) -> Option<&Player>;
    fn set_rarity(&mut self, rarity: u8) -> io::Result<()>;
    fn attack(&self, player: &mut Player) -> io::Result<()>;
}

pub struct Weapon<'a> {
    name: String,
    rarity: u8,
    equipped_by: Option<&'a Player>,
}

impl<'a> Weapon<'a> {
    pub fn new(name: String, rarity: u8) -> Self {
        Weapon {
            name,
            rarity,
            equipped_by: None,
        }
    }

    pub fn set_equipped_by(&mut self, player: &'a Player) {
        self.equipped_by = Some(player);
    }
}

impl<'a> AttackObject for Weapon<'a> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_rarity(&self) -> u8 {
        self.rarity
    }

    fn get_equipped_by(&self) -> Option<&Player> {
        self.equipped_by
    }

    fn set_rarity(&mut self, rarity: u8) -> io::Result<()> {
        self.rarity = rarity;
        Ok(())
    }

    fn attack(&self, player: &mut Player) -> io::Result<()> {
        if let Some(user) = self.equipped_by {
            let total_damage = self.rarity * user.get_strength();
            player.get_damage(total_damage)?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Weapon not equipped!"))
        }
    }
}
