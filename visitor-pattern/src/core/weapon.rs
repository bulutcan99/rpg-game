/// Attack Speed 1.0 means default 1 attack per second
pub trait Weapon {
    /// Already setted at creation.
    fn get_name(&self) -> &str;
    // Legendary/Epic/Rare/Common
    fn get_rarity(&self) -> Rarity;
    fn get_price(&self) -> u32;
    fn get_weight(&self) -> u8;
    fn get_attack_speed(&self) -> f32;

    /// They can change after.
    fn durability(&self) -> u8;
    fn set_durability(&mut self, durability: u8);
}

pub enum Rarity {
    Legendary,
    Epic,
    Rare,
    Common,
}
