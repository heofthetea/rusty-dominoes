use crate::types::domino::{Domino, Orientation};

pub fn solve(dominoes: &[Domino]) -> Vec<Domino> {
    _solve(&Vec::new(), dominoes)
}

/// Recursively build a chain of length `target` using a backtracking approach.
/// Note: only builds chains by appending on the right.
///
/// Returns: the longest chain built
fn _solve(chain: &[Domino], remaining: &[Domino]) -> Vec<Domino> {
    let mut longest: Vec<Domino> = Vec::from(chain);
    for (i, domino) in remaining.iter().enumerate() {
        if chain.is_empty() {
            let mut new_remaining = Vec::from(remaining);
            new_remaining.remove(i);

            let res_1 = _solve(&[domino.clone()], &new_remaining);
            let res_2 = _solve(&[domino.clone_rotated()], &new_remaining);

            let longer = if res_1.len() > res_2.len() {res_1} else {res_2};
            if longer.len() > longest.len() {
                longest = longer;
            }
            continue;
        }


        let orientation = chain.last().unwrap().matches(domino);

        if orientation.is_none() {
            continue;
        }
        let orientation = orientation.unwrap();

        // append domino to chain in correct orientation
        let mut new_chain = Vec::from(chain);
        new_chain.push(if orientation == domino.orientation {
            domino.clone()
        } else {
            domino.clone_rotated()
        });

        // remove domino from remaining
        let mut new_remaining = Vec::from(remaining);
        new_remaining.remove(i);

        // recursive call
        let res = _solve(&new_chain, &new_remaining);
        if res.len() > longest.len() {
            longest = res;
        }
    }
    return longest;
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
