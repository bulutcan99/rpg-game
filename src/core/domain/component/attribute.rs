use bevy::prelude::Component;

#[derive(Component)]
pub struct Strength(pub u16);

#[derive(Component)]
pub struct Dexterity(pub u16);

#[derive(Component)]
pub struct Intelligence(pub u16);

#[derive(Component)]
pub struct Attribute {
    pub strength: Strength,
    pub dexterity: Dexterity,
    pub intelligence: Intelligence,
}

impl Attribute {
    pub fn new() -> Self {
        Self {
            strength: Strength(10),
            dexterity: Dexterity(8),
            intelligence: Intelligence(12),
        }
    }
}
