use bevy::prelude::Component;

#[derive(Component)]
pub struct Health {
    pub max: u32,
    pub current: u32,
}

#[derive(Component)]
pub struct Mana {
    pub max: u32,
    pub current: u32,
}

#[derive(Component)]
pub struct Stamina {
    pub max: u32,
    pub current: u32,
}

#[derive(Component)]
pub struct Level(pub u8);

//TODO: EXP requirements for all levels will be kept in somewhere!
#[derive(Component)]
pub struct Experience(pub u32);
