use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// When a health-related operation exceeds boundaries (e.g., setting health to an invalid value)
    InvalidHealth(f32),

    /// When a weapon is already equipped but another is being set
    WeaponAlreadyEquipped,

    /// When attempting an invalid operation such as setting an unrealistic max health or stamina
    InvalidStat,

    /// When the target is out of range for a strike
    TargetOutOfRange,

    /// When trying to perform an action on a dead character
    CharacterDead,

    /// When the requested skill is not available
    InvalidSkill(String),

    /// Generic error, for other cases not specifically covered above
    Other(String),

    /// Wrap another error type
    Wrapper(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for Error {}
