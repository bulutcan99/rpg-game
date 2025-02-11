use crate::core::entity::player::class::{Alive, AliveClass, AliveStatus, Class, Dead, DeadClass};
use crate::core::entity::player::damage::DamageOutput;
use crate::core::entity::player::error::{Error, Result};
use crate::core::entity::player::stat::{Attribute, WhichAttribute};
use crate::core::entity::weapon::weapon::Weapon;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Warrior<S> {
	status: PhantomData<S>,
	name: String,
	health: f32,
	max_health: f32,
	stamina: f32,
	max_stamina: f32,
	level: u32,
	experience: u32,
	stat: Attribute,
	weapon: Option<Box<dyn Weapon>>,
	position: (f32, f32),
	gold: u32,
}

impl Warrior<Alive> {
	pub fn new(name: String, stat: Attribute, position: (f32, f32)) -> Self {
		Warrior {
			status: PhantomData,
			name,
			health: 100.0,
			max_health: 100.0,
			stamina: 50.0,
			max_stamina: 50.0,
			level: 1,
			experience: 0,
			stat,
			weapon: None,
			position,
			gold: 0,
		}
	}
}

impl<S> Class for Warrior<S>
where
	S: AliveStatus + Send + Sync,
{
	const MAIN_STAT: WhichAttribute = WhichAttribute::Str;
	type Status = S;

	fn get_name(&self) -> &str {
		&self.name
	}

	fn is_alive(&self) -> bool {
		S::is_alive()
	}

	fn get_health(&self) -> f32 {
		self.health
	}

	fn increase_health(&mut self, increase: f32) -> Result<()> {
		self.health += increase;
		if self.health > self.max_health {
			self.health = self.max_health;
		}
		Ok(())
	}

	fn set_health(&mut self, new_health: f32) -> Result<()> {
		self.health = new_health;
		if self.health > self.max_health {
			self.health = self.max_health;
		}
		Ok(())
	}

	fn get_max_health(&self) -> f32 {
		self.max_health
	}

	fn increase_max_health(&mut self, health: f32) -> Result<()> {
		self.max_health += health;
		Ok(())
	}

	fn set_max_health(&mut self, new_max_health: f32) -> Result<()> {
		self.max_health = new_max_health;
		Ok(())
	}

	fn get_stamina(&self) -> f32 {
		self.stamina
	}

	fn increase_stamina(&mut self, increase: f32) -> Result<()> {
		self.stamina += increase;
		if self.stamina > self.max_stamina {
			self.stamina = self.max_stamina;
		}
		Ok(())
	}

	fn set_stamina(&mut self, new_stamina: f32) -> Result<()> {
		self.stamina = new_stamina;
		Ok(())
	}

	fn get_level(&self) -> u32 {
		self.level
	}

	fn increase_level(&mut self) -> Result<()> {
		self.level += 1;
		Ok(())
	}

	fn set_level(&mut self, new_level: u32) -> Result<()> {
		self.level = new_level;
		Ok(())
	}

	fn get_experience(&self) -> u32 {
		self.experience
	}

	fn add_experience(&mut self, xp: u32) {
		self.experience += xp;
	}

	fn get_position(&self) -> (f32, f32) {
		self.position
	}

	fn get_weapon(&self) -> Option<Box<dyn Weapon>> {
		self.weapon.clone()
	}

	fn set_weapon(&mut self, weapon: Box<dyn Weapon>) -> Result<()> {
		if self.weapon.is_some() {
			return Err(Error::WeaponAlreadyEquipped);
		}

		self.weapon = Some(weapon);
		Ok(())
	}

	fn get_gold(&self) -> u32 {
		self.gold
	}

	fn increase_gold(&mut self, gold: u32) -> Result<()> {
		self.gold += gold;
		Ok(())
	}

	fn set_gold(&mut self, new_gold: u32) -> Result<()> {
		self.gold = new_gold;
		Ok(())
	}

	fn get_stat(&self) -> Attribute {
		self.stat.clone()
	}

	fn increase_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()> {
		match stat {
			WhichAttribute::Str => self.stat.strength += amount,
			WhichAttribute::Int => self.stat.intelligence += amount,
			WhichAttribute::Dex => self.stat.dexterity += amount,
		}
		Ok(())
	}

	fn set_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()> {
		match stat {
			WhichAttribute::Str => self.stat.strength = amount,
			WhichAttribute::Int => self.stat.intelligence = amount,
			WhichAttribute::Dex => self.stat.dexterity = amount,
		}
		Ok(())
	}
}

