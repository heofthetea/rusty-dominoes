use crate::types::domino::{Domino, Orientation};

pub fn solve(dominoes: &[Domino]) -> Vec<Domino> {
    return Vec::from(dominoes);
}

/// Recursively build a chain of length `target` using a backtracking approach.
/// Note: only builds chains by appending on the right.
///
/// Returns: the chain built
fn _solve(chain: &[Domino], remaining: &[Domino], target: usize) -> Vec<Domino> {
    if chain.len() >= target {
        return Vec::from(chain);
    }
    for _domino in remaining {
        // TODO: implement backtracking logic
    }

    unimplemented!()
}

/*
Checks whether `right` can be appended to the right of `left`.
 */
fn matches(left: Domino, right: Domino) -> Option<Orientation> {
    if left.right == right.left {
        return Option::Some(Orientation::LEFT);
    }
    if left.right == right.right {
        return Option::Some(Orientation::RIGHT);
    }
    Option::None
}
