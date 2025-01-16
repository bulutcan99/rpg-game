use std::io;

use super::weapon::{self, Weapon};

pub trait GameObjectVisitor<S> {
    fn visit(&self, to: &mut S);
}

pub trait GameObject<S>
where
    S: for<'a> Weapon<'a>,
{
    fn accept(&self, visitor: &dyn GameObjectVisitor<S>) -> io::Result<()>;
}
