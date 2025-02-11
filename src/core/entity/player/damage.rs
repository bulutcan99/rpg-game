#[derive(Debug,Clone)]
pub enum DamageOutput {
    NoDamage,
    Damage(f32),
    Kill,
}
