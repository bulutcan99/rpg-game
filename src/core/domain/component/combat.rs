use bevy::prelude::Component;

#[derive(Component)]
pub struct AttackPower(pub f32);

#[derive(Component)]
pub struct MagicPower(pub f32);

#[derive(Component)]
pub struct Defense(pub f32);

#[derive(Component)]
pub struct MagicDefense(pub f32);

#[derive(Component)]
pub struct CriticalChance(pub f32);

#[derive(Component)]
pub struct Evasion(pub f32);
