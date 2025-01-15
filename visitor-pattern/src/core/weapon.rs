// weapon.rs
use crate::core::player::Player;

pub trait Weapon<'a> {
    fn name(&self) -> &str;
    fn rarity(&self) -> &str;
    fn price(&self) -> u32;
    fn attack(&self) -> String;
    fn equipped_by(&self) -> Option<&'a Player>;
    fn set_equipped_by(&mut self, player: Option<&'a Player>);

    fn equip(&mut self, player: &'a Player) {
        self.set_equipped_by(Some(player));
    }
}
