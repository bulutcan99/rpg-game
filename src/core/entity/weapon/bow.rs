use std::any::Any;
use super::weapon::{Rarity, Weapon};

#[derive(Debug, Clone)]
pub struct Bow {
	name: String,
	rarity: Rarity,
	price: u8,
	weight: u8,
	attack_damage: f32,
	attack_speed: f32,
	durability: u8,
	range: u8,
	arrow_type: ArrowType,
	required_level: u32,
}

#[derive(Clone, Debug)]
pub enum ArrowType{
	Fire,
	Cold,
	Pierce
}

impl Bow {
	pub fn new(
		name: String,
		rarity: Rarity,
		price: u8,
		weight: u8,
		attack_damage: f32,
		attack_speed: f32,
		range: u8,
	arrow_type: ArrowType,
		required_level: u32,
	) -> Self {
		Self {
			name,
			rarity,
			price,
			weight,
			attack_damage,
			attack_speed,
			durability: 100_u8,
			range,
			arrow_type,
			required_level,
		}
	}

	fn get_arrow_type(&self) -> ArrowType{
		self.arrow_type.clone()
	}

}

impl Weapon for Bow {
	fn get_name(&self) -> &str {
		&self.name
	}

	fn get_rarity(&self) -> Rarity {
		self.rarity.clone()
	}

	fn get_price(&self) -> u8 {
		self.price
	}

	fn get_weight(&self) -> u8 {
		self.weight
	}

	fn get_attack_speed(&self) -> f32 {
		self.attack_speed
	}

	fn get_attack_damage(&self) -> f32 {
		self.attack_damage * self.rarity.get_value()
	}

	fn get_range(&self) -> u8 {
		self.range
	}

	fn get_required_level(&self) -> u32 {
		self.required_level
	}

	fn get_durability(&self) -> u8 {
		self.durability
	}

	fn set_durability(&mut self, durability: u8) {
		self.durability -= durability
	}

	fn clone_box(&self) -> Box<dyn Weapon> {
		Box::new(self.clone())
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}
