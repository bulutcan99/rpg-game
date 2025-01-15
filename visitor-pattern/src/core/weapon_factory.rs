use crate::core::player::Player;

pub enum WeaponType {
    Sword,
    Bow,
}

pub struct WeaponFactory;

impl WeaponFactory {
    pub fn create_weapon<'a>(
        &self,
        weapon_type: WeaponType,
        name: String,
        greatness: u8,
        range: Option<u8>,
        equipped_by: &'a Player,
    ) -> Box<&dyn AttackObject> {

    }
}
