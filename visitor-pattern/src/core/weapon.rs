use crate::core::player::Player;
use std::io;

pub struct Weapon<'a> {
    pub name: String,
    pub greatness: u8,
    pub equipped_by: &'a Player,
}

impl<'a> Weapon<'a> {
    pub fn new(name: String, greatness: u8, player: &'a Player) -> Self {
        Weapon {
            name,
            greatness,
            equipped_by: player,
        }
    }

    pub fn attack(&self, player: &mut Player) -> io::Result<()> {
        let total_damage = self.greatness * self.equipped_by.get_strength();
        player.get_damage(total_damage)?;
        Ok(())
    }
}