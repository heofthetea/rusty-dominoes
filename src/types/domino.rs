pub struct Domino {
    pub left: i8,
    pub right: i8,
}

impl Domino {
    pub fn new(left: i8, right: i8) -> Domino {
        Domino { left, right }
    }
}
