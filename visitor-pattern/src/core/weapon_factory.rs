use super::{bow::Bow, spear::Spear, sword::Sword, weapon::Weapon};

pub trait WeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon>;
}

pub struct SwordFactory;

impl WeaponFactory for SwordFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Sword::new(
            "EXCA".to_string(),
            "LEGENDARY".to_string(),
            1000,
        ))
    }
}

pub struct BowFactory;
impl WeaponFactory for BowFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Bow::new(
            "LIGHT".to_string(),
            "LEGENDARY".to_string(),
            1000,
            100,
        ))
    }
}

pub struct SpearFactory;
impl WeaponFactory for SpearFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Spear::new(
            "PIER".to_string(),
            "LEGENDARY".to_string(),
            1000,
            4,
        ))
    }
}
