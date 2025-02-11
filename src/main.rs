use crate::core::entity::player::class::{Alive, AliveClass, Class, DeadClass};
use crate::core::entity::player::stat::Attribute;
use core::entity::{
	player::warrior::Warrior,
	weapon::weapon::Weapon,
};

mod core;
mod error;

#[tokio::main]
async fn main() -> error::Result<()> {
	let stat = Attribute {
		strength: 10,
		dexterity: 5,
		intelligence: 3,
	};

	let mut warrior = Warrior::<Alive>::new("Arthas".to_string(), stat, (0.0, 0.0));
	println!("Warrior Created: {}", warrior.get_name());

	println!("Health: {}", warrior.get_health());

	warrior.take_damage(30.0);
	println!("After damage, Health: {}", warrior.get_health());

	warrior.take_damage(80.0); // Should kill the warrior

	let dead_warrior = warrior.die();

	// Trying to resurrect
	let resurrected_warrior = dead_warrior.resurrect();
	println!("{} has been resurrected with health: {}", resurrected_warrior.get_name(), resurrected_warrior.get_health());
	Ok(())
}