#[derive(Debug, Clone)]
pub struct Stat {
    strength: u8,
    dexterity: u8,
    intelligence: u8,
}

#[derive(Debug, Clone)]
pub enum WhichStat {
    Str,
    Dex,
    Int,
}
