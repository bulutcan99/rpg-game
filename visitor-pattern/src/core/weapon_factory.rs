use super::weapon::{AttackObject, Weapon};

pub enum WeaponType {
    Sword,
    Bow,
}

pub struct WeaponFactory;

impl WeaponFactory {
    pub fn create_weapon(
        &self,
        weapon_type: WeaponType,
        name: String,
        rarity: u8,
    ) -> Box<dyn AttackObject> {
        match weapon_type {
            WeaponType::Sword => {
                let sword = Weapon::new(name, rarity);
                Box::new(sword)
            }
            WeaponType::Bow => {
                let bow = Weapon::new(name, rarity);
                Box::new(bow)
            }
        }
    }
}