impl AliveClass for Warrior<Alive> {
	type DeadType = Warrior<Dead>;
	fn take_damage(&mut self, damage: f32) {
		if self.health < damage {
			println!("{} has been killed!", self.name);
			self.health = 0.0;
		} else {
			self.health -= damage;
			println!(
				"[{}] took {} damage, remaining health: {}",
				self.name, damage, self.health
			);
		}
	}

	fn strike<P>(&mut self, target: &mut P) -> DamageOutput
	where
		P: AliveClass,
	{
		if let Some(ref weapon) = self.weapon {
			let attack_damage = self.stat.strength as f32 * weapon.get_attack_damage();
			let distance = ((self.position.0 - target.get_position().0).powi(2)
				+ (self.position.1 - target.get_position().1).powi(2))
				.sqrt();
			if distance > weapon.get_range() as f32 {
				println!("Target is out of range!");
				return DamageOutput::NoDamage;
			}
			println!("Attacked {} with {} damage.", target.get_name(), attack_damage);
			return DamageOutput::Damage(attack_damage);
		}
		DamageOutput::NoDamage
	}

	fn set_position(&mut self, position: (f32, f32)) {
		self.position = position;
	}

	fn move_by(&mut self, dx: f32, dy: f32) {
		self.position.0 += dx;
		self.position.1 += dy;
	}

	fn die(self) -> Self::DeadType {
		println!("{} has died!", self.name);
		Warrior {
			status: PhantomData,
			name: self.name,
			health: 0.0,
			max_health: self.max_health,
			stamina: self.stamina,
			max_stamina: self.max_stamina,
			level: self.level,
			experience: self.experience,
			stat: self.stat,
			weapon: self.weapon,
			position: self.position,
			gold: self.gold,
		}
	}
}

impl DeadClass for Warrior<Dead> {
	type AliveType = Warrior<Alive>;
	fn resurrect(self) -> Self::AliveType {
		println!("{} has been resurrected!", self.name);
		Warrior {
			status: PhantomData,
			name: self.name,
			health: self.max_health,
			max_health: self.max_health,
			stamina: self.stamina,
			max_stamina: self.max_stamina,
			level: self.level,
			experience: self.experience,
			stat: self.stat,
			weapon: self.weapon,
			position: self.position,
			gold: self.gold,
		}
	}
}

mod test {
	use crate::core::entity::player::class::DeadClass;

	#[tokio::test]
	async fn dead_or_alive() {
		use crate::core::entity::player::class::{AliveClass, Class};
		use crate::core::entity::player::stat::Attribute;
		use crate::core::entity::player::warrior::Warrior;

		let stat = Attribute {
			strength: 10,
			dexterity: 5,
			intelligence: 3,
		};

		let mut warrior = Warrior::new("Arthas".to_string(), stat, (0.0, 0.0));

		assert_eq!(warrior.get_name(), "Arthas");
		assert_eq!(warrior.get_health(), 100.0);

		assert!(warrior.is_alive());

		warrior.take_damage(30.0);
		assert_eq!(warrior.get_health(), 70.0);

		warrior.take_damage(80.0);
		let dead_warrior = warrior.die();
		assert_eq!(dead_warrior.get_health(), 0.0);
		assert!(!dead_warrior.is_alive());

		let resurrected_warrior = dead_warrior.resurrect();
		assert!(resurrected_warrior.is_alive());
		assert_eq!(resurrected_warrior.get_health(), 100.0);
	}
}
