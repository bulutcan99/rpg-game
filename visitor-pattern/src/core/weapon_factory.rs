use super::{bow::Bow, sword::Sword, weapon::Weapon};

#[derive(Clone)]
pub enum WeaponType {
    Sword(Sword),
    Bow(Bow),
}

pub trait WeaponFactory: Clone {
    fn create_weapon(&self, weapon_choice: &str) -> WeaponType;
}

#[derive(Clone)]
pub struct WeaponCreator;

impl WeaponFactory for WeaponCreator {
    fn create_weapon(&self, weapon_choice: &str) -> WeaponType {
        match weapon_choice {
            "Sword" => WeaponType::Sword(Sword::new(
                "Excalibur".to_string(),
                "Legendary".to_string(),
                1000,
            )),
            "Bow" => WeaponType::Bow(Bow::new(
                "Eagle Bow".to_string(),
                "Rare".to_string(),
                800,
                50, // Bow range
            )),
            _ => panic!("Invalid weapon choice! Please choose 'Sword' or 'Bow'."),
        }
    }
}
