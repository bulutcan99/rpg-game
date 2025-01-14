use crate::core::game_objects::AttackObject;
use std::io;

pub trait GameObjectVisitor<S> {
    fn visit(&self, to: &mut S);
}

pub trait GameObject<S>
where
    S: AttackObject,
{
    fn accept(&self, visitor: &dyn GameObjectVisitor<S>) -> io::Result<()>;
}
