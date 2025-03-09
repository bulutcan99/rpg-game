use bevy::prelude::Component;

#[derive(Component)]
pub struct AttackPower(pub f32);

#[derive(Component)]
pub struct Defense(pub f32);

#[derive(Component)]
pub struct CriticalChance(pub f32);

#[derive(Component)]
pub struct Evasion(pub f32);

#[derive(Component)]
pub struct Combat {
    pub attack_power: AttackPower,
    pub defense: Defense,
    pub critical_chance: CriticalChance,
    pub evasion: Evasion,
}

impl Combat {
    pub fn new() -> Self {
        Self {
            attack_power: AttackPower(50.0),
            defense: Defense(10.0),
            critical_chance: CriticalChance(5.0),
            evasion: Evasion(3.0),
        }
    }
}
