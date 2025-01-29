use std::io;

use crate::core::entity::weapon::weapon::Weapon;

pub trait GameObjectVisitor<S> {
    fn visit(&self, to: &mut S);
}

pub trait GameObject<S>
where
    S: Weapon,
{
    fn accept(&self, visitor: &dyn GameObjectVisitor<S>) -> io::Result<()>;
}
