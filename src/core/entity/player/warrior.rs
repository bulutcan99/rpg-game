use super::{class::Class, error::Result, stat::Attribute};
use crate::core::entity::player::error::Error;
use crate::core::entity::player::stat::WhichAttribute;
use crate::core::entity::{player::damage::DamageOutput, weapon::weapon::Weapon};

#[derive(Debug, Clone)]
pub struct Warrior<W>
where
	W: Weapon,
{
	name: String,
	health: f32,
	max_health: f32,
	stamina: f32,
	max_stamina: f32,
	level: u32,
	experience: u32,
	stat: Attribute,
	weapon: Option<W>,
	position: (f32, f32),
	gold: u32,
}

impl<W> Warrior<W>
where
	W: Weapon,
{
	pub fn new(name: String, stat: Attribute, position: (f32, f32)) -> Warrior<W> {
		Warrior {
			name,
			health: 100_f32,
			max_health: 100_f32,
			stamina: 50_f32,
			max_stamina: 50_f32,
			level: 1,
			experience: 0,
			stat,
			weapon: None,
			position,
			gold: 0,
		}
	}
}

impl<W> Class for Warrior<W>
where
	W: Weapon,
{
	fn get_name(&self) -> &str {
		&self.name
	}

	fn is_alive(&self) -> bool {
		self.health > 0.0
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

	fn get_weapon(&self) -> Option<&dyn Weapon> {
		self.weapon.as_ref()
	}

	fn set_weapon(&mut self, weapon: W) -> Result<()> {
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

	fn take_damage(&mut self, damage: f32) {
		if self.health < damage {
			println!("{} is dead!", self.name);
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
		P: Class,
	{
		if let Some(ref weapon) = self.weapon {
			let attack_damage = self.calculate_attack_damage() * weapon.get_attack_damage();

			let distance = ((self.position.0 - target.get_position().0).powi(2)
				+ (self.position.1 - target.get_position().1).powi(2))
				.sqrt();

			if distance > weapon.get_range() as f32 {
				println!("Target is out of range!");
				return DamageOutput::NoDamage;
			}

			if target.get_health() < attack_damage {
				println!("{} killed {}", self.get_name(), target.get_name());
				return DamageOutput::Kill;
			}

			println!(
				"{} dealt {} damage to {}",
				self.get_name(),
				attack_damage,
				target.get_name()
			);
			DamageOutput::Damage(attack_damage)
		} else {
			println!("{} has no weapon!", self.get_name());
			DamageOutput::NoDamage
		}
	}

	fn get_position(&self) -> (f32, f32) {
		self.position
	}

	fn set_position(&mut self, position: (f32, f32)) -> Result<()> {
		self.position = position;
		Ok(())
	}

	fn move_by(&mut self, dx: f32, dy: f32) {
		self.position.0 += dx;
		self.position.1 += dy;
	}

	fn get_stat(&self) -> Attribute {
		self.stat.clone()
	}

	fn get_main_stat(&self) -> WhichAttribute {
		todo!()
	}

	fn increase_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()> {
		match stat {
			WhichAttribute::Str => self.stat.strength += amount,
			WhichAttribute::Int => self.stat.intelligence += amount,
			WhichAttribute::Dex => self.stat.dexterity += amount,
			_ => return Err(Error::InvalidStat),
		}
		Ok(())
	}

	fn set_stat(&mut self, amount: u8, stat: WhichAttribute) -> Result<()> {
		match stat {
			WhichAttribute::Str => self.stat.strength = amount,
			WhichAttribute::Int => self.stat.intelligence = amount,
			WhichAttribute::Dex => self.stat.dexterity = amount,
			_ => return Err(Error::InvalidStat),
		}
		Ok(())
	}
}


