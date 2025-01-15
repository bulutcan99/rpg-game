use super::weapon::{AttackObject, Weapon};
pub struct WeaponFactory;

impl WeaponFactory {
    pub fn create_weapon(&self, name: String, rarity: u8) -> Box<dyn AttackObject> {
        let weapon = Weapon::new(name, rarity);
        Box::new(weapon)
    }
}
