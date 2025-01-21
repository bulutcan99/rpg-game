pub trait Weapon {
    fn name(&self) -> &str;
    fn rarity(&self) -> &str;
    fn price(&self) -> u32;
    fn attack(&self) -> String;
}
