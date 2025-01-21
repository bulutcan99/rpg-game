use super::{bow::Bow, sword::Sword, weapon::Weapon};

pub trait WeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon>;
}

pub struct MeleeWeaponFactory;

impl WeaponFactory for MeleeWeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Sword::new("D".to_string(), "Legendary".to_string(), 100))
    }
}

pub struct RangedWeaponFactory;

impl WeaponFactory for RangedWeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Bow::new("B".to_string(), "Legendary".to_string(), 100, 100))
    }
}
