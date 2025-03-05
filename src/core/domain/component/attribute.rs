use bevy::prelude::{Bundle, Component};

use crate::core::domain::entity::class::ClassType;

#[derive(Component)]
pub struct Strength(pub u16);

#[derive(Component)]
pub struct Dexterity(pub u16);

#[derive(Component)]
pub struct Intelligence(pub u16);

#[derive(Bundle)]
pub struct ClassAttributeBundle {
    pub strength: Strength,
    pub dexterity: Dexterity,
    pub intelligence: Intelligence,
}

impl ClassAttributeBundle {
    pub fn new(class: ClassType) -> Self {
        match class {
            ClassType::Warrior => Self {
                strength: Strength(15),
                dexterity: Dexterity(8),
                intelligence: Intelligence(5),
            },
            ClassType::Rogue => Self {
                strength: Strength(8),
                dexterity: Dexterity(15),
                intelligence: Intelligence(7),
            },
            ClassType::Archer => Self {
                strength: Strength(10),
                dexterity: Dexterity(17),
                intelligence: Intelligence(6),
            },
        }
    }
}
