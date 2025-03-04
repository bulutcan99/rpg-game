use bevy::prelude::Component;

#[derive(Debug, Clone)]
pub enum SkillType {
    Physical,
    Magic,
    Heal,
    Buff,
    Debuff,
}

#[derive(Debug, Clone, Component)]
pub struct Skill {
    pub name: String,
    pub skill_type: SkillType,
    pub damage: f32,
    pub cost: f32,
    pub cooldown: u32,
    pub range: u8,
}
