use crate::core::player::Player;
use std::io;

pub trait AttackObject {
    fn attack(&self, player: &mut Player) -> io::Result<()>;
}