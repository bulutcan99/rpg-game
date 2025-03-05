use bevy::prelude::{Bundle, Component};

use crate::core::domain::entity::class::ClassType;

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct Experience(pub u32);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Mana(pub f32);

#[derive(Component)]
pub struct Stamina(pub f32);

#[derive(Bundle)]
pub struct ClassStatBundle {
    pub level: Level,
    pub experience: Experience,
    pub health: Health,
    pub mana: Mana,
    pub stamina: Stamina,
}

impl ClassStatBundle {
    pub fn new(class: ClassType) -> Self {
        match class {
            ClassType::Warrior => Self {
                level: Level(1),
                experience: Experience(0),
                health: Health(150.0),
                mana: Mana(30.0),
                stamina: Stamina(100.0),
            },
            ClassType::Rogue => Self {
                level: Level(1),
                experience: Experience(0),
                health: Health(100.0),
                mana: Mana(50.0),
                stamina: Stamina(120.0),
            },
            ClassType::Archer => Self {
                level: Level(1),
                experience: Experience(0),
                health: Health(90.0),
                mana: Mana(80.0),
                stamina: Stamina(90.0),
            },
        }
    }
}
