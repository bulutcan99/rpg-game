use rand::Rng;

use crate::core::entity::weapon::{
    bow::Bow,
    spear::Spear,
    sword::Sword,
    weapon::{Rarity, Weapon},
};

fn random_stat_within_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

fn random_int_within_range(min: u8, max: u8) -> u8 {
    rand::thread_rng().gen_range(min..=max)
}

fn random_rarity() -> Rarity {
    let random_value = rand::thread_rng().gen_range(1..=4);
    Rarity::from(random_value)
}

// Factory trait for weapon creation
pub trait WeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon>;
}

// Melee Weapon Factory (Creates Swords)
pub struct MeleeWeaponFactory;

impl WeaponFactory for MeleeWeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Sword::new(
            format!("Sword {}", rand::thread_rng().gen_range(1..100)),
            random_rarity(),
            random_int_within_range(50, 200),     // Price
            random_int_within_range(5, 15),       // Weight
            random_stat_within_range(10.0, 50.0), // Attack Damage
            random_stat_within_range(1.0, 2.0),   // Attack Speed
            1,                                    // Sword range is always 1
        ))
    }
}

// Ranged Weapon Factory (Creates Bows)
pub struct RangedWeaponFactory;

impl WeaponFactory for RangedWeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Bow::new(
            format!("Bow {}", rand::thread_rng().gen_range(1..100)),
            random_rarity(),
            random_int_within_range(100, 500),     // Price
            random_int_within_range(5, 10),        // Weight
            random_stat_within_range(20.0, 100.0), // Attack Damage
            random_stat_within_range(1.0, 2.5),    // Attack Speed
            random_int_within_range(20, 100),      // Range (Bow has largest range)
        ))
    }
}

// Spear Weapon Factory (Creates Spears)
pub struct SpearWeaponFactory;

impl WeaponFactory for SpearWeaponFactory {
    fn create_weapon(&self) -> Box<dyn Weapon> {
        Box::new(Spear::new(
            format!("Spear {}", rand::thread_rng().gen_range(1..100)),
            random_rarity(),
            random_int_within_range(100, 400),    // Price
            random_int_within_range(6, 12),       // Sharpness
            random_stat_within_range(15.0, 75.0), // Attack Damage
            random_stat_within_range(1.2, 2.2),   // Attack Speed
            random_int_within_range(10, 50),      // Range (Spear range is between Sword and Bow)
            random_int_within_range(10, 30),      // Sharpness multiplier
        ))
    }
}

// **RandomWeaponFactory** (Can create any weapon type randomly or a specific type)
pub struct RandomWeaponFactory;

impl RandomWeaponFactory {
    pub fn create_weapons(count: usize, weapon_type: Option<&str>) -> Vec<Box<dyn Weapon>> {
        let mut weapons: Vec<Box<dyn Weapon>> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let weapon: Box<dyn Weapon> = match weapon_type {
                Some("Sword") => MeleeWeaponFactory.create_weapon(),
                Some("Bow") => RangedWeaponFactory.create_weapon(),
                Some("Spear") => SpearWeaponFactory.create_weapon(),
                _ => {
                    let random_choice = rng.gen_range(0..3);
                    match random_choice {
                        0 => MeleeWeaponFactory.create_weapon(),
                        1 => RangedWeaponFactory.create_weapon(),
                        _ => SpearWeaponFactory.create_weapon(),
                    }
                }
            };
            weapons.push(weapon);
        }
        weapons
    }
}
