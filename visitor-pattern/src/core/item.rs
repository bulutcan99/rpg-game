use crate::core::game_objects::AttackObject;
use crate::core::player::Player;

pub struct Sword<'a> {
    pub name: String,
    pub equipped_by: &'a Player,
    pub greatness: u8,
}

impl<'a> Sword<'a> {
    pub fn new(name: String, player: &'a Player, greatness: u8) -> Sword {
        Sword {
            name,
            equipped_by: player,
            greatness,
        }
    }
}
impl<'a> AttackObject for Sword<'a> {
    fn attack(&self, player: &mut Player) -> std::io::Result<()> {
        let total_damage = self.greatness * self.equipped_by.get_strength();
        player.get_damage(total_damage)?;
        Ok(())
    }
}