use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Copy, Clone)]
pub struct Domino {
    pub left: i8,
    pub right: i8,
    pub orientation: Orientation,
}

impl Domino {
    pub fn new(left: i8, right: i8) -> Domino {
        Domino {
            left,
            right,
            orientation: Orientation::LEFT,
        }
    }
    // changes only the orientation indicator. left and right should stay final
    pub fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::LEFT => Orientation::RIGHT,
            Orientation::RIGHT => Orientation::LEFT,
        }
    }

    /// Checks whether another domino can be appended to this one
    /// Returns:
    ///     Orientation::LEFT if `other` matches with its left
    ///     Orientation::RIGHT if `other` matches with its right
    ///     Option::None if there is no match
    /// Note:
    ///     Comparison always happens with the "actual right", accounting for orientation of `self`.
    pub fn matches(&self, other: Domino) -> Option<Orientation> {
        let actual_right = match self.orientation {
            Orientation::LEFT => self.right,
            Orientation::RIGHT => self.left,
        };
        if actual_right == other.left {
            return Option::Some(Orientation::LEFT);
        }
        if actual_right == other.right {
            return Option::Some(Orientation::RIGHT);
        }
        return Option::None;
    }

    // -----------------------------------------------------------------------------------
    fn repr(&self, f: &mut Formatter<'_>) -> Result {
        match self.orientation {
            Orientation::LEFT => write!(f, "({} | {})", self.left, self.right),
            Orientation::RIGHT => write!(f, "({} | {})", self.right, self.left),
        }
    }
}

impl Debug for Domino {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.repr(f)
    }
}

impl Display for Domino {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.repr(f)
    }
}

// ---------------------------------------------------------------------------------------

/*
LEFT = domino.left is actually left
RIGHT = domino.right is the left side (i.e. the domino is flipped)
*/
//TODO encode this information into a domino itself
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Orientation {
    LEFT,
    RIGHT,
}
