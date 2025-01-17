//TODO: bir composition haline getir -> struct yap
pub trait Weapon {
    fn name(&self) -> &str;
    fn rarity(&self) -> &str;
    fn price(&self) -> u32;
    fn attack(&self) -> String;
}
