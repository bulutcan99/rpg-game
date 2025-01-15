use crate::core::player::Player;
use std::{f64::consts::E, io};
pub trait AttackObject {
    fn get_name(&self) -> &str;
    fn get_rarity(&self) -> u8;
    fn get_equipped_by(&self) -> Option<&Player>;
    //TODO: arti basma
    fn set_rarity(&self) -> io::Result<u8>;
    fn attack(&self, player: &mut Player) -> io::Result<()>;
}
pub struct Weapon<'a> {
    pub name: String,
    pub rarity: u8,
    pub equipped_by: Option<&'a Player>,
}

impl<'a> Weapon<'a> {
    pub fn new(name: String, greatness: u8, player: Option<&'a Player>) -> Self {
        Weapon {
            name,
            greatness,
            equipped_by: player,
        }
    }

    pub fn attack(&self, player: &mut Player) -> io::Result<()> {
        if let Some(user) = self.equipped_by {
            let total_damage = self.greatness * user.get_strength();
            player.get_damage(total_damage)?;
            return Ok(());
        }

        return Err(io::Error::new(io::ErrorKind::Other, "Not equipped!"));
    }
}
