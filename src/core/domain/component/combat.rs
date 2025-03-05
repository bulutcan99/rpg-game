use crate::core::domain::entity::class::ClassType;
use bevy::prelude::{Bundle, Component, Query};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    Melee,
    Ranged,
}

#[derive(Component)]
pub struct AttackPower {
    pub power: f32,
    pub attack_type: AttackType,
}

#[derive(Component)]
pub struct Defense(pub f32);

#[derive(Component)]
pub struct CriticalChance(pub f32);

#[derive(Component)]
pub struct Evasion(pub f32);

#[derive(Bundle)]
pub struct ClassCombatBundle {
    pub attack_power: AttackPower,
    pub defense: Defense,
    pub critical_chance: CriticalChance,
    pub evasion: Evasion,
}

impl ClassCombatBundle {
    pub fn new(class: ClassType) -> Self {
        match class {
            ClassType::Warrior => Self {
                attack_power: AttackPower {
                    power: 20.0,
                    attack_type: AttackType::Melee,
                },
                defense: Defense(15.0),
                critical_chance: CriticalChance(0.07),
                evasion: Evasion(0.03),
            },
            ClassType::Archer => Self {
                attack_power: AttackPower {
                    power: 18.0,
                    attack_type: AttackType::Ranged,
                },
                defense: Defense(8.0),
                critical_chance: CriticalChance(0.12),
                evasion: Evasion(0.06),
            },
            ClassType::Rogue => Self {
                attack_power: AttackPower {
                    power: 12.0,
                    attack_type: AttackType::Melee,
                },
                defense: Defense(10.0),
                critical_chance: CriticalChance(0.15),
                evasion: Evasion(0.08),
            },
        }
    }
}
