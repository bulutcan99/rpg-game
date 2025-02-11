use rand::Rng;

use crate::core::entity::weapon::bow::ArrowType;
use crate::core::entity::weapon::{
	bow::Bow,
	spear::Spear,
	sword::Sword,
	weapon::{Rarity, Weapon},
};

fn random_stat_within_range(min: f32, max: f32) -> f32 {
	rand::thread_rng().gen_range(min..max)
}

fn random_stat_int_within_range(min: u8, max: u8) -> u8 {
	rand::thread_rng().gen_range(min..=max)
}

fn random_rarity() -> Rarity {
	let random_value = rand::thread_rng().gen_range(1..=4);
	Rarity::from(random_value)
}

/// Factory trait for weapon creation
pub trait WeaponFactory {
	fn create_weapon(&self) -> Box<dyn Weapon>;
}

/// Melee Weapon Factory (Creates Swords)
#[derive(Default)]
pub struct MeleeWeaponFactory;

impl WeaponFactory for MeleeWeaponFactory {
	fn create_weapon(&self) -> Box<dyn Weapon> {
		Box::new(Sword::new(format!("Sword {}", rand::thread_rng().gen_range(1..100)), random_rarity(), random_stat_int_within_range(50, 200), random_stat_int_within_range(5, 15), random_stat_within_range(10.0, 50.0), random_stat_within_range(1.0, 2.0), 1, 0))
	}
}

/// Ranged Weapon Factory (Creates Bows)
#[derive(Default)]
pub struct RangedWeaponFactory;

impl WeaponFactory for RangedWeaponFactory {
	fn create_weapon(&self) -> Box<dyn Weapon> {
		Box::new(Bow::new(format!("Bow {}", rand::thread_rng().gen_range(1..100)), random_rarity(), random_stat_int_within_range(100, 255), random_stat_int_within_range(5, 10), random_stat_within_range(20.0, 100.0), random_stat_within_range(1.0, 2.5), random_stat_int_within_range(20, 100), ArrowType::Fire, 0))
	}
}

/// Spear Weapon Factory (Creates Spears)
#[derive(Default)]
pub struct SpearWeaponFactory;

impl WeaponFactory for SpearWeaponFactory {
	fn create_weapon(&self) -> Box<dyn Weapon> {
		Box::new(Spear::new(format!("Spear {}", rand::thread_rng().gen_range(1..100)), random_rarity(), random_stat_int_within_range(100, 255), random_stat_int_within_range(6, 12), random_stat_within_range(15.0, 75.0), random_stat_within_range(1.2, 2.2), random_stat_int_within_range(10, 50), random_stat_int_within_range(10, 30), 0))
	}
}

/// **RandomWeaponFactory** (Can create any weapon type randomly or a specific type)
#[derive(Default)]
pub struct RandomWeaponFactory;

impl RandomWeaponFactory {
	pub fn create_weapons(&self, count: usize, weapon_type: Option<&str>) -> Vec<Box<dyn Weapon>> {
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


mod test {
	use crate::core::entity::weapon::bow::Bow;
	use crate::core::entity::weapon::spear::Spear;
	use crate::core::entity::weapon::sword::Sword;
	use crate::core::services::factory::weapon_factory::RandomWeaponFactory;

	#[tokio::test]
	async fn test_weapon_count() {
		let factory = RandomWeaponFactory::default();
		let weapons = factory.create_weapons(5, None);
		assert_eq!(weapons.len(), 5, "5 silah üretilmesi bekleniyordu.");
	}

	/// Rastgele silah üretiminin çalıştığını test eder
	#[tokio::test]
	async fn test_random_weapon_generation() {
		let factory = RandomWeaponFactory::default();
		let weapons = factory.create_weapons(10, None);

		assert!(!weapons.is_empty(), "Silah listesi boş olmamalı.");
		for weapon in weapons.iter() {
			println!("Generated Weapon: {:?}", weapon);
		}
	}

	/// Belirli bir silah türünün üretildiğini doğrular
	#[tokio::test]
	async fn test_specific_weapon_generation() {
		let factory = RandomWeaponFactory::default();

		let swords = factory.create_weapons(3, Some("Sword"));
		let bows = factory.create_weapons(3, Some("Bow"));
		let spears = factory.create_weapons(3, Some("Spear"));

		for weapon in swords.iter() {
			assert!(weapon.as_any().downcast_ref::<Sword>().is_some(), "Sword bekleniyordu.");
		}
		for weapon in bows.iter() {
			assert!(weapon.as_any().downcast_ref::<Bow>().is_some(), "Bow bekleniyordu.");
		}
		for weapon in spears.iter() {
			assert!(weapon.as_any().downcast_ref::<Spear>().is_some(), "Spear bekleniyordu.");
		}
	}

	/// Rastgele üretilen silahların istatistik değerlerinin beklenen aralıklarda olup olmadığını kontrol eder
	#[tokio::test]
	async fn test_weapon_stats_in_range() {
		let factory = RandomWeaponFactory::default();
		let weapons = factory.create_weapons(5, None);

		for weapon in weapons.iter() {
			let damage = weapon.get_attack_damage();
			let durability = weapon.get_durability();

			assert!(damage > 0.0, "Hasar 0'dan büyük olmalı.");
			assert!(durability >= 0, "Dayanıklılık negatif olmamalı.");
		}
	}
}