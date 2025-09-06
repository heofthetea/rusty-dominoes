use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Copy, Clone)]
pub struct Domino {
    pub left: i8,
    pub right: i8,
}

impl Domino {
    pub fn new(left: i8, right: i8) -> Domino {
        Domino { left, right }
    }
}

impl Debug for Domino {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} | {})", self.left, self.right)
    }
}

impl Display for Domino {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} | {})", self.left, self.right)
    }
}
