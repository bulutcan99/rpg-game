use bevy::prelude::Component;

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

#[derive(Component)]
pub struct Stat {
    pub level: Level,
    pub experience: Experience,
    pub health: Health,
    pub mana: Mana,
    pub stamina: Stamina,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            level: Level(1),
            experience: Experience(0),
            health: Health(100.0),
            mana: Mana(50.0),
            stamina: Stamina(75.0),
        }
    }
}
