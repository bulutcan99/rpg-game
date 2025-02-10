#[derive(Debug, Clone)]
pub struct Attribute {
    pub strength: u8,
    pub dexterity: u8,
    pub intelligence: u8,
}

#[derive(Debug, Clone)]
pub enum WhichAttribute {
    Str,
    Dex,
    Int,
}
